use crate::{LOG_TARGET, LogClass, LogLevel, LogRecord};
use log::{debug, info, warn};
use pass_it_on::notifications::{ClientReadyMessage, Message};
use std::collections::HashSet;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::string::String;
use std::time::{Duration, SystemTime};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, BufReader};
use tokio::sync::mpsc;

struct LogFile {
    file: File,
    reader: BufReader<File>,
}

impl LogFile {
    pub async fn from<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path).await?;
        let file_clone = file.try_clone().await?;
        let reader = BufReader::new(file_clone);
        Ok(LogFile { file, reader })
    }

    pub async fn modified_time(&self) -> io::Result<SystemTime> {
        self.file.metadata().await?.modified()
    }

    #[allow(dead_code)]
    pub async fn length(&self) -> io::Result<u64> {
        Ok(self.file.metadata().await?.len())
    }

    pub async fn inode(&self) -> io::Result<u64> {
        Ok(self.file.metadata().await?.ino())
    }

    #[allow(dead_code)]
    pub async fn position(&mut self) -> io::Result<u64> {
        self.reader.stream_position().await
    }

    #[allow(dead_code)]
    pub async fn reset_position(&mut self) -> io::Result<u64> {
        self.reader.rewind().await
    }

    pub async fn read_log(&mut self) -> Option<Vec<String>> {
        let mut buf: Vec<u8> = Vec::new();
        let bytes_read = self.reader.read_to_end(&mut buf).await;

        if bytes_read.is_err() {
            warn!("{}", bytes_read.unwrap_err());
            None
        } else {
            let buf_string = match std::str::from_utf8(&buf) {
                Err(e) => {
                    warn!("{}", e);
                    String::new()
                }
                Ok(s) => String::from(s),
            };
            Some(buf_string.lines().map(String::from).collect())
        }
    }
}

pub async fn monitor_log<P: AsRef<Path>>(
    path: P,
    frequency: Duration,
    level_filter: HashSet<LogLevel>,
    class_filter: HashSet<LogClass>,
    notification_name: &str,
    interface: mpsc::Sender<ClientReadyMessage>,
) {
    let mut logfile = LogFile::from(path.as_ref()).await.unwrap();
    let mut previous_mod_time = SystemTime::UNIX_EPOCH;
    info!(target: LOG_TARGET, "Monitoring log -> {}", path.as_ref().to_string_lossy());

    loop {
        let this_mod_time = logfile.modified_time().await.unwrap();
        let records = {
            if log_has_rotated(&path, &logfile).await {
                debug!(target: LOG_TARGET, "Log rotation detected");
                logfile = LogFile::from(path.as_ref()).await.unwrap();
                previous_mod_time = this_mod_time;
                parse_log_records(logfile.read_log().await)
            } else {
                match previous_mod_time != this_mod_time {
                    false => None,
                    true => {
                        previous_mod_time = this_mod_time;
                        parse_log_records(logfile.read_log().await)
                    }
                }
            }
        };

        if let Some(recs) = records {
            let messages = filter_messages(notification_name, recs, &level_filter, &class_filter);
            for message in messages {
                if let Err(error) = interface.send(message).await {
                    warn!(target: LOG_TARGET, "Error sending notification: {}", error)
                }
            }
        }
        tokio::time::sleep(frequency).await;
    }
}

fn parse_log_records(logs: Option<Vec<String>>) -> Option<Vec<LogRecord>> {
    match logs {
        None => None,
        Some(logs) => {
            let records: Vec<LogRecord> = logs
                .iter()
                .filter_map(|log| LogRecord::from_record(log))
                .collect();
            Some(records)
        }
    }
}

fn filter_messages(
    notification_name: &str,
    records: Vec<LogRecord>,
    level_filter: &HashSet<LogLevel>,
    class_filter: &HashSet<LogClass>,
) -> Vec<ClientReadyMessage> {
    let filtered: Vec<_> = records
        .iter()
        .filter(|r| level_filter.contains(&r.level) || class_filter.contains(&r.class))
        .collect();
    let to_send: Vec<_> = filtered
        .iter()
        .map(|r| Message::new(r.status_message.as_str()).to_client_ready_message(notification_name))
        .collect();
    to_send
}

async fn get_inode<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    Ok(File::open(path.as_ref()).await?.metadata().await?.ino())
}

//#[cfg(target_family = "unix")]
async fn log_has_rotated<P: AsRef<Path>>(path: P, original_file: &LogFile) -> bool {
    original_file.inode().await.unwrap_or(0) != get_inode(path).await.unwrap_or(0)
}

// TODO: Add function to check for log rotation by identifying the most recent log that was backed up
/*#[cfg(target_family = "windows")]
fn log_has_rotated_win<P: AsRef<Path>>(path: P, original_file: &LogFile) -> bool {
    let original_file.file.
    let new_file = File::open(path);
}*/

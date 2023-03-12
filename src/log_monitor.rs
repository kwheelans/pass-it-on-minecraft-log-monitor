use std::collections::HashSet;
use std::fs::{File};
use std::io;
use std::io::{BufReader, Read, Seek};
use std::os::unix::fs::{MetadataExt};
use std::path::{Path};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::string::String;
use crate::{create_message, LogClass, LogLevel, LogRecord, send_messages};


struct LogFile {
    file: File,
    reader: BufReader<File>
}

impl LogFile {
    pub fn from <P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let file_clone = file.try_clone()?;
        let reader = BufReader::new(file_clone);
        Ok(LogFile {file, reader})
        }

    pub fn modified_time(&self) -> io::Result<SystemTime> {
        self.file.metadata()?.modified()
    }

    #[allow(dead_code)]
    pub fn length(&self) -> io::Result<u64> {
        Ok(self.file.metadata()?.len())
    }

    pub fn inode(&self) -> io::Result<u64> {
        Ok(self.file.metadata()?.ino())
    }

    #[allow(dead_code)]
    pub fn position(&mut self) -> io::Result<u64> {
        self.reader.stream_position()
    }

    #[allow(dead_code)]
    pub fn reset_position(&mut self) -> io::Result<()> {
        self.reader.rewind()
    }

    pub fn read_log(&mut self) -> Option<Vec<String>> {
        let mut buf:Vec<u8> = Vec::new();
        let bytes_read = self.reader.read_to_end(&mut buf);

        if bytes_read.is_err() {
            eprint!("{}", bytes_read.unwrap_err());
            None

        } else {
            let buf_string = match std::str::from_utf8(&buf) {
                Err(e) => {
                    eprint!("{}", e);
                    String::new()
                },
                Ok(s) => String::from(s),
            };
            Some(buf_string.lines().map(String::from).collect())
        }
    }
}

pub fn monitor_log<P: AsRef<Path>>(path: P, frequency: Duration, url: &str, botname: &str, level_filter:HashSet<LogLevel>, class_filter: HashSet<LogClass>) {
    let mut logfile = LogFile::from(path.as_ref()).unwrap();
    let mut previous_mod_time = SystemTime::UNIX_EPOCH;

    loop {
        let this_mod_time = logfile.modified_time().unwrap();
        let records = {
            if log_has_rotated(&path, logfile.inode().unwrap_or(0)) {
                logfile = LogFile::from(path.as_ref()).unwrap();
                parse_log_records(logfile.read_log())
            } else {
                match previous_mod_time != this_mod_time {
                    false => None,
                    true => {
                        previous_mod_time = this_mod_time;
                        parse_log_records(logfile.read_log())
                    }
                }
            }
        };

        if let Some(recs) = records {
            send_discord_alert(recs, url, botname, &level_filter, &class_filter)
        }
        sleep(frequency);
    }
}

fn parse_log_records(logs: Option<Vec<String>>) -> Option<Vec<LogRecord>> {
    match logs {
        None => None,
        Some(logs) => {
            let records:Vec<LogRecord> = logs.iter().filter_map(|log| LogRecord::from_record(log)).collect();
            Some(records)
        }
    }
}

fn send_discord_alert(records: Vec<LogRecord>, url: &str, botname: &str, level_filter: &HashSet<LogLevel>, class_filter: &HashSet<LogClass>) {
    let filtered:Vec<_> = records.iter().filter(|r| level_filter.contains(&r.level) || class_filter.contains(&r.class)).collect();
    let to_send:Vec<_> = filtered.iter().map(|r| create_message(botname, &r.message)).collect();

    send_messages(url, to_send);
}

// TODO: getting ino to check if the current latest.log is the same are the one already opened
// TODO: Add function to check for log rotation by identifying the most recent log that was backed up
fn get_inode<P: AsRef<Path>>(path: P) -> io::Result<u64> {
   Ok(File::open(path.as_ref())?.metadata()?.ino())
}

fn log_has_rotated<P: AsRef<Path>>(path: P, original_inode: u64) -> bool {
    original_inode != get_inode(path).unwrap_or(0)
}




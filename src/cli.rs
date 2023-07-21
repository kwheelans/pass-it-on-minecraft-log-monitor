use crate::LogClass::{ServerStart, ServerStop, ServerVersion};
use crate::LogLevel::Error;
use crate::{LogClass, LogLevel};
use clap::Parser;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to minecraft server directory
    #[clap(short, long, value_parser)]
    directory: PathBuf,

    /// Path to pass-it-on client configuration file
    #[clap(short, long, value_parser)]
    client_config: PathBuf,

    /// How often to check the log file in seconds
    #[clap(short, long, value_parser, default_value_t = 5)]
    frequency: u64,

    /// Notification name for pass-it-on client to use
    #[clap(short, long, value_parser, default_value = "mc-log")]
    notification_name: String,

    /// Specify log levels to always be included [default: error]
    #[clap(long, value_enum)]
    include_level: Vec<LogLevel>,

    /// Specify log classes to always be included [default: ServerVersion, ServerStart, ServerStop]
    #[clap(long, value_enum)]
    include_class: Vec<LogClass>,
}

impl CliArgs {
    pub fn directory(&self) -> &Path {
        self.directory.as_path()
    }

    pub fn client_config(&self) -> &Path {
        self.client_config.as_path()
    }

    pub fn frequency(&self) -> Duration {
        Duration::from_secs(self.frequency)
    }

    pub fn notification_name(&self) -> &str {
        &self.notification_name
    }

    pub fn include_level(&self) -> HashSet<LogLevel> {
        let include_level = self.include_level.to_owned();
        match include_level.is_empty() {
            true => HashSet::from([Error]),
            false => include_level.into_iter().collect(),
        }
    }

    pub fn include_class(&self) -> HashSet<LogClass> {
        let include_class = self.include_class.to_owned();
        match include_class.is_empty() {
            true => HashSet::from([ServerStart, ServerVersion, ServerStop]),
            false => include_class.into_iter().collect(),
        }
    }
}

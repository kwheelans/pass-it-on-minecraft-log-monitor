use crate::{LogClass, LogLevel};
use clap::Parser;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;
use log::LevelFilter;

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

    /// Specify log levels to always be included
    #[clap(long, value_enum, default_value = "error")]
    include_level: Vec<LogLevel>,

    /// Specify log classes to always be included
    #[clap(long, value_enum, default_values_t = [LogClass::ServerVersion, LogClass::ServerStart, LogClass::ServerStop])]
    include_class: Vec<LogClass>,

    /// Set the logging level
    #[clap(long, value_enum, default_value = "info")]
    log_level: LevelFilter,
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
        self.include_level.iter().copied().collect()
    }

    pub fn include_class(&self) -> HashSet<LogClass> {
        self.include_class.iter().copied().collect()
    }

    pub fn log_level(&self) -> LevelFilter {
        self.log_level
    }
}

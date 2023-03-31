use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use clap::Parser;
use crate::{LogClass, LogLevel};
use crate::LogClass::{ServerOverload, ServerStart, ServerStop, ServerVersion};
use crate::LogLevel::{Error};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to minecraft server directory
    #[clap(short, long, value_parser)]
    pub directory: PathBuf,

    ///How often to check the log file in seconds
    #[clap(short, long, value_parser, default_value_t = 5)]
    frequency: u64,

    /// Discord webhook URL
    #[clap(short, long, value_parser)]
    pub webhook_url: String,

    /// Discord bot username
    #[clap(short, long, value_parser,  default_value_t = String::from("Server-Status"))]
    pub bot_name: String,

    /// Specify log levels to always be included [default: error]
    #[clap(short = 'l', long, value_enum)]
    include_level: Vec<LogLevel>,

    /// Specify log classes to always be included [default: ServerVersion, ServerStart, ServerStop]
    #[clap(short = 'c', long, value_enum)]
    include_class: Vec<LogClass>,
}

impl CliArgs {
    pub fn include_level(&self) -> HashSet<LogLevel> {
        let include_level = self.include_level.to_owned();
        match include_level.is_empty() {
            true => HashSet::from([Error]),
            false => include_level.into_iter().collect()
        }
    }

    pub fn include_class(&self) -> HashSet<LogClass> {
        let include_class = self.include_class.to_owned();
        match include_class.is_empty() {
            true => HashSet::from([ServerStart, ServerVersion, ServerStop]),
            false => include_class.into_iter().collect(),
        }
    }

    pub fn frequency (&self) -> Duration {
        Duration::from_secs(self.frequency)
    }
}

pub fn get() -> CliArgs {
    CliArgs::parse()
}
use std::path::{Path, PathBuf};

use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to minecraft-log-monitor configuration file.
    #[arg(short, long, value_parser)]
    monitor_config: PathBuf,

    /// Set the logging level
    #[arg(long, value_enum, default_value = "info")]
    log_level: LevelFilter,

    /// Set delay in seconds to wait before stating to monitor log path
    #[arg(short, long, value_parser)]
    delay_start: Option<u64>,
}

impl CliArgs {
    pub fn log_level(&self) -> LevelFilter {
        self.log_level
    }

    pub fn monitor_config(&self) -> Option<&Path> {
        Some(self.monitor_config.as_path())
    }

    pub fn delay(&self) -> Option<u64> {
        self.delay_start
    }
}

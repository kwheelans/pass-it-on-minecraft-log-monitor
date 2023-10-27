use clap::Parser;
use log::LevelFilter;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to pio-minecraft-server-monitor configuration file.
    #[arg(short, long, value_parser)]
    monitor_config: PathBuf,

    /// Set the logging level
    #[arg(long, value_enum, default_value = "info")]
    log_level: LevelFilter,
}

impl CliArgs {
    pub fn log_level(&self) -> LevelFilter {
        self.log_level
    }

    pub fn monitor_config(&self) -> Option<&Path> {
        Some(self.monitor_config.as_path())
    }
}

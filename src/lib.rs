pub mod cli;
pub mod configuration;
pub mod log_monitor;
pub mod log_record;

pub use {cli::CliArgs, log_monitor::monitor_log, log_record::*};

pub const LOG_TARGET: &str = "minecraft-log-monitor";

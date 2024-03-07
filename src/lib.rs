//! # Minecraft Log Monitor
//! A simple Minecraft server status monitor which uses the pass-it-on crate to send the selected
//! logs to a pass-it-on server instance.
//!
//! ```toml
//! [client]
//! key = "UVXu7wtbXHWNgAr6rWyPnaZbZK9aYin8"
//!
//! [[client.interface]]
//! type = "http"
//! host = "localhost"
//! port = 8080
//!
//! [monitor]
//! log_path = "logs/latest.log"
//!
//! [[monitor.notification]]
//! name = "mc_test1"
//!
//! [[monitor.notification]]
//! name = "mc_test2"
//! include_class = ["UserJoinedDetails"]
//! ```

mod cli;
pub mod configuration;
pub mod log_record;
mod monitor;

pub use {cli::CliArgs, monitor::monitor_log};

pub const LOG_TARGET: &str = "minecraft-log-monitor";

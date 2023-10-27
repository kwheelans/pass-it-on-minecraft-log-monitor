use crate::{LogClass, LogLevel};
use pass_it_on::ClientConfigFile;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Deserialize)]
pub struct MonitorConfigFileParser {
    pub monitor: MonitorConfiguration,
    pub client: ClientConfigFile,
}

#[derive(Deserialize)]
pub struct MonitorConfiguration {
    log_path: PathBuf,
    #[serde(default = "default_freq")]
    frequency: u64,
    notification: Vec<Notification>,
}

#[derive(Deserialize, Debug)]
pub struct Notification {
    name: String,
    #[serde(default = "default_levels")]
    include_level: HashSet<LogLevel>,
    #[serde(default = "default_classes")]
    include_class: HashSet<LogClass>,
}

impl TryFrom<&str> for MonitorConfigFileParser {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}

fn default_freq() -> u64 {
    5
}

fn default_levels() -> HashSet<LogLevel> {
    [LogLevel::Error].iter().copied().collect()
}

fn default_classes() -> HashSet<LogClass> {
    [
        LogClass::ServerVersion,
        LogClass::ServerStart,
        LogClass::ServerStop,
    ]
    .iter()
    .copied()
    .collect()
}

impl MonitorConfiguration {
    pub fn new(log_path: PathBuf, frequency: u64, notification: Vec<Notification>) -> Self {
        Self {
            log_path,
            frequency,
            notification,
        }
    }

    pub fn log_path(&self) -> &Path {
        self.log_path.as_path()
    }
    pub fn frequency(&self) -> Duration {
        Duration::from_secs(self.frequency)
    }

    pub fn notification(&self) -> &[Notification] {
        &self.notification
    }
}

impl Notification {
    pub fn new<S: AsRef<str>>(
        name: S,
        include_level: HashSet<LogLevel>,
        include_class: HashSet<LogClass>,
    ) -> Self {
        Self {
            name: name.as_ref().to_string(),
            include_level,
            include_class,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn include_level(&self) -> HashSet<LogLevel> {
        self.include_level.iter().copied().collect()
    }
    pub fn include_class(&self) -> HashSet<LogClass> {
        self.include_class.iter().copied().collect()
    }
}

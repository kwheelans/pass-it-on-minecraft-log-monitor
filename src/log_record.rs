use crate::LOG_TARGET;
use clap::ValueEnum;
use log::trace;
use serde::Deserialize;
use std::ops::Add;

const SQUARE_BRACKETS: [char; 2] = ['[', ']'];
const ROUND_BRACKETS: [char; 2] = ['(', ')'];
const COLON: char = ':';
const BACKSLASH: char = '/';
const STATUS_DELIM: &str = " - ";

#[derive(Debug, PartialEq, Eq, Hash, ValueEnum, Deserialize, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Other,
}

#[derive(Debug, PartialEq, Eq, Hash, ValueEnum, Deserialize, Clone, Copy)]
pub enum LogClass {
    UserAuth,
    UserJoinedDetails,
    UserJoined,
    UserLeft,
    ServerVersion,
    ServerStart,
    ServerStop,
    ServerOverload,
    Other,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LogRecord {
    pub time: String,
    pub level: LogLevel,
    pub class: LogClass,
    pub status_message: String,
}

impl LogRecord {
    pub fn new(time: String, level: LogLevel, class: LogClass, status_message: String) -> Self {
        Self {
            time,
            level,
            class,
            status_message,
        }
    }

    pub fn from_record(record: &str) -> Option<LogRecord> {
        trace!(target: LOG_TARGET, "Processing log: {}", record);
        if !record.starts_with(SQUARE_BRACKETS[0]) {
            None
        } else {
            let time = record[0..10]
                .trim()
                .trim_matches(SQUARE_BRACKETS.as_ref())
                .to_string();
            let split = record[10..].split_once(COLON);

            if split.is_some() {
                let split = split?;
                let level = parse_log_level(split.0);
                let log_message = split.1.trim();
                let class = parse_class(log_message);
                let status_message = parse_status_message(time.as_str(), level, class, log_message);

                Some(LogRecord::new(time, level, class, status_message))
            } else {
                None
            }
        }
    }
}

fn parse_log_level(level_string: &str) -> LogLevel {
    let level = level_string
        .trim_matches(SQUARE_BRACKETS.as_ref())
        .split_once(BACKSLASH)
        .unwrap();
    match level.1 {
        "INFO" => LogLevel::Info,
        "WARN" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        _ => LogLevel::Other,
    }
}

fn parse_class(log_message: &str) -> LogClass {
    let msg = log_message.trim();
    match msg {
        m if m.contains("UUID") => LogClass::UserAuth,
        m if m.contains("logged in with entity id") => LogClass::UserJoinedDetails,
        m if m.ends_with("joined the game") => LogClass::UserJoined,
        m if m.ends_with("left the game") => LogClass::UserLeft,
        m if m.starts_with("Starting minecraft server version") => LogClass::ServerVersion,
        m if m.starts_with("Can't keep up!") => LogClass::ServerOverload,
        m if m.starts_with("Done (") => LogClass::ServerStart,
        m if m.starts_with("Stopping server") => LogClass::ServerStop,
        _ => LogClass::Other,
    }
}

fn parse_status_message(time: &str, level: LogLevel, class: LogClass, log_message: &str) -> String {
    let status_msg = String::from(time).add(STATUS_DELIM);
    let status_msg = match level {
        LogLevel::Warning => status_msg.add("Warning").add(STATUS_DELIM),
        LogLevel::Error => status_msg.add("Error").add(STATUS_DELIM),
        _ => status_msg,
    };

    let status_msg = match class {
        LogClass::ServerVersion => status_msg
            .add("Server starting up using version: ")
            .add(log_message.split_whitespace().last().unwrap_or("Unknown")),
        LogClass::ServerStart => status_msg.add(parse_server_start_log(log_message).as_str()),
        LogClass::ServerStop => status_msg.add("Server is shutting down"),
        LogClass::ServerOverload => {
            status_msg.add(parse_server_overloaded_log(log_message).as_str())
        }
        _ => status_msg.add(log_message),
    };
    status_msg
}

fn parse_server_start_log(log_message: &str) -> String {
    let split_msg: Vec<_> = log_message.splitn(3, ROUND_BRACKETS).collect();
    trace!(target: LOG_TARGET, "Log message split into {} parts", split_msg.len());
    if split_msg.len() >= 3 {
        String::from("Server started after ").add(split_msg[1])
    } else {
        log_message.to_string()
    }
}

fn parse_server_overloaded_log(log_message: &str) -> String {
    let split_msg = log_message.split_once('?');
    match split_msg {
        Some(split) => String::from("Server running slow.").add(split.1),
        _ => log_message.to_string(),
    }
}

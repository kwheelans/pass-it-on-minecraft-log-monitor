use clap::ValueEnum;
const SQUARE_BRACKETS: [char; 2] = ['[', ']'];
const COLON: char = ':';
const BACKSLASH: char = '/';

#[derive(Debug, PartialEq, Eq, Hash, ValueEnum, Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Other,
}

#[derive(Debug, PartialEq, Eq, Hash, ValueEnum, Clone)]
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
    pub message: String,
}

impl LogRecord {
    pub fn new(time: String, level: LogLevel, class: LogClass, message: String) -> Self {
        Self { time, level, class, message }
    }

    pub fn from_record(record: &str) -> Option<LogRecord>{
        if !record.starts_with(SQUARE_BRACKETS[0]) {
            None
        } else {
            let time = record[0..10].trim().trim_matches(SQUARE_BRACKETS.as_ref());
            let split = record[10..].split_once(COLON);

            if split.is_some() {
                let split = split?;
                let level = parse_log_level(split.0);
                let message = split.1.trim();
                let class = parse_class(message);

                Some(LogRecord::new(time.to_string(), level, class, message.to_string()))
            } else {
                None
            }
        }
    }
}

fn parse_log_level(level_string: &str) -> LogLevel {
    let level = level_string.trim_matches(SQUARE_BRACKETS.as_ref()).split_once(BACKSLASH).unwrap();
    match level.1 {
        "INFO" => LogLevel::Info,
        "WARN" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        _ => LogLevel::Other,
    }
}

fn parse_class(logmsg: &str) -> LogClass {
    let msg = logmsg.trim();
    match msg {
        m if m.contains("UUID") => LogClass::UserAuth,
        m if m.contains("logged in with entity id") => LogClass::UserJoinedDetails,
        m if m.ends_with("joined the game") => LogClass::UserJoined,
        m if m.ends_with("left the game") => LogClass::UserLeft,
        m if m.starts_with("Starting minecraft server version") => LogClass::ServerVersion,
        m if m.starts_with("Can't keep up!") => LogClass::ServerOverload,
        m if m.starts_with("Done (") => LogClass::ServerStart,
        m if m.starts_with("Stopping server") => LogClass::ServerStop,
        _ => LogClass::Other
    }
}

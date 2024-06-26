use pass_it_on_minecraft_log_monitor::log_record::{LogClass, LogLevel, LogRecord};

const SERVER_START: &str =
    "[15:44:37] [Server thread/INFO]: Done (4.736s)! For help, type \"help\"";
const SERVER_VERSION: &str =
    "[15:44:32] [Server thread/INFO]: Starting minecraft server version 1.19.3";
const SERVER_OVERLOAD: &str = "[19:33:39] [Server thread/WARN]: Can't keep up! Is the server overloaded? Running 2033ms or 40 ticks behind";
const SERVER_STOP: &str = "[17:47:21] [Server thread/INFO]: Stopping server";

const USER_AUTH: &str = "[01:41:59] [User Authenticator #1/INFO]: UUID of player Player is 9zz9999z-9zzz-99zz-z9zz-9z9z9zzzz99z";
const USER_JOIN: &str = "[15:51:33] [Server thread/INFO]: Player joined the game";
const USER_JOIN_DETAILS: &str = "[01:42:00] [Server thread/INFO]: Player[/127.0.0.1:60084] logged in with entity id 1769 at (-147.54709899840574, 64.0, -189.49623526120888)";
const USER_LEFT: &str = "[15:52:05] [Server thread/INFO]: Player left the game";

#[test]
fn parse_log_server_start() {
    let expected = LogRecord::new(
        "15:44:37".to_string(),
        LogLevel::Info,
        LogClass::ServerStart,
        "15:44:37 - Server started after 4.736s".to_string(),
    );
    let log = LogRecord::from_record(SERVER_START).unwrap();
    assert_eq!(log, expected);
}

#[test]
fn parse_log_server_version() {
    let expected = LogRecord::new(
        "15:44:32".to_string(),
        LogLevel::Info,
        LogClass::ServerVersion,
        "15:44:32 - Server starting up using version: 1.19.3".to_string(),
    );
    let log = LogRecord::from_record(SERVER_VERSION).unwrap();
    assert_eq!(log, expected);
}

#[test]
fn parse_log_server_overload() {
    let expected = LogRecord::new(
        "19:33:39".to_string(),
        LogLevel::Warning,
        LogClass::ServerOverload,
        "19:33:39 - Warning - Server running slow. Running 2033ms or 40 ticks behind".to_string(),
    );
    let log = LogRecord::from_record(SERVER_OVERLOAD).unwrap();
    assert_eq!(log, expected);
}

#[test]
fn parse_log_server_stop() {
    let expected = LogRecord::new(
        "17:47:21".to_string(),
        LogLevel::Info,
        LogClass::ServerStop,
        "17:47:21 - Server is shutting down".to_string(),
    );
    let log = LogRecord::from_record(SERVER_STOP).unwrap();
    assert_eq!(log, expected);
}

#[test]
fn parse_log_user_auth() {
    let expected = LogRecord::new(
        "01:41:59".to_string(),
        LogLevel::Info,
        LogClass::UserAuth,
        "01:41:59 - UUID of player Player is 9zz9999z-9zzz-99zz-z9zz-9z9z9zzzz99z".to_string(),
    );
    let log = LogRecord::from_record(USER_AUTH).unwrap();
    assert_eq!(log, expected);
}

#[test]
fn parse_log_user_join() {
    let expected = LogRecord::new(
        "15:51:33".to_string(),
        LogLevel::Info,
        LogClass::UserJoined,
        "15:51:33 - Player joined the game".to_string(),
    );
    let log = LogRecord::from_record(USER_JOIN).unwrap();
    assert_eq!(log, expected);
}

#[test]
fn parse_log_user_join_details() {
    let expected = LogRecord::new("01:42:00".to_string(), LogLevel::Info, LogClass::UserJoinedDetails, "01:42:00 - Player[/127.0.0.1:60084] logged in with entity id 1769 at (-147.54709899840574, 64.0, -189.49623526120888)".to_string());
    let log = LogRecord::from_record(USER_JOIN_DETAILS).unwrap();
    assert_eq!(log, expected);
}

#[test]
fn parse_log_user_left() {
    let expected = LogRecord::new(
        "15:52:05".to_string(),
        LogLevel::Info,
        LogClass::UserLeft,
        "15:52:05 - Player left the game".to_string(),
    );
    let log = LogRecord::from_record(USER_LEFT).unwrap();
    assert_eq!(log, expected);
}

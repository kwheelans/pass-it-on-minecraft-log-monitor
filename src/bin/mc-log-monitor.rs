use minecraft_server_monitor_discord_bot::{cli, monitor_log};


fn main() {
    let args = cli::get();
    let log_path = args.directory.join("logs/latest.log");

    monitor_log(log_path, args.frequency(), &*args.webhook_url, &*args.bot_name, args.include_level(), args.include_class())
}
use log::{error, LevelFilter};
use pass_it_on::{ClientConfiguration, Error, start_client};
use tokio::sync::mpsc;
use minecraft_server_monitor_discord_bot::{cli, LOG_TARGET, monitor_log};

const NOTIFICATION_NAME: &str = "mc-log";

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .env()
        .with_module_level(pass_it_on::LIB_LOG_TARGET, LevelFilter::Info)
        .with_module_level(LOG_TARGET, LevelFilter::Info)
        .with_colors(true)
        .init()
        .unwrap();


    if let Err(error) = run().await {
        error!(target: LOG_TARGET, "{}", error)
    }

}

async fn run() -> Result<(), Error>{
    let args = cli::get();
    let log_path = args.directory.join("logs/latest.log");
    let frequency = args.frequency();
    let level_filter = args.include_level();
    let class_filter = args.include_class();
    let client_config = ClientConfiguration::from_toml(std::fs::read_to_string(args.client_config)?.as_str())?;
    let (interface_tx, interface_rx) = mpsc::channel(100);

    tokio::spawn( async move {
        monitor_log(log_path, frequency, level_filter, class_filter, NOTIFICATION_NAME, interface_tx.clone()).await;
    });

    start_client(client_config, interface_rx, None).await?;

    Ok(())
}

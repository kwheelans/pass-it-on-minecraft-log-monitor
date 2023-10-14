use clap::Parser;
use log::{error, LevelFilter};
use pio_minecraft_server_monitor::{monitor_log, CliArgs, LOG_TARGET};
use pass_it_on::{start_client, ClientConfiguration, Error};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .env()
        .with_module_level(pass_it_on::LIB_LOG_TARGET, args.log_level())
        .with_module_level(LOG_TARGET, args.log_level())
        .with_colors(true)
        .init()
        .unwrap();

    if let Err(error) = run(args).await {
        error!(target: LOG_TARGET, "{}", error)
    }
}

async fn run(args: CliArgs) -> Result<(), Error> {
    let log_path = args.directory().join("logs/latest.log");
    let frequency = args.frequency();
    let level_filter = args.include_level();
    let class_filter = args.include_class();
    let client_config =
        ClientConfiguration::try_from(std::fs::read_to_string(args.client_config())?.as_str())?;
    let (interface_tx, interface_rx) = mpsc::channel(100);

    tokio::spawn(async move {
        monitor_log(
            log_path,
            frequency,
            level_filter,
            class_filter,
            args.notification_name(),
            interface_tx.clone(),
        )
        .await
    });

    start_client(client_config, interface_rx, None, None).await?;

    Ok(())
}

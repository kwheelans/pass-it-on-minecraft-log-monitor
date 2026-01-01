use clap::Parser;
use pass_it_on::{start_client, Error};
use pass_it_on_minecraft_log_monitor::configuration::MonitorConfigFileParser;
use pass_it_on_minecraft_log_monitor::{monitor_log, CliArgs, LOG_TARGET};
use std::io::ErrorKind;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CliArgs::parse();
    tracing_subscriber::fmt().with_max_level(args.log_level()).init();

    if let Err(error) = run(args).await {
        error!(target: LOG_TARGET, "{}", error)
    }
    Ok(())
}

async fn run(args: CliArgs) -> Result<(), Error> {
    let monitor_config;
    let client_config;

    info!(target: LOG_TARGET, "Minecraft log monitor starting");

    {
        let parsed_config = MonitorConfigFileParser::try_from(
            std::fs::read_to_string(args.monitor_config().unwrap())?.as_str(),
        )?;
        monitor_config = parsed_config.monitor;
        client_config = parsed_config.client.try_into()?;
    }
    if monitor_config.log_path().exists() {
        let (interface_tx, interface_rx) = mpsc::channel(100);

        if let Some(delay) = args.delay() {
            info!(target: LOG_TARGET, "Delaying log monitoring start for {} seconds", delay);
            tokio::time::sleep(Duration::from_secs(delay)).await;
        }

        tokio::spawn(async move { monitor_log(monitor_config, interface_tx.clone()).await });

        start_client(client_config, interface_rx, None, None).await?;
        Ok(())
    } else {
        error!(target: LOG_TARGET, "Specified logfile does not exist -> {}", monitor_config.log_path().to_string_lossy());
        Err(Error::from(std::io::Error::new(
            ErrorKind::NotFound,
            "file does not exist",
        )))
    }
}

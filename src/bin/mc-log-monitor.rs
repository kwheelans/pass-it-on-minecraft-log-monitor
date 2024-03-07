use std::io::ErrorKind;
use clap::Parser;
use log::{error, LevelFilter};
use pass_it_on::{start_client, Error};
use pass_it_on_minecraft_log_monitor::configuration::MonitorConfigFileParser;
use pass_it_on_minecraft_log_monitor::{monitor_log, CliArgs, LOG_TARGET};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Error> {
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
    Ok(())
}

async fn run(args: CliArgs) -> Result<(), Error> {
    let monitor_config;
    let client_config;

    {
        let parsed_config = MonitorConfigFileParser::try_from(
            std::fs::read_to_string(args.monitor_config().unwrap())?.as_str(),
        )?;
        monitor_config = parsed_config.monitor;
        client_config = parsed_config.client.try_into()?;
    }
    if monitor_config.log_path().exists() {
        let (interface_tx, interface_rx) = mpsc::channel(100);

        tokio::spawn(async move { monitor_log(monitor_config, interface_tx.clone()).await });

        start_client(client_config, interface_rx, None, None).await?;
        Ok(())
    } else {
        error!(target: LOG_TARGET, "Specified logfile does not exist -> {}", monitor_config.log_path().to_string_lossy());
        Err(Error::IOError(std::io::Error::new(ErrorKind::NotFound, "file does not exist")))
    }
}

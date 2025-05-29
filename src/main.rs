use clap::Parser;
use integration_tests_sv2::sniffer::Sniffer;
use tracing::info;

mod cli;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = cli::Args::parse();
    let config = config::Config::from_file(args.config)?;
    let sniffer = Sniffer::new("", config.listen_addr, config.server_addr, false, vec![]);
    info!("Starting sniffer on {}", config.listen_addr);
    sniffer.start();

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;

    info!("Shutting down...");

    Ok(())
}

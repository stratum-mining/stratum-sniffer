use clap::Parser;
use integration_tests_sv2::sniffer::Sniffer;
use integration_tests_sv2::sv1_sniffer::SnifferSV1;
use tracing::info;

mod cli;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = cli::Args::parse();
    let config = config::Config::from_file(args.config)?;
    match config.sv2 {
        true => {
            let sniffer = Sniffer::new(
                "",
                config.listen_addr,
                config.server_addr,
                true,
                vec![],
                None,
            );
            info!("Starting Sv2 sniffer on {}", config.listen_addr);
            info!("Pubkey: 9auqWEzQDVyd2oe1JVGFLMLHZtCo2FFqZwtKA5gd9xbuEu7PH72");
            sniffer.start();
        }
        false => {
            let sniffer = SnifferSV1::new(config.listen_addr, config.server_addr);
            info!("Starting Sv1 sniffer on {}", config.listen_addr);
            sniffer.start();
        }
    }

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;

    info!("Shutting down...");

    Ok(())
}

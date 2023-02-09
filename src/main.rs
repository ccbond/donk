use clap::Parser;
use config::global_config::GlobalConfig;
mod cli;
mod config;
mod core;

#[tokio::main]
async fn main() {
    let config = GlobalConfig::load();
    let cli = cli::BaseCli::parse();

    cli.process(&config).await;
}

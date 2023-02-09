use crate::config::global_config::GlobalConfig;
use crate::core;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "donkey", author = "ccbond", about = "Productivity artifact.")]
pub struct BaseCli {
    #[clap(subcommand)]
    pub commands: FactCli,
}

#[derive(Subcommand)]
pub enum FactCli {
    /// open vsc global settings.json
    #[clap(name = "v")]
    VSCOpenSettings {},
}

impl BaseCli {
    pub async fn process(&self, config: &GlobalConfig) {
        match &self.commands {
            FactCli::VSCOpenSettings {} => core::settings::process(config),
        }
    }
}

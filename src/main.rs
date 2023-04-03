use clap::{Args, Parser, Subcommand};
use config::config::MainConfig;
use download::download;
use runner::Runner;

use eyre::Result;

mod download;
mod runner;
mod import;
mod config;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Command {
    Run(Run),
    Config,
    Proton,
    Import
}

#[derive(Args)]
struct Run {
    id: Option<usize>,
}

fn main() -> Result<(), eyre::Report> {
    let cli = Cli::parse();
    let mut config = MainConfig::new()?;
    let runner = Runner::new(&config, cli.verbose)?;

    match &cli.command {
        Command::Run(id) => match id.id {
            Some(v) => Ok(runner.run_id(v)?),
            None => Ok(runner.run_intr()?),
        },
        Command::Config => Ok(config.config_editor()?),
        Command::Proton => Ok(download()?),
        Command::Import => Ok(import::import_from_lutris()?)
    }
}

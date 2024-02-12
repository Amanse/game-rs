use clap::{Args, Parser, Subcommand};
use config::config::MainConfig;
use download::download;
use runner::Runner;

use eyre::Result;

mod config;
mod download;
mod import;
mod runner;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run(Run),
    Config,
    #[command(subcommand)]
    Download(DownloadOptions),
    Import,
}

#[derive(Subcommand)]
pub enum DownloadOptions {
    Proton,
    ULWGL,
}

#[derive(Args)]
struct Run {
    id: Option<usize>,
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), eyre::Report> {
    let cli = Cli::parse();
    let mut config = MainConfig::new()?;
    let runner = Runner::new(&config)?;

    match &cli.command {
        Command::Run(r_param) => match r_param.id {
            Some(v) => Ok(runner.verbosity(r_param.verbose).run_id(v)?),
            None => Ok(runner.verbosity(r_param.verbose).run_intr()?),
        },
        Command::Config => Ok(config.config_editor()?),
        Command::Download(what) => Ok(download(what)?),
        Command::Import => Ok(import::import_from_lutris()?),
    }
}

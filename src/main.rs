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
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    print_only: bool,
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
    ULGWL,
}

#[derive(Args)]
struct Run {
    id: Option<usize>,
}

fn main() -> Result<(), eyre::Report> {
    let cli = Cli::parse();
    let mut config = MainConfig::new()?;
    let runner = Runner::new(&config, cli.verbose, cli.print_only)?;

    match &cli.command {
        Command::Run(id) => match id.id {
            Some(v) => Ok(runner.run_id(v)?),
            None => Ok(runner.run_intr()?),
        },
        Command::Config => Ok(config.config_editor()?),
        Command::Download(what) => Ok(download(what)?),
        Command::Import => Ok(import::import_from_lutris()?),
    }
}

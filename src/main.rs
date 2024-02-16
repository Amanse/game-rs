use clap::{Args, Parser, Subcommand};
use config::config::MainConfig;
use download::download;

use eyre::Result;

mod config;
mod download;
mod import;

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

    match &cli.command {
        Command::Run(r_param) => {
            let idx = config.game_selector(r_param.id)?;
            config.games[idx].clone().run()
        }
        Command::Config => Ok(config.config_editor()?),
        Command::Download(what) => Ok(download(what)?),
        Command::Import => Ok(import::import_from_lutris()?),
    }
}

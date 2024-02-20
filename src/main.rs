use clap::{Args, Parser, Subcommand};
use config::config2::Config;
use download::download;

use eyre::Result;

mod config;
mod download;

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

    let mut con2 = Config::new();
    match &cli.command {
        Command::Run(r_param) => {
            let g = con2.game_selector()?;
            let game = g.run()?;
            con2.update_with_id(game);

            Ok(())
        }
        Command::Config => {
            con2.editor();
            Ok(())
        }
        Command::Download(what) => Ok(download(what)?),
    }
}

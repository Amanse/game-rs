use clap::{Args, Parser, Subcommand};
use download::download;
use runner::Runner;

use eyre::Result;

mod download;
mod runner;
mod import;

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
    let mut runner = Runner::new(cli.verbose);

    match &cli.command {
        Command::Run(id) => match id.id {
            Some(v) => Ok(runner.run_game(v)?),
            None => Ok(runner.run_intr()?),
        },
        Command::Config => Ok(runner.config_editor()?),
        Command::Proton => Ok(download()?),
        Command::Import => Ok(import::import_from_lutris()?)
    }
}

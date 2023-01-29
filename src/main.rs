use clap::{Args, Parser, Subcommand};
use download::download;
use runner::Runner;
mod runner;
mod download;

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
}

#[derive(Args)]
struct Run {
    id: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let mut runner = Runner::new();

    match &cli.command {
        Command::Run(id) => match id.id {
            Some(v) => runner.run_game(v),
            None => runner.run_intr(),
        },
        Command::Config => {
            runner.config_editor();
        }
        Command::Proton => {
            download();
        }
    }
}

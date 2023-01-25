use clap::{Parser, Subcommand, Args};
use runner::Runner;
mod runner;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    run: RunCommand,
}

#[derive(Subcommand)]
enum RunCommand {
    Run(Run),
}

#[derive(Args)]
struct Run {
    id: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let runner = Runner::new();

    match &cli.run {
        RunCommand::Run(id) => {
            match id.id {
                Some(v) => runner.run_game(v),
                None => println!("No id was given, interactive selection will be here")
            }
        }
    }
}

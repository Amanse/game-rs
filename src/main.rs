use clap::{Parser, Subcommand, Args};
use runner::Runner;
mod runner;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    run: Option<RunCommand>,
    #[arg(short, long)]
    config: bool,
    #[arg(short, long)]
    verbose: bool
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
    let mut runner = Runner::new();

    if cli.config {
        runner.config_editor();
        
    } else {
        match &cli.run.unwrap() {
            RunCommand::Run(id) => {
                match id.id {
                    Some(v) => runner.run_game(v),
                    None => runner.run_intr() 
                }
            }
        }
    }

}

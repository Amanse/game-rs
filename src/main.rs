use runner::Runner;
mod runner;

fn main() {
    let id = std::env::args().nth(1).expect("no id given");
    let id: usize = id.parse().unwrap(); 

    let runner = Runner::new();
    runner.run_game(id);
        
}

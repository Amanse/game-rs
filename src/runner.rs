use serde_derive::{Deserialize, Serialize};
use std::process::Command;

pub struct Runner {
    config: MainConfig
}

#[derive(Serialize, Deserialize, Clone)]
struct MainConfig {
    games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Game {
    id: usize,
    name: String,
    use_nvidia: bool,
    prefix_path: String,
    runner_path: String,
    exect_path: String,
}

impl ::std::default::Default for MainConfig {
    fn default() -> Self {
        Self { games: vec![] }
    }
}

impl Runner {
    pub fn new() -> Self { 
        let cfg = confy::load("game-rs", None).unwrap();
        Runner{config: cfg} 
    }
    
    pub fn run_game(self, id: usize) {
        let prefix_path = self.config.games[id].prefix_path.clone();
        let runner_path = self.config.games[id].runner_path.clone();
        let exect_path = self.config.games[id].exect_path.clone();
        
        Command::new("steam-run")
            .env("WINE_PREFIX", prefix_path)
            .args([runner_path, exect_path])
            .output()
            .expect("Could not run game");
    }
}

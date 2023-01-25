use std::process::Command;

use serde_derive::{Deserialize, Serialize};

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

fn main() {
    let cfg: MainConfig = confy::load("game-rs", None).unwrap();
//    let mut new_cfg: MainConfig = cfg.clone();
  //  new_cfg.games[0].runner_path = String::from("/home/me/.local/share/lutris/runners/wine/lutris-fshack-7.2-x86_64/bin/wine64");
  //  confy::store("game-rs", None,new_cfg).unwrap();
    let id = std::env::args().nth(1).expect("no id given");
    let id: usize = id.parse().unwrap(); 
    let prefix_path = cfg.games[id].prefix_path.clone();
    let runner_path = cfg.games[id].runner_path.clone();
    let exect_path = cfg.games[id].exect_path.clone();
    println!("WINE_PREFIX={} steam-run {} {}", prefix_path, runner_path, exect_path);
    Command::new("steam-run")
        .env("WINE_PREFIX", prefix_path)
        .args([runner_path, exect_path])
        .output()
        .expect("Could not run game");
        
}

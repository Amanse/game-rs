use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

pub struct Runner {
    config: MainConfig,
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
        Runner { config: cfg }
    }

    pub fn run_intr(self) {
        let prompts: Vec<String> = self.config.games.iter().map(|g| format!("{} - {}", g.id.clone(), g.name.clone())).collect();
        let game = FuzzySelect::new()
            .default(0)
            .items(&prompts)
            .interact_opt()
            .unwrap()
            .unwrap();

        self.run_game(game);
    }

    pub fn run_game(self, id: usize) {
        let prefix_path = self.config.games[id].prefix_path.clone();
        let runner_path = self.config.games[id].runner_path.clone();
        let exect_path = self.config.games[id].exect_path.clone();
        let use_nvidia = self.config.games[id].use_nvidia.clone();

        let nvidia_envs: HashMap<&str, &str> = {
            if use_nvidia {
                HashMap::from([
                    ("__NV_PRIME_RENDER_OFFLOAD", "1"),
                    ("__NV_PRIME_RENDER_OFFLOAD", "NVIDIA-G0"),
                    ("__GLX_VENDOR_LIBRARY_NAME", "nvidia"),
                    ("__VK_LAYER_NV_optimus", "NVIDIA_only"),
                    ("WINEPREFIX", prefix_path.as_str()),
                ])
            } else {
                HashMap::from([("WINEPREFIX", prefix_path.as_str())])
            }
        };

        #[cfg(not(feature = "nixos"))]
        runner_default(&nvidia_envs, runner_path, exect_path);

        #[cfg(feature = "nixos")]
        runner_nixos(&nvidia_envs, runner_path, exect_path);
    }

    pub fn config_editor(&mut self) {
        let mode = FuzzySelect::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&vec!["Add game", "Edit game", "Delete game"])
            .interact_opt()
            .unwrap()
            .unwrap();

        if mode == 0 as usize {
            self.add_game()
        } else if mode == 1 as usize {
            self.edit_game()
        } else if mode == 2 as usize {
            self.delete_game()
        } else {
            panic!("What the fuck");
        }
    }

    fn add_game(&mut self) {
        let name: String = Input::new()
            .with_prompt("Name of the game ")
            .interact_text()
            .unwrap();

        let exect_path: String = Input::new()
            .with_prompt("Path to executable")
            .interact_text()
            .unwrap();

        let runner_path: String = Input::new()
            .with_prompt("Path to proton/wine binary")
            .interact_text()
            .unwrap();

        let prefix_path: String = Input::new()
            .with_prompt("Path to prefix(Default $HOME/.wine)")
            .default("$HOME/.wine".to_string())
            .interact_text()
            .unwrap();

        let use_nvidia = Confirm::new()
            .with_prompt("Do you want to run this with nvidia gpu?")
            .interact_opt()
            .unwrap().unwrap();

        let id: usize = {
            if let Some(v) = self.config.games.last() {
                v.id + 1
            } else {
                0
            }
        };

        let new_game = Game{
            prefix_path,
            name,
            id,
            use_nvidia,
            exect_path,
            runner_path
        };

        self.config.games.push(new_game);

        confy::store("game-rs", None, self.config.clone()).unwrap();
        
    }

    fn edit_game(&mut self) {}

    fn delete_game(&mut self) {}
}

#[cfg(feature = "nixos")]
fn runner_nixos(envs: &HashMap<&str, &str>, runner_path: String, exect_path: String) {
    Command::new("steam-run")
        .envs(envs)
        .args([runner_path, exect_path])
        .spawn()
        .expect("Could not run game");
}

#[cfg(not(feature = "nixos"))]
fn runner_default(envs: &HashMap<&str, &str>, runner_path: String, exect_path: String) {
    Command::new(runner_path)
        .envs(envs)
        .args([exect_path])
        .spawn()
        .expect("Could not run game");
}

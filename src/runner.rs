use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

use eyre::{eyre, Result};

pub struct Runner {
    config: MainConfig,
    extra: ExtraConfig,
    is_verbose: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct MainConfig {
    games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ExtraConfig {
    runner_path: String,
    prefix_dir: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Game {
    id: usize,
    name: String,
    use_nvidia: bool,
    prefix_path: String,
    runner_path: String,
    exect_path: String,
    is_native: Option<bool>,
}

impl ::std::default::Default for MainConfig {
    fn default() -> Self {
        Self { games: vec![] }
    }
}

impl ::std::default::Default for ExtraConfig {
    fn default() -> Self {
        Self {
            runner_path: "".to_string(),
            prefix_dir: "".to_string(),
        }
    }
}

impl Runner {
    pub fn new(is_verbose: bool) -> Self {
        let cfg: MainConfig = confy::load("game-rs", None).unwrap_or(MainConfig::default());
        let extra: ExtraConfig = confy::load("game-rs", "Extra").unwrap_or(ExtraConfig::default());
        Runner {
            config: cfg,
            extra,
            is_verbose,
        }
    }

    pub fn run_intr(&self) -> Result<()> {
        let id = self.game_selector()?;

        Ok(self.run_game(id)?)
    }

    pub fn run_game(&self, id: usize) -> Result<()> {
        let prefix_path = self.config.games[id].prefix_path.clone();
        let runner_path = self.config.games[id].runner_path.clone();
        let exect_path = self.config.games[id].exect_path.clone();
        let use_nvidia = self.config.games[id].use_nvidia.clone();

        let mut envs: HashMap<&str, &str> = {
            if use_nvidia {
                HashMap::from([
                    ("__NV_PRIME_RENDER_OFFLOAD", "1"),
                    ("__NV_PRIME_RENDER_OFFLOAD", "NVIDIA-G0"),
                    ("__GLX_VENDOR_LIBRARY_NAME", "nvidia"),
                    ("__VK_LAYER_NV_optimus", "NVIDIA_only"),
                ])
            } else {
                HashMap::from([])
            }
        };

        if prefix_path != "".to_string() {
            envs.insert("WINEPREFIX", prefix_path.as_str());
        }

        runner_main(&envs, runner_path, exect_path, self.is_verbose);
        println!("Finished {}", self.config.games[id].name.clone());
        Ok(())
    }

    pub fn config_editor(&mut self) -> Result<()> {
        let mode = FuzzySelect::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&vec![
                "Add game",
                "Edit game",
                "Delete game",
                "Set default runner path",
                "Add prefix directory",
            ])
            .interact_opt()?
            .ok_or(eyre!("Nothing selected, goodbye"))?;

        if mode == 0 as usize {
            self.add_game()
        } else if mode == 1 as usize {
            self.edit_game()
        } else if mode == 2 as usize {
            self.delete_game()
        } else if mode == 3 as usize {
            let path: String = Input::new()
                .with_prompt("Enter runner executable path")
                .default(self.extra.runner_path.clone())
                .interact_text()?;
            self.extra.runner_path = path;
            confy::store("game-rs", "Extra", self.extra.clone())?;
            Ok(())
        } else if mode == 4 {
            let path: String = Input::new()
                .with_prompt("Prefixes directory")
                .default(self.extra.prefix_dir.clone())
                .interact_text()?;
            self.extra.prefix_dir = path;
            confy::store("game-rs", "Extra", self.extra.clone())?;
            Ok(())
        } else {
            return Err(eyre!(
                "Achievement unlocked: How did we get here(invalid mode selected)"
            ));
        }
    }

    fn add_game(&mut self) -> Result<()> {
        let name: String = Input::new()
            .with_prompt("Name of the game ")
            .interact_text()?;

        let exect_path: String = Input::new()
            .with_prompt("Path to executable")
            .interact_text()?;

        let is_native: bool = Confirm::new()
            .with_prompt("Is it native linux game?")
            .interact_opt()?
            .ok_or(eyre!("select something next time"))?;

        let mut runner_path: String = "".to_string();
        let mut prefix_path: String = "".to_string();

        if !is_native {
            runner_path = Input::new()
                .with_prompt("Path to proton/wine binary")
                .default(self.extra.runner_path.clone())
                .interact_text()?;

            prefix_path = Input::new()
                .with_prompt("Path to prefix (Uses $HOME/.wine) by default")
                .default("".to_string())
                .with_initial_text(self.extra.prefix_dir.clone())
                .show_default(false)
                .interact_text()?;
        }

        let use_nvidia = Confirm::new()
            .with_prompt("Do you want to run this with nvidia gpu?")
            .interact_opt()?
            .ok_or(eyre!("Why not select anything bruh"))?;

        let id: usize = {
            if let Some(v) = self.config.games.last() {
                v.id + 1
            } else {
                0
            }
        };

        let new_game = Game {
            prefix_path,
            name,
            id,
            use_nvidia,
            exect_path,
            runner_path,
            is_native: Some(is_native),
        };

        self.config.games.push(new_game);

        confy::store("game-rs", None, self.config.clone())?;
        Ok(())
    }

    fn edit_game(&mut self) -> Result<()> {
        let id = self.game_selector()?;

        loop {
            let edit_options = [
                "Name",
                "Executable path",
                "Prefix path",
                "Runner path",
                "Exit",
            ];

            let selection: usize = FuzzySelect::new()
                .items(&edit_options)
                .default(0)
                .interact_opt()?
                .ok_or(eyre!("Nothing selected, goodbye"))?;

            match selection {
                0 => {
                    let input: String = Input::new()
                        .default(self.config.games[id].name.clone())
                        .interact_text()?;

                    self.config.games[id].name = input;
                    confy::store("game-rs", None, self.config.clone())?;

                    println!("{} Updated", self.config.games[id].name.clone());
                }
                1 => {
                    let input: String = Input::new()
                        .default(self.config.games[id].exect_path.clone())
                        .interact_text()?;

                    self.config.games[id].exect_path = input;
                    confy::store("game-rs", None, self.config.clone())?;

                    println!("{} Updated", self.config.games[id].name.clone());
                }
                2 => {
                    let input: String = Input::new()
                        .default(self.config.games[id].prefix_path.clone())
                        .interact_text()?;

                    self.config.games[id].prefix_path = input;
                    confy::store("game-rs", None, self.config.clone())?;

                    println!("{} Updated", self.config.games[id].name.clone());
                }
                3 => {
                    let input: String = Input::new()
                        .default(self.config.games[id].runner_path.clone())
                        .interact_text()?;

                    self.config.games[id].runner_path = input;
                    confy::store("game-rs", None, self.config.clone())?;

                    println!("{} Updated", self.config.games[id].name.clone());
                }
                4 => {
                    break;
                }
                _ => return Err(eyre!("Achievement unlocked: What the fuck")),
            }
        }
        Ok(())
    }

    fn delete_game(&mut self) -> Result<()> {
        let id = self.game_selector()?;

        let game = self.config.games[id].clone();

        println!("Executable Path: {}", game.exect_path);
        println!("Prefix: {}", game.prefix_path);

        let confirmation: bool = Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete {} - {}",
                game.id, game.name
            ))
            .interact_opt()?
            .ok_or(eyre!("NOthing selected, goodbye"))?;

        if confirmation {
            self.config.games.remove(id);
            confy::store("game-rs", None, self.config.clone())?;
            println!("Deleted {}", game.name);
            Ok(())
        } else {
            std::process::exit(1);
        }
    }

    fn game_selector(&self) -> Result<usize> {
        let prompts: Vec<String> = self
            .config
            .games
            .iter()
            .map(|g| format!("{} - {}", g.id.clone(), g.name.clone()))
            .collect();

        let id: usize = FuzzySelect::new()
            .with_prompt("Select game")
            .default(0)
            .items(&prompts)
            .interact_opt()?
            .ok_or(eyre!("Nothing selected, goodbye"))?;

        Ok(id)
    }
}

fn runner_main(
    envs: &HashMap<&str, &str>,
    runner_path: String,
    exect_path: String,
    is_verbose: bool,
) {
    use std::path::Path;

    let game_dir = Path::new(&exect_path).parent().unwrap();

    let stdout: std::process::Stdio = {
        if !is_verbose {
            std::process::Stdio::null()
        } else {
            std::process::Stdio::inherit()
        }
    };

    let mut runner_path: String = runner_path.clone();
    let mut exect_path: String = exect_path.clone();

    if runner_path == "".to_string() {
        runner_path = exect_path;
        exect_path = "".to_string();
    }

    #[cfg(feature = "nixos")]
    Command::new("steam-run")
        .current_dir(game_dir)
        .stdout(stdout)
        .envs(envs)
        .args([runner_path, exect_path])
        .output()
        .expect("Could not run game");

    #[cfg(not(feature = "nixos"))]
    Command::new(runner_path)
        .current_dir(game_dir)
        .stdout(stdout)
        .envs(envs)
        .args([exect_path])
        .output()
        .expect("Could not run game");
}

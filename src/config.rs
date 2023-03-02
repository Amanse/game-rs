use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, Select};
use eyre::{eyre, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MainConfig {
    pub games: Vec<Game>,
    pub extra: ExtraConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameList {
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtraConfig {
    pub runner_path: Option<String>,
    pub prefix_dir: Option<String>,
    pub runner_dirs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: usize,
    pub name: String,
    pub use_nvidia: bool,
    pub prefix_path: String,
    pub runner_path: String,
    pub exect_path: String,
    pub is_native: Option<bool>,
}

impl ::std::default::Default for MainConfig {
    fn default() -> Self {
        Self {
            games: vec![],
            extra: ExtraConfig::default(),
        }
    }
}

impl ::std::default::Default for GameList {
    fn default() -> Self {
        Self { games: vec![] }
    }
}

impl ::std::default::Default for ExtraConfig {
    fn default() -> Self {
        Self {
            runner_path: None,
            prefix_dir: None,
            runner_dirs: None,
        }
    }
}

impl MainConfig {
    pub fn new() -> Result<Self> {
        let games: GameList = confy::load("game-rs", None).unwrap_or(GameList::default());
        let extra = ExtraConfig::new()?;
        Ok(Self {
            games: games.games,
            extra,
        })
    }

    pub fn save_games(&self) -> Result<()> {
        confy::store(
            "game-rs",
            None,
            GameList {
                games: self.games.clone(),
            },
        )?;
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
                "Add Runner directory",
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
                .default(self.extra.runner_path.clone().unwrap_or("".to_string()))
                .interact_text()?;
            self.extra.runner_path = Some(path);
            confy::store("game-rs", "Extra", self.extra.clone())?;
            Ok(())
        } else if mode == 4 {
            let path: String = Input::new()
                .with_prompt("Prefixes directory")
                .default(self.extra.prefix_dir.clone().unwrap_or("".to_string()))
                .interact_text()?;
            self.extra.prefix_dir = Some(path);
            confy::store("game-rs", "Extra", self.extra.clone())?;
            Ok(())
        } else if mode == 5 {
            let mut dirs = self.extra.runner_dirs.clone().unwrap_or(vec![]);
            let path: String = Input::new()
                .with_prompt("Another Runner directory")
                .interact_text()?;
            dirs.push(path);

            self.extra.runner_dirs = Some(dirs);
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
            runner_path = self.runner_selector()?;

            prefix_path = Input::new()
                .with_prompt("Path to prefix (Uses $HOME/.wine) by default")
                .default("".to_string())
                .with_initial_text(self.extra.prefix_dir.clone().unwrap_or("".to_string()))
                .show_default(false)
                .interact_text()?;
        }

        let use_nvidia = Confirm::new()
            .with_prompt("Do you want to run this with nvidia gpu?")
            .interact_opt()?
            .ok_or(eyre!("Why not select anything bruh"))?;

        let id: usize = {
            if let Some(v) = self.games.last() {
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

        self.games.push(new_game);

        self.save_games()?;
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
                        .default(self.games[id].name.clone())
                        .interact_text()?;

                    self.games[id].name = input;
                    confy::store("game-rs", None, self.clone())?;

                    println!("{} Updated", self.games[id].name.clone());
                }
                1 => {
                    let input: String = Input::new()
                        .default(self.games[id].exect_path.clone())
                        .interact_text()?;

                    self.games[id].exect_path = input;
                    confy::store("game-rs", None, self.clone())?;

                    println!("{} Updated", self.games[id].name.clone());
                }
                2 => {
                    let input: String = Input::new()
                        .default(self.games[id].prefix_path.clone())
                        .interact_text()?;

                    self.games[id].prefix_path = input;
                    confy::store("game-rs", None, self.clone())?;

                    println!("{} Updated", self.games[id].name.clone());
                }
                3 => {
                    let path = self.runner_selector()?;

                    self.games[id].runner_path = path;
                    self.save_games()?;

                    println!("{} Updated", self.games[id].name.clone());
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

        let game = self.games[id].clone();

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
            self.games.remove(id);
            self.save_games()?;
            println!("Deleted {}", game.name);
            Ok(())
        } else {
            std::process::exit(1);
        }
    }

    pub fn game_selector(&self) -> Result<usize> {
        let prompts: Vec<String> = self
            .games
            .iter()
            .map(|g| format!("{} - {}", g.id.clone(), g.name.clone()))
            .collect();

        let id: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select game")
            .default(0)
            .items(&prompts)
            .interact_opt()?
            .ok_or(eyre!("Nothing selected, goodbye"))?;

        Ok(id)
    }

    fn runner_selector(&self) -> Result<String> {
        #[warn(unused_assignments)]
        let runner_path: String;
        let runner_list = self.extra.get_runners()?;
        let runner_s = Select::new()
            .with_prompt(
                "Wine Runner [You can add runner dir to automatically fetch these in config]",
            )
            .default(0)
            .item("Custom path")
            .items(&runner_list)
            .interact()?;

        if runner_s != 0 {
            runner_path = runner_list[runner_s - 1].clone();
        } else {
            runner_path = Input::new()
                .with_prompt("Path to proton/wine binary")
                .default(self.extra.runner_path.clone().unwrap_or("".to_string()))
                .interact_text()?;
        }
        Ok(runner_path)
    }
}

impl ExtraConfig {
    pub fn new() -> Result<Self> {
        Ok(confy::load("game-rs", "Extra").unwrap_or(ExtraConfig::default()))
    }

    pub fn get_runners(&self) -> Result<Vec<String>> {
        let mut runners = vec![];
        let base_path = format!("{}/lutris/runners/wine", std::env::var("XDG_DATA_HOME")?);
        if std::path::Path::new(&base_path).exists() {
            Self::get_runners_for(base_path, &mut runners)?;
        }
        if let Some(dir) = self.runner_dirs.clone() {
            for p in dir {
                Self::get_runners_for(p, &mut runners)?;
            }
        }
        Ok(runners)
    }

    fn get_runners_for(base_path: String, runners: &mut Vec<String>) -> Result<&mut Vec<String>> {
        match std::fs::read_dir(base_path.clone()) {
            Ok(paths) => {
                for path in paths {
                    let p = path?.path().clone();
                    if let Some(dir) = p.iter().last().clone() {
                        if p.join("bin").exists() {
                            runners.push(format!(
                                "{}/{}/bin/wine",
                                base_path.clone().to_string(),
                                dir.to_str().unwrap()
                            ));
                        }
                    }
                }
            }
            Err(_) => {}
        };
        Ok(runners)
    }
}

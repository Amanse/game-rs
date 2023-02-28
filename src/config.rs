use serde_derive::{Deserialize, Serialize};
use dialoguer::{Input, FuzzySelect, Confirm, theme::ColorfulTheme};
use eyre::{eyre, Result};

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
    pub runner_path: String,
    pub prefix_dir: String,
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
            runner_path: "".to_string(),
            prefix_dir: "".to_string(),
        }
    }
}

impl MainConfig {
    pub fn new() -> Result<Self>{
        let games: GameList = confy::load("game-rs", None).unwrap_or(GameList::default());
        let extra = ExtraConfig::new()?;
        Ok(Self{games: games.games, extra})
    }

    pub fn save_games(&self) -> Result<()> {
        confy::store("game-rs", None, GameList{games: self.games.clone()})?;
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
                    let input: String = Input::new()
                        .default(self.games[id].runner_path.clone())
                        .interact_text()?;

                    self.games[id].runner_path = input;
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

        let id: usize = FuzzySelect::new()
            .with_prompt("Select game")
            .default(0)
            .items(&prompts)
            .interact_opt()?
            .ok_or(eyre!("Nothing selected, goodbye"))?;

        Ok(id)
    }
}

impl ExtraConfig {
    pub fn new() -> Result<Self>{
        Ok(confy::load("game-rs", "Extra").unwrap_or(ExtraConfig::default()))
    }
}

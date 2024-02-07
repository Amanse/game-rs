use crate::config::extra_config::ExtraConfig;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, Select};
use enum_iterator::{all, Sequence};
use eyre::{eyre, Result};
use serde_aux::serde_introspection;
use serde_derive::{Deserialize, Serialize};
use std::any::TypeId;

#[derive(Debug, PartialEq, Eq)]
struct ParseBoolError;

#[derive(Serialize, Deserialize, Clone)]
pub struct MainConfig {
    pub games: Vec<Game>,
    pub extra: ExtraConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameList {
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Clone, lib_reflect::dynamic_update)]
pub struct Game {
    pub id: usize,
    pub name: String,
    pub use_nvidia: bool,
    pub prefix_path: String,
    pub runner_path: String,
    pub exect_path: String,
    //Set to false as before this option was added there was no support for native games
    #[serde(default = "default_false")]
    pub is_native: bool,
    #[serde(default = "default_playtime")]
    pub playtime: u64,
}

#[derive(Debug, PartialEq, Sequence)]
enum ConfigMenu {
    AddGame,
    EditGame,
    DeleteGame,
    PrefixDir,
    RunnerDir,
}

impl std::fmt::Display for ConfigMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigMenu::AddGame => write!(f, "Add game"),
            ConfigMenu::EditGame => write!(f, "Edit game"),
            ConfigMenu::DeleteGame => write!(f, "Delete game"),
            ConfigMenu::PrefixDir => write!(f, "Add prefix directory"),
            ConfigMenu::RunnerDir => write!(f, "Add runners directory"),
        }
    }
}

fn default_playtime() -> u64 {
    0
}

//Set to false as before this option was added there was no support for native games
fn default_false() -> bool {
    false
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

impl MainConfig {
    pub fn new() -> Result<Self> {
        #[cfg(debug_assertions)]
        let games: GameList = confy::load("game-rs", "debug").unwrap_or(GameList::default());

        #[cfg(not(debug_assertions))]
        let games: GameList = confy::load("game-rs", None).unwrap_or(GameList::default());

        let extra = ExtraConfig::new()?;
        Ok(Self {
            games: games.games,
            extra,
        })
    }

    pub fn save_games(&self) -> Result<()> {
        #[cfg(debug_assertions)]
        confy::store(
            "game-rs",
            "debug",
            GameList {
                games: self.games.clone(),
            },
        )?;

        #[cfg(not(debug_assertions))]
        confy::store(
            "game-rs",
            None,
            GameList {
                games: self.games.clone(),
            },
        )?;
        Ok(())
    }

    //@TODO: Move functions to Game struct impl
    pub fn add_playtime(&self, id: usize, to_add: u64) -> Result<()> {
        let mut new = self.clone();
        new.games[id].playtime += to_add;
        new.save_games()
    }

    pub fn config_editor(&mut self) -> Result<()> {
        let menu_modes = all::<ConfigMenu>().collect::<Vec<_>>();
        let select_mode = FuzzySelect::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&menu_modes)
            .interact_opt()?
            .ok_or(eyre!("Nothing selected, goodbye"))?;

        match menu_modes[select_mode] {
            ConfigMenu::AddGame => self.add_game(),
            ConfigMenu::EditGame => self.edit_game(),
            ConfigMenu::DeleteGame => self.delete_game(),
            ConfigMenu::PrefixDir => {
                let path: String = Input::new()
                    .with_prompt("Prefixes directory")
                    .default(self.extra.prefix_dir.clone().unwrap_or("".to_string()))
                    .interact_text()?;
                self.extra.prefix_dir = Some(path);
                confy::store("game-rs", "Extra", self.extra.clone())?;
                return Ok(());
            }
            ConfigMenu::RunnerDir => {
                let mut dirs = self.extra.runner_dirs.clone().unwrap_or(vec![]);
                let path: String = Input::new()
                    .with_prompt("Another Runner directory")
                    .interact_text()?;
                dirs.push(path);

                self.extra.runner_dirs = Some(dirs);
                confy::store("game-rs", "Extra", self.extra.clone())?;
                return Ok(());
            }
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
        let mut is_ulwgl: bool = false;

        if !is_native {
            let use_ulwgl_proton: bool = Confirm::new()
                .with_prompt("Auto download proton with ulwgl?")
                .default(true)
                .interact_opt()?
                .ok_or(eyre!("Invalid data"))?;

            if !use_ulwgl_proton {
                runner_path = self.extra.runner_selector()?;
            }

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
            is_native,
            playtime: 0,
        };

        self.games.push(new_game);

        self.save_games()?;
        Ok(())
    }

    fn edit_game(&mut self) -> Result<()> {
        let id = self.game_selector()?;
        let mut game = self.games[id].clone();
        let fields = serde_introspection::serde_introspect::<Game>();

        loop {
            let selection = Select::new()
                .items(&fields)
                .item("Save")
                .default(0)
                .interact_opt()?
                .unwrap();

            if selection == fields.len() {
                break;
            }

            if fields[selection] == "runner_path" {
                let new_val = self.extra.runner_selector()?;
                game.runner_path = new_val;
                continue;
            }

            let new_val: String = {
                match game.get(fields[selection]) {
                    Ok(v) => {
                        if v == TypeId::of::<bool>() {
                            let c = dialoguer::Confirm::new()
                                .with_prompt(fields[selection])
                                .interact()
                                .unwrap();
                            if c {
                                "true".to_string()
                            } else {
                                "false".to_string()
                            }
                        } else {
                            dialoguer::Input::new()
                                .with_prompt(fields[selection])
                                .interact_text()
                                .unwrap()
                        }
                    }
                    Err(_) => panic!("bitch"),
                }
            };

            game = game.clone().update(fields[selection], new_val.as_str())?;
        }
        self.games[id] = game;
        self.save_games()?;

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
            .map(|g| {
                format!(
                    "{} - {} ({}m)",
                    g.id.clone(),
                    g.name.clone(),
                    g.playtime.clone() / 60
                )
            })
            .collect();

        let id: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select game")
            .default(0)
            .items(&prompts)
            .interact_opt()?
            .ok_or(eyre!("Nothing selected, goodbye"))?;

        Ok(id)
    }
}

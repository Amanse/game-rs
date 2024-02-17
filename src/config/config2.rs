use std::{collections::HashMap, io::Write};

use crate::config::game::Game;
use config::Config;
use serde_derive::{Deserialize, Serialize};

use eyre::Result;
use log::{info, warn};

use super::menu::Menu;

#[derive(Serialize, Deserialize)]
pub struct State {
    games: Vec<Game>,
}

impl State {
    //Needs to be able to load games into a vector
    //
    //Needs to perform CRUD operation on the said vector
    //I say, seperate the service files, the functions that modify in this file while the front end
    //which user will interact with into a different file
    //
    //
    pub fn new() -> Result<State> {
        let conf_path = format!("{}/.config/game-rs/debug.toml", std::env::var("HOME")?);

        if !Self::path_exists(&conf_path)? {
            warn!("Config file not found, creating one");
            Self::create_config(&conf_path)?;
        }

        let builder = Config::builder()
            .add_source(config::File::with_name(&conf_path))
            .build()
            .unwrap();

        let state = builder.try_deserialize::<State>()?;

        info!("Config loaded");

        Ok(state)
    }

    fn create_config(path: &str) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(b"games=")?;

        Ok(())
    }

    fn path_exists(path: &str) -> Result<bool> {
        use std::fs;
        let metadata = fs::metadata(path)?;
        Ok(metadata.is_file())
    }

    pub fn editor(&mut self) -> Result<()> {
        let mut opts: HashMap<&str, fn(&mut Self) -> &mut Self> = HashMap::new();
        opts.insert("Add game", |state: &mut State| state.add_game());
        opts.insert("Delete game", |state: &mut State| state.delete_game());

        // Menu takes in a hashmap of &str and function associated with that string
        // for example "Delete game" has the associated delete_game function with it, this way the
        // service functions can also be moved into a different file with just taking a vector and
        // mutating it based on the index provided, you can implement entire crud opeartions just
        // from one menu
        let menu = Menu::new(opts)?;
        menu.select_and_run(self)?;
        Ok(())
    }

    pub fn add_game(&mut self) -> &mut Self {
        eprint!("YAY");
        todo!()
    }

    pub fn delete_game(&mut self) -> &mut Self {
        eprint!("YAY");
        todo!()
    }
}

use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
struct LutrisConfig {
    game: LutrisGame,
}

#[derive(Serialize, Deserialize)]
struct LutrisGame {
    exe: String,
    prefix: String,
}

impl ::std::default::Default for LutrisConfig {
    fn default() -> Self {
        Self {
            game: LutrisGame {
                exe: "".to_string(),
                prefix: "".to_string(),
            },
        }
    }
}

pub fn import_from_lutris() -> Result<()> {
    let config_folder = std::env::var("XDG_CONFIG_HOME").map(|x| Path::new(&x).to_path_buf())?;
    let lutris_dir = config_folder.join("lutris/games");

    for file in fs::read_dir(lutris_dir)? {
        let f = fs::File::open(file.unwrap().path())?;
        let game: LutrisConfig = serde_yaml::from_reader(f)?;
        println!("{}", game.game.exe);
    }

    todo!()
}

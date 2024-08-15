use eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command};

use super::{
    extra::ExtraConfig,
    util::{self, bool_input, string_input},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

impl ToString for Game {
    fn to_string(&self) -> String {
        format!("{} - {} ({})", self.id, self.name, self.playtime)
    }
}

fn default_playtime() -> u64 {
    0
}

//Set to false as before this option was added there was no support for native games
fn default_false() -> bool {
    false
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            playtime: 0,
            // Apparently dialogure also uses builder pattern this way
            // https://docs.rs/dialoguer/latest/src/dialoguer/prompts/fuzzy_select.rs.html#381
            name: "".to_string(),
            exect_path: "".to_string(),
            prefix_path: "".to_string(),
            runner_path: "".to_string(),
            use_nvidia: false,
            is_native: false,
        }
    }

    pub fn take_user_input(self, extra: ExtraConfig) -> Result<Game> {
        let mut game = self
            .clone()
            .set_name(string_input("Name Of the game", self.name.clone()))
            .set_exect(string_input("Executable path", self.exect_path.clone()))
            .set_native(bool_input("Is this a native game?"));

        if !game.is_native {
            let prefix = {
                if self.prefix_path.is_empty() {
                    extra.prefix_selector()?
                } else {
                    util::string_input("Prefix dir", self.prefix_path)
                }
            };
            game = game.set_wine_params(prefix, extra.runner_selector()?);
        }

        game = game.set_nvidia(bool_input("Use nvidia GPU?"));

        Ok(game)
    }

    pub fn set_native(mut self, native: bool) -> Self {
        self.is_native = native;
        self
    }

    pub fn set_exect(mut self, exect_path: String) -> Self {
        self.exect_path = exect_path;
        self
    }

    pub fn set_wine_params(mut self, prefix_path: String, runner_path: String) -> Self {
        self.prefix_path = prefix_path;
        self.runner_path = runner_path;
        self
    }

    pub fn set_nvidia(mut self, nvidia: bool) -> Self {
        self.use_nvidia = nvidia;
        self
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn run(mut self, is_verbose: bool) -> Result<Game> {
        let mut cmd = self.gen_cmd()?;
        self.run_cmd(&mut cmd, is_verbose)
    }

    fn gen_cmd(&self) -> Result<Command> {
        // Change the args based on nixos feature or not
        // Use sh -c as the main so that only args differ

        let mut cmd = Command::new("sh");

        if !self.is_native {
            cmd.arg("umu-run");
        }

        cmd.arg(self.exect_path.clone());

        cmd.env("WINEPREFIX", self.prefix_path.clone());
        if !self.runner_path.is_empty() {
            cmd.env("PROTONPATH", self.runner_path.clone());
        }
        cmd.env("GAMEID", "game-rs");

        if self.use_nvidia {
            let nvidia_envs = HashMap::from([
                ("__NV_PRIME_RENDER_OFFLOAD", "1"),
                ("__NV_PRIME_RENDER_OFFLOAD", "NVIDIA-G0"),
                ("__GLX_VENDOR_LIBRARY_NAME", "nvidia"),
                ("__VK_LAYER_NV_optimus", "NVIDIA_only"),
            ]);

            cmd.envs(nvidia_envs);
        }

        Ok(cmd)
    }

    fn run_cmd(&mut self, cmd: &mut Command, is_verbose: bool) -> Result<Game> {
        //Execute the command and return Game object with updated runtime

        let start = std::time::Instant::now();

        if is_verbose {
            cmd.status().unwrap();
        } else {
            cmd.output().unwrap();
        }

        let played = start.elapsed().as_secs();
        self.playtime += played;
        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::Game;

    fn get_ulwgl_exec_path() -> String {
        format!("umu-run")
    }

    fn get_game() -> Game {
        Game {
            id: 0,
            name: "test-game".to_string(),
            prefix_path: "/home/me/prefix".to_string(),
            runner_path: "/home/me/proton".to_string(),
            exect_path: "/home/me/exec".to_string(),
            playtime: 0,
            is_native: false,
            use_nvidia: false,
        }
    }

    #[test]
    fn env_test() {
        let game = get_game();
        let cmd = game.gen_cmd().unwrap();

        let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();

        assert_eq!(
            envs,
            &[
                (OsStr::new("GAMEID"), Some(OsStr::new("game-rs"))),
                (
                    OsStr::new("PROTONPATH"),
                    Some(OsStr::new("/home/me/proton"))
                ),
                (
                    OsStr::new("WINEPREFIX"),
                    Some(OsStr::new("/home/me/prefix"))
                ),
            ]
        );
    }

    #[test]
    fn args_test() {
        let game = get_game();
        let cmd = game.gen_cmd().unwrap();

        let args: Vec<&OsStr> = cmd.get_args().collect();

        #[cfg(feature = "nixos")]
        assert_eq!(args, &[&get_ulwgl_exec_path(), "/home/me/exec"]);

        #[cfg(not(feature = "nixos"))]
        assert_eq!(args, &[&get_ulwgl_exec_path(), "/home/me/exec"]);
    }
}

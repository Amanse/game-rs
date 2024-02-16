use eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::{any::TypeId, collections::HashMap, process::Command};

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

fn default_playtime() -> u64 {
    0
}

//Set to false as before this option was added there was no support for native games
fn default_false() -> bool {
    false
}

impl Game {
    pub fn run(mut self) -> Result<()> {
        let mut cmd = self.gen_cmd()?;
        self.run_cmd(&mut cmd)?;
        Ok(())
    }

    fn gen_cmd(&self) -> Result<Command> {
        // Change the args based on nixos feature or not
        // Use sh -c as the main so that only args differ

        let mut cmd = Command::new("sh");

        if !self.is_native {
            let ulwgl_path = format!("{}/.local/share/ULWGL", std::env::var("HOME")?);

            #[cfg(feature = "nixos")]
            cmd.arg("steam-run");

            cmd.arg(format!("{}/ulwgl-run", ulwgl_path.clone()));
        }

        cmd.arg(self.exect_path.clone());

        cmd.env("WINEPREFIX", self.prefix_path.clone());
        cmd.env("PROTONPATH", self.runner_path.clone());
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

    fn run_cmd(&mut self, cmd: &mut Command) -> Result<Game> {
        //Execute the command and return Game object with updated runtime

        let start = std::time::Instant::now();
        cmd.spawn().unwrap();
        let played = start.elapsed().as_secs();
        self.playtime += played;
        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::Game;

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
        assert_eq!(
            args,
            &[
                "steam-run",
                "/home/me/.local/share/ULWGL/ulwgl-run",
                "/home/me/exec"
            ]
        );

        #[cfg(not(feature = "nixos"))]
        assert_eq!(
            args,
            &["/home/me/.local/share/ULWGL/ulwgl-run", "/home/me/exec"]
        );
    }
}

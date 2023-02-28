use std::collections::HashMap;
use std::process::Command;

use eyre::Result;
use crate::config::MainConfig;

pub struct Runner<'a> {
    config: &'a MainConfig,
    is_verbose: bool,
}

impl<'a> Runner<'a> {
    pub fn new(config: &'a MainConfig, is_verbose: bool) -> Result<Self>{
        Ok(Runner {
            config,
            is_verbose,
        })
    }

    pub fn run_intr(&self) -> Result<()> {
        let id = self.config.game_selector()?;

        Ok(self.run_game(id)?)
    }

    pub fn run_game(&self, id: usize) -> Result<()> {
        let game = self.config.games[id].clone();

        let mut envs: HashMap<&str, &str> = {
            if game.use_nvidia {
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

        if game.prefix_path != "".to_string() {
            envs.insert("WINEPREFIX", game.prefix_path.as_str());
        }

        runner_main(&envs, game.runner_path, game.exect_path, self.is_verbose);
        println!("Finished {}", self.config.games[id].name.clone());
        Ok(())
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

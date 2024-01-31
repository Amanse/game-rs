use std::collections::HashMap;
use std::process::Command;

use crate::config::config::MainConfig;
use eyre::Result;

pub struct Runner<'a> {
    config: &'a MainConfig,
    is_verbose: bool,
    print_only: bool,
}

impl<'a> Runner<'a> {
    pub fn new(config: &'a MainConfig, is_verbose: bool, print_only: bool) -> Result<Self> {
        Ok(Runner {
            config,
            is_verbose,
            print_only,
        })
    }

    pub fn run_intr(&self) -> Result<()> {
        let id = self.config.game_selector()?;

        Ok(self.run_game(id)?)
    }

    pub fn run_id(&self, id: usize) -> Result<()> {
        let idx = self.config.games.iter().position(|a| a.id == id).unwrap();
        Ok(self.run_game(idx)?)
    }

    pub fn run_game(&self, id: usize) -> Result<()> {
        let game = self.config.games[id].clone();

        println!("Running {}", game.name);

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

        if game.is_ulwgl {
            envs.insert("PROTONPATH", game.runner_path.as_str());
            envs.insert("GAMEID", "game-rs");

            run_ulwgl(
                &envs,
                self.config
                    .extra
                    .ulwgl_path
                    .clone()
                    .expect("ULGWL path not set in config"),
                game.exect_path,
            );

            return Ok(());
        }

        if self.print_only {
            println!(
                "WINEPREFIX=\"{}\" \"{}\" \"{}\"",
                game.prefix_path, game.runner_path, game.exect_path,
            );
            return Ok(());
        }

        let start = std::time::Instant::now();
        runner_main(&envs, game.runner_path, game.exect_path, self.is_verbose);
        let played = start.elapsed().as_secs();
        println!("Played {} for {} minutes", game.name.clone(), played / 60);
        self.config.add_playtime(id, played)?;
        Ok(())
    }
}

fn run_ulwgl(envs: &HashMap<&str, &str>, ulwgl_path: String, exect_path: String) {
    let ulwgl_path = {
        if ulwgl_path.chars().last().unwrap() != '/' {
            format!("{}/gamelauncher.sh", ulwgl_path)
        } else {
            format!("{}gamelauncher.sh", ulwgl_path)
        }
    };

    let mut args: Vec<String> = vec![];

    #[cfg(feature = "nixos")]
    args.push(ulwgl_path);

    args.push(exect_path.clone());

    #[cfg(feature = "nixos")]
    run_cmd(String::from("steam-run"), args, envs);

    #[cfg(not(feature = "nixos"))]
    run_cmd(ulwgl_path, args, envs);
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

fn run_cmd(main_program: String, args: Vec<String>, envs: &HashMap<&str, &str>) {
    Command::new(main_program)
        .args(args)
        .envs(envs)
        .status()
        .expect("Could not run game");
}

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use crate::config::config::MainConfig;
use crate::download::download_ulwgl;
use eyre::Result;

pub struct Runner<'a> {
    config: &'a MainConfig,
    is_verbose: bool,
}

impl<'a> Runner<'a> {
    pub fn new(config: &'a MainConfig, is_verbose: bool) -> Result<Self> {
        Ok(Runner { config, is_verbose })
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

        let start = std::time::Instant::now();

        if !game.is_native {
            envs.insert("PROTONPATH", game.runner_path.as_str());
            envs.insert("GAMEID", "game-rs");

            run_ulwgl(&envs, game.exect_path, self.is_verbose)?;
        } else {
            run_native(&envs, game.exect_path, self.is_verbose)?;
        }

        let played = start.elapsed().as_secs();
        println!("Played {} for {} minutes", game.name.clone(), played / 60);
        self.config.add_playtime(id, played)?;

        Ok(())
    }
}

fn run_native(envs: &HashMap<&str, &str>, exect_path: String, is_verbose: bool) -> Result<()> {
    let mut args: Vec<String> = vec![];

    args.push(exect_path.clone());

    #[cfg(feature = "nixos")]
    run_cmd(String::from("steam-run"), args, envs, is_verbose)?;

    #[cfg(not(feature = "nixos"))]
    run_cmd("bash".to_string(), args, envs, is_verbose)?;

    Ok(())
}

fn run_ulwgl(envs: &HashMap<&str, &str>, exect_path: String, is_verbose: bool) -> Result<()> {
    let ulwgl_path = String::from("~/.local/share/ULWGL/ulwgl-run");

    if !Path::new(&ulwgl_path).exists() {
        println!("ULWGL not installed, installing now!");
        download_ulwgl()?;
    }

    let mut args: Vec<String> = vec![];

    #[cfg(feature = "nixos")]
    args.push(ulwgl_path);

    args.push(exect_path.clone());

    #[cfg(feature = "nixos")]
    run_cmd(String::from("steam-run"), args, envs, is_verbose)?;

    #[cfg(not(feature = "nixos"))]
    run_cmd(ulwgl_path, args, envs, is_verbose)?;

    Ok(())
}

fn run_cmd(
    main_program: String,
    args: Vec<String>,
    envs: &HashMap<&str, &str>,
    is_verbose: bool,
) -> Result<()> {
    #[cfg(feature = "nixos")]
    let binding = {
        if args.len() == 1 {
            // Native game
            args[0].clone()
        } else {
            // Wine game, first argument is ulwgl
            args[1].clone()
        }
    };

    #[cfg(not(feature = "nixos"))]
    let binding = main_program.clone();
    let path = std::env::current_dir()?;

    let game_dir = Path::new(&binding).parent().unwrap_or(&path);

    if is_verbose {
        Command::new(main_program)
            .current_dir(game_dir)
            .args(args)
            .envs(envs)
            .status()
            .expect("Could not run game");
    } else {
        Command::new(main_program)
            .current_dir(game_dir)
            .args(args)
            .envs(envs)
            .output()
            .expect("Could not run game");
    }

    Ok(())
}

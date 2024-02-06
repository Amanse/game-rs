use std::collections::HashMap;
use std::process::Command;

use crate::config::config::{Game, MainConfig};
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

            run_ulwgl(&envs, game.exect_path, self.is_verbose);

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
        runner_main(&envs, &game, self.is_verbose);
        let played = start.elapsed().as_secs();
        println!("Played {} for {} minutes", game.name.clone(), played / 60);
        self.config.add_playtime(id, played)?;
        Ok(())
    }
}

fn run_ulwgl(envs: &HashMap<&str, &str>, exect_path: String, is_verbose: bool) {
    let ulwgl_path = String::from("~/.local/share/ULWGL/ulwgl-run");

    let mut args: Vec<String> = vec![];

    #[cfg(feature = "nixos")]
    args.push(ulwgl_path);

    args.push(exect_path.clone());

    #[cfg(feature = "nixos")]
    run_cmd(String::from("steam-run"), args, envs, is_verbose);

    #[cfg(not(feature = "nixos"))]
    run_cmd(ulwgl_path, args, envs, is_verbose);
}

fn runner_main(envs: &HashMap<&str, &str>, game: &Game, is_verbose: bool) {
    let mut args: Vec<String> = vec![];
    #[cfg(not(feature = "nixos"))]
    let mut main_program: String;

    // Different If statement cuz using #[cfg] inside same if is experimental apparently

    //On Nixos, put the runner and exect_path in args, as main_program will be steam-run but if it
    //is native game then only put exect_path as runner doesn't exist

    #[cfg(feature = "nixos")]
    if !game.is_native {
        args.push(game.runner_path.clone());
        args.push(game.exect_path.clone());
    } else {
        args.push(game.exect_path.clone());
    }

    //On non-nixos systems, the main program should be the runner and exect should be the argument
    //while for native games the exec itself will be the main_program
    #[cfg(not(feature = "nixos"))]
    if !game.is_native {
        main_program = game.runner_path.clone();
        args.push(game.exect_path.clone());
    } else {
        main_program = game.exect_path.clone();
    }

    #[cfg(feature = "nixos")]
    run_cmd("steam-run".to_string(), args, envs, is_verbose);

    #[cfg(not(feature = "nixos"))]
    run_cmd(main_program, args, envs, is_verbose);
}

fn run_cmd(main_program: String, args: Vec<String>, envs: &HashMap<&str, &str>, is_verbose: bool) {
    if is_verbose {
        Command::new(main_program)
            .args(args)
            .envs(envs)
            .status()
            .expect("Could not run game");
    } else {
        Command::new(main_program)
            .args(args)
            .envs(envs)
            .output()
            .expect("Could not run game");
    }
}

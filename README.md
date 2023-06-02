# A minimal (cli) Game launcher for linux written in rust

[![Rust](https://github.com/Amanse/game-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Amanse/game-rs/actions/workflows/rust.yml)

## Install
`cargo install --path .`
or use [AUR](https://aur.archlinux.org/packages/game-rs-bin) (Thankyou proledatarian. very cool)
### for nixos
#### Without flakes
`cargo install --features nixos --path .` to compile and install manually  
#### With flakes
You can do `nix run` to run it or you can add it as a package in your configuration
```nix
#flake.nix
{
  inputs = {
    game-rs.url = "github:amanse/game-rs";
  };
}
```
and then you can add it in your packages with
```nix
{
  environment.systemPackages = with pkgs; [
    game-rs.packages.x86_64-linux.default
  ];
}
```
You can also use [Cachix](https://game-rs.cachix.org) to get binary cache <br />
To add cachix:
```nix
{
  nix.settings = {
    substituters = ["https://game-rs.cachix.org"];
    trusted-public-keys = ["game-rs.cachix.org-1:fsu+ijfA2GCUE2QX0af80D7x9PCZS79EZbqwtOtlIhA="];
  };
}
```
cachix works with both adding as a package and just doing `nix build` or `nix run`

## Usage
`game-rs config` to go into interactive config mode where you can add, edit or delete the games in config <br />
`game-rs run` to get a fuzzy select menu of all the games in config, selecting it runs the game <br />
`game-rs run <id>` to directly run the game with specified id <br />
`game-rs proton` Download and extract latest wine-ge-custom <br />

unplanned but maybe once it is complete
- GUI app with tauri or soemthing else


# A minimal (cli) Game launcher for linux written in rust

## Install
`cargo install --path .` or for nixos `cargo install --features nixos --path .`
or use [AUR](https://aur.archlinux.org/packages/game-rs) (Thankyou proledatarian. very cool)

## Usage
`game-rs config` to go into interactive config mode where you can add, edit or delete the games in config <br />
`game-rs run` to get a fuzzy select menu of all the games in config, selecting it runs the game <br />
`game-rs run <id>` to directly run the game with specified id <br />
`game-rs proton` Download and extract latest wine-ge-custom <br />

unplanned but maybe once it is complete
- GUI app with tauri or soemthing else
- playtime counting 

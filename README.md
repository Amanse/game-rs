# A minimal (cli) Game launcher for linux written in rust

Right now can it can:
<br />
- Add game to config
- compile seperately for nixos to use steam-run
- run games from the config

## Install
`cargo install --path .` or for nixos `cargo install --features nixos --path .`

## Usage
`game-rs -c` to go into interactive config mode where you can add, edit or delete the games in config <br />
`game-rs run` to get a fuzzy select menu of all the games in config, selecting it runs the game <br />
`game-rs run <id>` to directly run the game with specified id <br />

unplanned but maybe once it is complete
- GUI app with tauri or soemthing else
- playtime counting 
- downloading runners itself without needing another program

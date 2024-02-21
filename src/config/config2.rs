use dialoguer::FuzzySelect;
use serde_derive::{Deserialize, Serialize};

use super::{game::Game, menu::Menu};

use eyre::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    games: Vec<Game>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            games: Default::default(),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        #[cfg(debug_assertions)]
        let config: Config = confy::load("game-rs", "debug").unwrap_or(Config::default());

        #[cfg(not(debug_assertions))]
        let config: Config = confy::load("game-rs", None).unwrap_or(vec![]);

        Ok(config)
    }

    fn save_config(&self) {
        #[cfg(debug_assertions)]
        confy::store("game-rs", "debug", self).unwrap();
        #[cfg(not(debug_assertions))]
        confy::store("game-rs", None, self).unwrap();
    }

    pub fn update_with_id(&mut self, game: Game) {
        let idx = self.get_game_idx(game.id).unwrap();
        self.games[idx] = game.clone();
    }

    fn get_next_id(&self) -> usize {
        if let Some(v) = self.games.last() {
            v.id + 1
        } else {
            0
        }
    }

    pub fn game_selector(&self) -> Result<Game> {
        let sel = FuzzySelect::new()
            .with_prompt("Select game")
            .items(&self.games.clone())
            .interact_opt()
            .unwrap()
            .unwrap();

        Ok(self.games[sel].clone())
    }

    fn get_game_idx(&self, id: usize) -> Result<usize> {
        Ok(self.games.iter().position(|a| a.id == id).unwrap())
    }

    pub fn editor(&mut self) {
        let mut menu = Menu::new();
        menu.add_option("Add Game", &Self::add_game);
        menu.add_option("Update Game", &Self::update_game);
        menu.add_option("Delete Game", &Self::delete_game);

        let a = menu.user_select();
        a(self);
        // Self.save
        self.save_config();
    }

    pub fn add_game(&mut self) -> &mut Self {
        let id = self.get_next_id();
        let game = Game::take_user_input(Game::new().set_id(id)).unwrap();
        self.games.push(game);
        self
    }

    pub fn delete_game(&mut self) -> &mut Self {
        // Call game selector and then call the main deleting function on that index
        let idx = self.get_game_idx(self.game_selector().unwrap().id).unwrap();

        self.games.remove(idx);
        self
    }

    pub fn update_game(&mut self) -> &mut Self {
        // Call game selector and then call the main updating function on that index
        let g = self.game_selector().unwrap();
        let game = Game::take_user_input(g.clone()).unwrap();
        self.update_with_id(game);
        self
    }
}

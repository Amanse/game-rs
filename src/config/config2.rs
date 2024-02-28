use dialoguer::FuzzySelect;
use serde_derive::{Deserialize, Serialize};

use super::{extra::ExtraConfig, game::Game, menu::Menu};

use eyre::Result;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    games: Vec<Game>,
    extra: ExtraConfig,
}

impl Config {
    pub fn new() -> Result<Self> {
        #[cfg(debug_assertions)]
        let config: Config = confy::load("game-rs", "debug").unwrap_or(Config::default());

        #[cfg(not(debug_assertions))]
        let config: Config = confy::load("game-rs", None).unwrap_or(Config::default());

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
        menu.add_option("Edit extra Options", &Self::extra_editor);

        let a = menu.user_select();
        a(self);
        // Self.save
        self.save_config();
    }

    fn extra_editor(&mut self) -> &mut Self {
        ExtraConfig::editor(&mut self.extra);
        println!(
            "{}",
            self.extra.prefix_dir.clone().unwrap_or("".to_string())
        );
        self
    }

    pub fn add_game(&mut self) -> &mut Self {
        let id = self.get_next_id();
        let game = Game::take_user_input(Game::new().set_id(id), self.extra.clone()).unwrap();
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
        let game = Game::take_user_input(g.clone(), self.extra.clone()).unwrap();
        self.update_with_id(game);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{extra::ExtraConfig, game::Game};

    use super::Config;
    fn get_game(id: usize) -> Game {
        Game {
            id,
            name: "test-game".to_string(),
            prefix_path: "/home/me/prefix".to_string(),
            runner_path: "/home/me/proton".to_string(),
            exect_path: "/home/me/exec".to_string(),
            playtime: 0,
            is_native: false,
            use_nvidia: false,
        }
    }

    fn get_extra() -> ExtraConfig {
        ExtraConfig {
            prefix_dir: Some("/home/me/prefixes/".to_string()),
            runner_dirs: Some(vec!["/home/me/runners".to_string()]),
        }
    }

    fn get_config() -> Config {
        Config {
            games: vec![get_game(0), get_game(1), get_game(3)],
            extra: get_extra(),
        }
    }

    #[test]
    fn test_update_with_id() {
        let mut conf = get_config();
        let newg = get_game(3).set_name("Test success".to_string());

        conf.update_with_id(newg.clone());
        let idx = conf.get_game_idx(3).unwrap();
        let got = conf.games[idx].clone();

        assert_eq!(got, newg);
    }
}

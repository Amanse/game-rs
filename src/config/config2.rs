use dialoguer::FuzzySelect;

use super::{game::Game, menu::Menu};

use eyre::Result;

pub struct Config {
    games: Vec<Game>,
}

impl Config {
    pub fn print_games(&self) {
        println!("{:?}", self.games);
    }

    //@TODO: impl default for Config, impl new
    //impl save config, use confy

    pub fn new() -> Self {
        Config { games: vec![] }
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

    pub fn editor(&mut self) {
        let mut menu = Menu::new();
        menu.add_option("Add Game", &Self::add_game);
        menu.add_option("Update Game", &Self::update_game);
        menu.add_option("Delete Game", &Self::delete_game);

        let a = menu.user_select();
        a(self);
    }

    pub fn add_game(&mut self) -> &mut Self {
        let id = self.get_next_id();
        let game = Game::take_user_input(Game::new().set_id(id)).unwrap();
        self.games.push(game);
        self
    }

    pub fn delete_game(&mut self) -> &mut Self {
        // Call game selector and then call the main deleting function on that index
        todo!()
    }

    pub fn update_game(&mut self) -> &mut Self {
        // Call game selector and then call the main updating function on that index
        todo!()
    }
}

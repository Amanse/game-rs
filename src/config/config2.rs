use super::{game::Game, menu::Menu};

pub struct Config {
    games: Vec<Game>,
}

impl Config {
    pub fn print_games(&self) {
        println!("{:?}", self.games);
    }

    pub fn new() -> Self {
        Config { games: vec![] }
    }

    pub fn editor(&mut self) {
        let mut menu = Menu::new();
        menu.add_option("Add", &Self::add_game);

        let a = menu.user_select();
        let next_idx = 0;
        a(self, next_idx);
    }

    pub fn add_game(&mut self, id: usize) -> &mut Self {
        todo!()
    }
}

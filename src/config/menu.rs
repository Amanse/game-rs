use core::hash::Hash;
use std::{collections::HashMap, fmt::Display, ops::Deref};

use dialoguer::FuzzySelect;

pub struct Menu<'a, T: Display + Hash + Eq + ?Sized, S> {
    options: HashMap<&'a T, &'a dyn Fn(&mut S, usize) -> &mut S>,
}

impl<'a, T: Display + Hash + Eq + ?Sized, S> Menu<'a, T, S> {
    pub fn new() -> Menu<'a, T, S> {
        Menu {
            options: HashMap::from([]),
        }
    }

    //Fuck it just call the add function with random usize
    pub fn add_option(&mut self, val: &'a T, f: &'a dyn Fn(&mut S, usize) -> &mut S) -> &mut Self {
        self.options.insert(val, f);
        self
    }

    pub fn user_select(&self) -> &'a dyn Fn(&mut S, usize) -> &mut S {
        let mut items = vec![];

        for key in self.options.keys() {
            items.push(key);
        }

        let sel = FuzzySelect::new()
            .items(&items)
            .interact_opt()
            .unwrap()
            .unwrap();

        let key = items[sel].deref();
        return self.options[key];
    }
}

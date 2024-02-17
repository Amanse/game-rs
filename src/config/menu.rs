use std::{collections::HashMap, ops::Deref};

use dialoguer::FuzzySelect;
use eyre::Result;

use super::config2::State;

// pub struct Menu<'a, T: Display, S> {
//     options: HashMap<T, &'a dyn Fn(&mut S) -> &mut S>,
// }
//

pub struct Menu<'a, S> {
    options: HashMap<&'a str, fn(&mut S) -> &mut S>,
}

impl<'a, S> Menu<'a, S> {
    pub fn new(options: HashMap<&'a str, fn(&mut S) -> &mut S>) -> Result<Self> {
        Ok(Menu { options })
    }

    pub fn select_and_run(&self, _state: &mut State) -> Result<fn(&mut S) -> &mut S> {
        //@TODO: Find a cleaner way to collect hashmap keys into a vector/slice
        let mut items = vec![];

        for key in self.options.keys() {
            items.push(key);
        }

        let selection = FuzzySelect::new()
            .items(&items)
            .interact_opt()
            .unwrap()
            .unwrap();

        let key = items[selection].deref();

        //Return the function associated with the selected key
        return Ok(self.options[key]);
    }
}

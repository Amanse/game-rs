use dialoguer::{Confirm, Input};

pub fn string_input(prompt: &str, default: String) -> String {
    Input::new()
        .default(default)
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

pub fn bool_input(prompt: &str) -> bool {
    Confirm::new()
        .with_prompt(prompt)
        .interact_opt()
        .unwrap()
        .unwrap()
}

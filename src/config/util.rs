use dialoguer::{Confirm, Input};

pub fn string_input(prompt: &str, default: String) -> String {
    Input::new()
        .default(default.clone())
        .with_prompt(prompt)
        .with_initial_text(default)
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

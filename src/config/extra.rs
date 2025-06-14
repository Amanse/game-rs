use dialoguer::{Input, Select};
use eyre::Result;
use serde_derive::{Deserialize, Serialize};

use super::{menu::Menu, util};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ExtraConfig {
    pub prefix_dir: Option<String>,
    pub runner_dirs: Option<Vec<String>>,
}

impl ExtraConfig {
    pub fn get_runners(&self) -> Result<Vec<String>> {
        let mut runners = vec![];
        let base_path = "~/lutris/runners/wine".to_string();
        if std::path::Path::new(&base_path).exists() {
            Self::get_runners_for(base_path, &mut runners)?;
        }
        if let Some(dir) = self.runner_dirs.clone() {
            for p in dir {
                Self::get_runners_for(p, &mut runners)?;
            }
        }
        Ok(runners)
    }

    pub fn editor(&mut self) -> &mut Self {
        let mut menu = Menu::new();
        menu.add_option("Add Runner dirs", &Self::add_runners_dir);
        menu.add_option("Add Prefix Dir", &Self::add_prefix_dir);

        let a = menu.user_select();
        a(self);

        self
    }

    pub fn add_runners_dir(&mut self) -> &mut Self {
        let inp = util::string_input("Add new dir with runners", String::from(""));
        if let Some(ref mut a) = self.runner_dirs {
            a.push(inp);
        } else {
            self.runner_dirs = Some(vec![inp]);
        }
        self
    }

    pub fn add_prefix_dir(&mut self) -> &mut Self {
        let inp = util::string_input(
            "Add Prefix Directory",
            self.prefix_dir.clone().unwrap_or(String::from("")),
        );

        self.prefix_dir = Some(inp);

        self
    }

    fn get_runners_for(base_path: String, runners: &mut Vec<String>) -> Result<&mut Vec<String>> {
        if let Ok(paths) = std::fs::read_dir(base_path.clone()) {
            for path in paths {
                let p = path?.path().clone();
                if let Some(dir) = p.iter().last() {
                    if p.join("proton").exists() {
                        runners.push(format!("{}/{}", base_path.clone(), dir.to_str().unwrap()));
                    }
                }
            }
        };

        Ok(runners)
    }

    pub fn prefix_selector(&self) -> Result<String> {
        let inp = util::string_input(
            "Prefix dir",
            self.prefix_dir.clone().unwrap_or(String::from("")),
        );
        Ok(inp)
    }

    pub fn runner_selector(&self) -> Result<String> {
        let runner_list = self.get_runners()?;
        let runner_s = Select::new()
            .with_prompt("Wine Runner")
            .default(0)
            .item("Custom path")
            .item("Auto download(needs ulwgl)")
            .items(&runner_list)
            .interact()?;

        // @TODO: Remove this crap with wine-ge phaseout
        let runner_path: String = match runner_s {
            0 => Input::new()
                .with_prompt("Path to proton/wine binary")
                .interact_text()?,
            1 => "".to_string(),
            _ => runner_list[runner_s - 2].clone(),
        };

        Ok(runner_path)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
}

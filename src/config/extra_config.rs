use dialoguer::{Input, Select};
use eyre::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ExtraConfig {
    pub runner_path: Option<String>,
    pub prefix_dir: Option<String>,
    pub runner_dirs: Option<Vec<String>>,
}

impl ExtraConfig {
    pub fn new() -> Result<Self> {
        Ok(confy::load("game-rs", "Extra").unwrap_or(ExtraConfig::default()))
    }

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

    pub fn runner_selector(&self) -> Result<String> {
        let runner_path: String;
        let runner_list = self.get_runners()?;
        let runner_s = Select::new()
            .with_prompt("Wine Runner")
            .default(0)
            .item("Custom path")
            .item("Auto download(needs ulwgl)")
            .items(&runner_list)
            .interact()?;

        // @TODO: Remove this crap with wine-ge phaseout
        match runner_s {
            0 => {
                runner_path = Input::new()
                    .with_prompt("Path to proton/wine binary")
                    .default(self.runner_path.clone().unwrap_or("".to_string()))
                    .interact_text()?;
            }
            1 => {
                runner_path = "".to_string();
            }
            _ => {
                runner_path = runner_list[runner_s - 2].clone();
            }
        }

        Ok(runner_path)
    }
}

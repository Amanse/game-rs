use eyre::Result;
use dialoguer::{Input, Select};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtraConfig {
    pub runner_path: Option<String>,
    pub prefix_dir: Option<String>,
    pub runner_dirs: Option<Vec<String>>,
}

impl ::std::default::Default for ExtraConfig {
    fn default() -> Self {
        Self {
            runner_path: None,
            prefix_dir: None,
            runner_dirs: None,
        }
    }
}

impl ExtraConfig {
    pub fn new() -> Result<Self> {
        Ok(confy::load("game-rs", "Extra").unwrap_or(ExtraConfig::default()))
    }

    pub fn get_runners(&self) -> Result<Vec<String>> {
        let mut runners = vec![];
        let base_path = format!("{}/lutris/runners/wine", std::env::var("XDG_DATA_HOME")?);
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
        match std::fs::read_dir(base_path.clone()) {
            Ok(paths) => {
                for path in paths {
                    let p = path?.path().clone();
                    if let Some(dir) = p.iter().last().clone() {
                        if p.join("bin").exists() {
                            runners.push(format!(
                                "{}/{}/bin/wine",
                                base_path.clone().to_string(),
                                dir.to_str().unwrap()
                            ));
                        }
                    }
                }
            }
            Err(_) => {}
        };
        Ok(runners)
    }

    pub fn runner_selector(&self) -> Result<String> {
        let runner_path: String;
        let runner_list = self.get_runners()?;
        let runner_s = Select::new()
            .with_prompt(
                "Wine Runner [You can add runner dir to automatically fetch these in config]",
            )
            .default(0)
            .item("Custom path")
            .items(&runner_list)
            .interact()?;

        if runner_s != 0 {
            runner_path = runner_list[runner_s - 1].clone();
        } else {
            runner_path = Input::new()
                .with_prompt("Path to proton/wine binary")
                .default(self.runner_path.clone().unwrap_or("".to_string()))
                .interact_text()?;
        }
        Ok(runner_path)
    }
}
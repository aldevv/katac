use std::{fs, path::PathBuf};

use log::info;
use serde::{Deserialize, Serialize};

// default location
// ~/.config/share/katac/katac.json

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct GlobalConfigFile {
    pub repos: Option<Vec<Repo>>,
    pub workplaces: Option<Vec<Workplace>>,
}

impl GlobalConfigFile {
    pub fn new(filename: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Reading global config.json file");

        if let Some(path) = filename.parent() {
            if !path.exists() {
                Err(format!("{} does not exist", path.display()))?;
            }
        }

        let str = fs::read_to_string(filename)?;
        Ok(serde_json::from_str(&str)?)
    }

    pub fn update(&self, filename: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        info!("Updating global config.json file");

        if let Some(path) = filename.parent() {
            if !path.exists() {
                Err(format!("{} does not exist", path.display()))?;
            }
        }

        let str = serde_json::to_string_pretty(self)?;
        fs::write(filename, str)?;

        Ok(())
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub path: String,
    pub author: String,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Workplace {
    pub name: String,
    pub path: String,
    pub katas_dir: String,
    pub days_dir: String,
}

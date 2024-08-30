use std::{fs, path::PathBuf};

use log::info;
use serde::{Deserialize, Serialize};

// default location
// ~/.config/share/katac/katac.json

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct GlobalConfigFile {
    #[serde(skip)]
    pub filepath: PathBuf,

    pub repos: Option<Vec<Repo>>,
    pub workspaces: Option<Vec<Workspace>>,
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
        let mut global_config: GlobalConfigFile = serde_json::from_str(&str)?;
        global_config.filepath = filename.to_path_buf();
        Ok(global_config)
    }

    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Updating global config.json file");

        if let Some(path) = self.filepath.parent() {
            if !path.exists() {
                Err(format!("{} does not exist", path.display()))?;
            }
        }

        let str = serde_json::to_string_pretty(self)?;
        fs::write(self.filepath.clone(), str)?;

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
pub struct Workspace {
    pub name: String,
    pub path: String,
    pub katas_dir: String,
    pub days_dir: String,
}

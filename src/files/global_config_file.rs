use std::{fs, path::PathBuf};

use log::info;
use serde::{Deserialize, Serialize};

use crate::{config::global_config_filepath, Workspace};

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
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Reading global config.json file");

        let filepath = global_config_filepath();
        if let Some(path) = filepath.parent() {
            if !path.exists() {
                Err(format!("{} does not exist", path.display()))?;
            }
        }

        let str = fs::read_to_string(&filepath)?;
        let mut global_config: GlobalConfigFile = serde_json::from_str(&str)?;
        global_config.filepath = filepath.to_path_buf();
        if !filepath.exists() {
            global_config
                .update()
                .expect("Unable to update config file");
        }

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

    pub fn add_workspace(&mut self, ws: &Workspace) {
        let mut workspaces = self.workspaces.clone().unwrap_or_default();
        workspaces.push(ws.clone());

        self.workspaces = Some(workspaces);
        self.update().expect("Unable to update config file");
    }

    pub fn contains_workspace(&self, name: &str) -> bool {
        self.workspaces
            .clone()
            .unwrap_or_default()
            .iter()
            .any(|ws| ws.name == name)
    }

    pub fn load_repos(&self) -> Vec<Repo> {
        // TODO: load from repos folder
        vec![]
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub path: String,
    pub author: String,
    pub katas: Vec<String>,
}

use std::{fs, path::PathBuf};

use log::info;
use serde::{Deserialize, Serialize};

use crate::{config::global_config_path, Kata, Workspace};

// default location
// ~/.config/share/katac/katac.json

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct GlobalConfigFile {
    #[serde(skip)]
    pub path: PathBuf,

    // TODO: maybe repos are not needed, add a source field to a workspace, to see if is a remote
    // workspace!
    pub workspaces: Vec<Workspace>,
}

impl GlobalConfigFile {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Reading global config file");

        let path = global_config_path();
        if let Some(path) = path.parent() {
            if !path.exists() {
                fs::create_dir_all(path).expect("failed to create the global config folder");
            }
        };

        match fs::read_to_string(&path) {
            Ok(str) => {
                let mut global_config: GlobalConfigFile = match serde_json::from_str(&str) {
                    Ok(g) => g,
                    Err(e) => {
                        info!("Error: {:?}", e);
                        GlobalConfigFile::default()
                    }
                };
                global_config.path = path.clone();
                Ok(global_config)
            }
            Err(_) => Ok(Self {
                path,
                workspaces: vec![],
            }),
        }
    }

    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Updating global config.json file");

        if let Some(path) = self.path.parent() {
            if !path.exists() {
                fs::create_dir_all(path).expect("failed to create the global config folder");
            }
        }

        let str = serde_json::to_string_pretty(self)?;
        fs::write(self.path.clone(), str)?;

        Ok(())
    }

    pub fn add_workspace(&mut self, ws: &Workspace) {
        self.workspaces.push(ws.clone());
        self.update().expect("Unable to update config file");
    }

    pub fn contains_workspace(&self, name: &str) -> bool {
        self.workspaces.clone().iter().any(|ws| ws.name == name)
    }

    pub fn all_katas(&self) -> Vec<Kata> {
        let mut katas = vec![];
        for ws in self.workspaces.clone() {
            for kata in ws.katas.clone() {
                katas.push(kata);
            }
        }
        katas
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub path: String,
    pub author: String,
    pub katas: Vec<String>,
}

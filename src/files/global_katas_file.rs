use std::{fs, path::PathBuf};

use crate::Kata;
use log::info;
use serde::{Deserialize, Serialize};

// default location
// ~/.config/share/katac/global_katas.json

/// keeps track of all katas added
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(transparent)] // this is to make sure that the json is an array
pub struct GlobalKatasFile {
    pub katas: Option<Vec<Kata>>,
}

impl GlobalKatasFile {
    pub fn new(filename: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Reading katac.toml file");

        if let Some(path) = filename.parent() {
            if !path.exists() {
                fs::create_dir_all(path).expect("failed to create the days folder");
            }
        }

        let str = fs::read_to_string(filename)?;

        Ok(toml::from_str(&str)?)
    }

    pub fn update(
        &self,
        global_katas_file_name: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self)?;
        Ok(fs::write(global_katas_file_name, json)?)
    }
}

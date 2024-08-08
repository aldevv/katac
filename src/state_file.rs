use std::{fs, path::PathBuf};

use log::info;
use serde::{Deserialize, Serialize};

use crate::Kata;

// keeps track of all katas added
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct StateFile {
    pub random: Option<Vec<String>>,
    pub katas: Option<Vec<Kata>>,
}

impl StateFile {
    pub fn new(state_file_name: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Reading katac.toml file");

        if let Some(path) = state_file_name.parent() {
            if !path.exists() {
                fs::create_dir_all(path).expect("failed to create the days folder");
            }
        }

        let str = fs::read_to_string(state_file_name)?;

        Ok(toml::from_str(&str)?)
    }

    pub fn update(&self, state_file_name: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let toml = toml::to_string(&self)?;
        Ok(fs::write(state_file_name, toml)?)
    }
}

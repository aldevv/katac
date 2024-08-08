// keeps track of all katas added
use std::{fs, path::PathBuf};

use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub random: Option<Vec<String>>,
}

impl ConfigFile {
    pub fn new(config_file_name: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Reading katac_config.toml file");

        if let Some(path) = config_file_name.parent() {
            if !path.exists() {
                Err(format!("{} does not exist", path.display()))?;
            }
        }

        let str = fs::read_to_string(config_file_name)?;
        Ok(toml::from_str(&str)?)
    }

    pub fn update(&self, config_file_name: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let toml = toml::to_string(&self)?;
        Ok(fs::write(config_file_name, toml)?)
    }

    /// reads the katas.toml file and returns a vector of random katas
    pub fn get_random_katas_from_config(&self) -> Vec<String> {
        let mut kata_names = self.random.clone().unwrap_or_default();
        if kata_names.is_empty() {
            println!("config file's random setting is empty");
            std::process::exit(1);
        }

        kata_names.shuffle(&mut thread_rng());
        kata_names
    }
}

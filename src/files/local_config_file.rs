use std::{fs, path::PathBuf};

use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

// default location
// ./katac.json

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct LocalConfigFile {
    #[serde(skip)]
    pub filepath: PathBuf,

    pub random: Option<Vec<String>>,
}

impl LocalConfigFile {
    pub fn new(filename: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Reading katac_config.toml file");

        if let Some(path) = filename.parent() {
            if !path.exists() {
                Err(format!("{} does not exist", path.display()))?;
            }
        }

        let str = fs::read_to_string(filename)?;
        let mut local_config: LocalConfigFile = serde_json::from_str(&str)?;
        local_config.filepath = filename.to_path_buf();
        Ok(local_config)
    }

    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self)?;
        Ok(fs::write(self.filepath.clone(), json)?)
    }

    /// reads the katas.json file and returns a vector of random katas
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

use std::{fs, path::PathBuf};

use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
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

        let str = fs::read_to_string(state_file_name)?;

        Ok(toml::from_str(&str)?)
    }

    pub fn update(&self, state_file_name: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let toml = toml::to_string(&self)?;
        Ok(fs::write(state_file_name, toml)?)
    }
}

/// reads the katas.toml file and returns a vector of random katas
//TODO: fix these functions so that they use a local config file
pub fn read_random_katas_from_config_file(config_file: &PathBuf) -> Vec<String> {
    let tom = StateFile::new(config_file).unwrap_or_default();
    let mut kata_names = tom.random.unwrap_or_default();

    kata_names.shuffle(&mut thread_rng());
    if kata_names.is_empty() {
        println!("config file is empty");
        std::process::exit(1);
    }
    kata_names
}

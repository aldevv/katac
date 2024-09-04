use std::{fs, path::PathBuf};

use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{args::Args, config::local_config_path, Result};

// default location
// ./katac.json

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct LocalConfigFile {
    #[serde(skip)]
    pub path: PathBuf,

    pub random: Vec<String>,
}

impl LocalConfigFile {
    pub fn new(args: &Args) -> Option<Self> {
        info!("Reading local config file");

        let path = local_config_path(args);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("failed to create the global config folder");
            }
        };

        if !path.exists() {
            eprintln!(
                "given config file \"{}\" does not exist. Ignoring...",
                path.display()
            );
            return None;
        }

        match fs::read_to_string(&path) {
            Ok(str) => {
                let mut local_config: LocalConfigFile = serde_json::from_str(&str)
                    .map_err(|e| {
                        eprintln!("Unable to parse local config file: {}", e);
                        std::process::exit(1);
                    })
                    .unwrap();
                local_config.path = path.clone();
                if !path.exists() {
                    local_config.update().expect("Unable to update config file");
                }
                Some(local_config)
            }
            Err(_) => None,
        }
    }

    pub fn update(&self) -> Result<()> {
        let json = serde_json::to_string(&self)?;
        Ok(fs::write(self.path.clone(), json)?)
    }

    /// reads the katas.json file and returns a vector of random katas
    pub fn get_random_katas_from_config(&self) -> Vec<String> {
        let mut kata_names = self.random.clone();
        if kata_names.is_empty() {
            println!("config file's random setting is empty");
            std::process::exit(1);
        }

        kata_names.shuffle(&mut thread_rng());
        kata_names
    }
}

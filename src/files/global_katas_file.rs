use std::{fs, path::PathBuf};

use crate::{config::global_katas_filepath, Kata};
use log::info;
use serde::{Deserialize, Serialize};

// default location
// ~/.config/share/katac/global_katas.json

/// keeps track of all katas added
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(transparent)] // this is to make sure that the json is an array
pub struct GlobalKatasFile {
    #[serde(skip)]
    pub filepath: PathBuf,

    pub katas: Option<Vec<Kata>>,
}

impl GlobalKatasFile {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Reading katac.toml file");

        let filepath = global_katas_filepath();
        if let Some(path) = filepath.parent() {
            if !path.exists() {
                fs::create_dir_all(path).expect("failed to create the days folder");
            }
        }

        let str = fs::read_to_string(&filepath)?;

        let mut global_katas_file: GlobalKatasFile = serde_json::from_str(&str)?;
        global_katas_file.filepath = filepath.to_path_buf();
        if !filepath.exists() {
            global_katas_file
                .update()
                .expect("Unable to update config file");
        }
        Ok(global_katas_file)
    }

    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self)?;
        Ok(fs::write(self.filepath.clone(), json)?)
    }

    // save in global katas file
    pub fn save(&mut self, kata_name: &str, kata_path: PathBuf) {
        let mut katas = self.katas.clone().unwrap_or_default();
        katas.push(Kata {
            name: kata_name.to_string(),
            path: kata_path,
        });

        let new_global_katas_file = GlobalKatasFile {
            katas: Some(katas),
            ..self.clone()
        };
        new_global_katas_file
            .update()
            .expect("failed to update global katas file");

        *self = new_global_katas_file;
    }

    // is saved in global katas file
    pub fn is_saved(&self, kata: &str) -> bool {
        self.katas
            .clone()
            .unwrap_or_default()
            .iter()
            .any(|k| k.name == kata)
    }

    pub fn global_katas(&self) -> Vec<Kata> {
        self.katas
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|k| Kata {
                name: k.name.clone(),
                path: k.path.clone(),
            })
            .collect()
    }
}

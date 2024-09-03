use log::info;

use crate::args::Args;
use crate::files::global_config_file::GlobalConfigFile;
use crate::files::local_config_file::LocalConfigFile;
use crate::workspaces::Workspace;
use crate::Kata;
use std::path::PathBuf;

pub const DEF_KATAS_DIR: &str = "katas";
pub const DEF_DAYS_DIR: &str = "days";
pub const DEF_GLOBAL_KATAS_FILENAME: &str = "global_katas.json";
pub const DEF_CONFIG_FILENAME: &str = "katac.json";

pub struct Config {
    pub global_config_file: GlobalConfigFile,
    pub local_config_file: LocalConfigFile,
}

impl Config {
    pub fn new(args: &Args) -> Self {
        let local_config_file = LocalConfigFile::new(args).unwrap_or_default();
        // let global_katas_file = GlobalKatasFile::new().unwrap_or_default();
        let global_config_file = GlobalConfigFile::new().unwrap_or_default();
        info!("Global config: {:?}", global_config_file);
        Self {
            // global_katas_file,
            local_config_file,
            global_config_file,
        }
    }
}

impl Config {
    pub fn is_new_workspace(&self, workspace: &str) -> bool {
        !self.global_config_file.contains_workspace(workspace)
    }

    pub fn add_workspace(&mut self, workspace: &Workspace) {
        self.global_config_file.add_workspace(workspace);
    }

    pub fn list_workspaces(&self) {
        self.global_config_file.list_workspaces();
    }

    pub fn remove_workspace(&mut self, name: &str) {
        self.global_config_file.remove_workspace(name);
    }

    pub fn find_workspace(&self, name: &str) -> Option<Workspace> {
        self.global_config_file.find_workspace(name)
    }
}

pub fn share_dir() -> String {
    if cfg!(windows) {
        std::env::var("USERPROFILE").unwrap() + "/katac" // TODO: check this dst
    } else {
        std::env::var("HOME").unwrap() + "/.local/share/katac"
    }
}

pub fn local_config_path(args: &Args) -> PathBuf {
    PathBuf::from(
        args.config_file
            .clone()
            .unwrap_or_else(|| DEF_CONFIG_FILENAME.to_string()),
    )
}

pub fn global_katas_filepath() -> PathBuf {
    PathBuf::from(share_dir() + "/" + DEF_GLOBAL_KATAS_FILENAME)
}

pub fn global_config_path() -> PathBuf {
    PathBuf::from(share_dir() + "/" + DEF_CONFIG_FILENAME)
}

/// returns a vector of katas from the katas folder and the config file
pub fn merge_local_and_global_katas(local_katas: Vec<Kata>, state_katas: Vec<Kata>) -> Vec<Kata> {
    // merge the two vectors
    let mut katas: Vec<Kata> = local_katas.clone();
    for kata in state_katas.iter() {
        if !local_katas.iter().any(|k| k.name == kata.name) {
            katas.push(kata.clone());
        }
    }
    katas
}

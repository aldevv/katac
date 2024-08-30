use crate::args::Args;
use crate::files::global_config_file::GlobalConfigFile;
use crate::files::global_katas_file::GlobalKatasFile;
use crate::files::local_config_file::LocalConfigFile;
use crate::workspaces::Workspace;
use crate::Kata;
use std::path::PathBuf;

pub const DEF_KATAS_DIR: &str = "katas";
pub const DEF_DAYS_DIR: &str = "days";
pub const DEF_GLOBAL_KATAS_FILENAME: &str = "global_katas.json";
pub const DEF_CONFIG_FILENAME: &str = "katac.json";

pub struct Config {
    pub global_katas_file: GlobalKatasFile,
    pub global_config_file: GlobalConfigFile,
    pub local_config_file: LocalConfigFile,
}

impl Config {
    pub fn new(args: &Args) -> Self {
        let local_config_file = LocalConfigFile::new(args).unwrap_or_default();
        let global_katas_file = GlobalKatasFile::new().unwrap_or_default();
        let global_config_file = GlobalConfigFile::new().unwrap_or_default();
        Self {
            global_katas_file,
            local_config_file,
            global_config_file,
        }
    }
}

impl Config {
    pub fn is_saved_in_globals(&self, kata: &str) -> bool {
        self.global_katas_file.is_saved(kata)
    }

    pub fn save_in_globals(&mut self, kata: Kata) {
        self.global_katas_file.save(&kata.name, kata.path);
    }

    pub fn is_new_workspace(&self, workspace: &str) -> bool {
        !self.global_config_file.contains_workspace(workspace)
    }

    pub fn add_workspace(&mut self, workspace: &Workspace) {
        self.global_config_file.add_workspace(workspace);
    }

    pub fn global_katas(&self) -> Vec<Kata> {
        self.global_katas_file.global_katas()
    }
}

pub fn share_dir() -> String {
    if cfg!(windows) {
        std::env::var("USERPROFILE").unwrap() + "/katac" // TODO: check this dst
    } else {
        std::env::var("HOME").unwrap() + "/.local/share/katac"
    }
}

pub fn local_config_filepath(args: &Args) -> PathBuf {
    PathBuf::from(
        args.config_file
            .clone()
            .unwrap_or_else(|| DEF_CONFIG_FILENAME.to_string()),
    )
}

pub fn global_katas_filepath() -> PathBuf {
    PathBuf::from(share_dir() + "/" + DEF_GLOBAL_KATAS_FILENAME)
}

pub fn global_config_filepath() -> PathBuf {
    PathBuf::from(share_dir() + DEF_CONFIG_FILENAME)
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

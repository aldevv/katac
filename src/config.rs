use crate::args::Args;
use crate::files::global_config_file::GlobalConfigFile;
use crate::files::local_config_file::LocalConfigFile;
use crate::workspaces::Workspace;
use std::path::PathBuf;

pub const DEF_KATAS_DIR: &str = "katas";
pub const DEF_DAYS_DIR: &str = "days";
pub const DEF_CONFIG_FILENAME: &str = "katac.json";

pub struct Config {
    pub global_config_file: GlobalConfigFile,
    pub local_config_file: Option<LocalConfigFile>,
}

impl Config {
    pub fn new(args: &Args) -> Self {
        let local_config_file = LocalConfigFile::new(args);
        let global_config_file = GlobalConfigFile::new().unwrap_or_default();
        Self {
            local_config_file,
            global_config_file,
        }
    }
}

impl Config {
    pub fn is_new_workspace(&self, name: &str) -> bool {
        !self.global_config_file.contains_workspace(name)
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

    pub fn update(&self) {
        self.global_config_file
            .update()
            .expect("Unable to update global config file");
    }

    pub fn update_cfg_if_given_args(&mut self, args: &Args, workspace: &mut Workspace) {
        let mut changed = false;
        if let Some(katas_dir) = &args.katas_dir {
            workspace.katas_dir = PathBuf::from(katas_dir);
            changed = true;
        }

        if let Some(days_dir) = &args.days_dir {
            workspace.days_dir = PathBuf::from(days_dir);
            changed = true;
        }

        if changed {
            self.global_config_file.update_workspace(workspace);
        }
    }
}

pub fn share_dir() -> String {
    if cfg!(windows) {
        std::env::var("USERPROFILE").unwrap() + "/katac"
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

pub fn global_config_path() -> PathBuf {
    PathBuf::from(share_dir() + "/" + DEF_CONFIG_FILENAME)
}

pub mod args;
mod commands;
mod config;
mod files;
mod workspaces;

use args::Args;
use config::Config;
use fs_extra::dir::CopyOptions;
use inquire::{InquireError, MultiSelect};
use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use workspaces::Workspace;

use crate::commands::{
    create_command, create_makefile, create_os_run_file, make_is_installed, run_custom_command,
    run_using_makefile, USE_MAKEFILE,
};
use crate::workspaces::get_kata_path;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
pub type Error = Box<dyn std::error::Error>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Kata {
    /// the name of the kata
    pub name: String,
    /// the relative path
    pub path: PathBuf,
}

pub struct Katac {
    pub args: Args,
    /// current working directory
    pub workspace: Workspace,
    /// apps configuration
    pub cfg: Config,
    /// all katas, both globally and local to the workspace
    pub all_katas: Vec<Kata>,
}

impl Katac {
    pub fn new(args: &Args) -> Self {
        let mut cfg = Config::new(args);
        let mut workspace = Workspace::new(args);
        if cfg.is_new_workspace(&workspace.name) {
            cfg.add_workspace(&workspace);
        } else {
            workspace = cfg.find_workspace(&workspace.name).unwrap();
            cfg.update_cfg_if_given_args(args, &mut workspace);
        }

        let all_katas = cfg.all_katas(&workspace);
        Self {
            args: args.clone(),
            workspace,
            cfg,
            all_katas,
        }
    }

    pub fn open_prompt(&self) -> Vec<String> {
        let options: Vec<&str> = self.all_katas.iter().map(|k| k.name.as_str()).collect();
        MultiSelect::new("Choose the katas you want:", options)
            .prompt()
            .unwrap_or_else(|err| match err {
                InquireError::OperationInterrupted => {
                    println!("Interrupted");
                    std::process::exit(1);
                }
                InquireError::NotTTY => {
                    println!("Not a TTY");
                    std::process::exit(1);
                }
                _ => {
                    std::process::exit(1);
                }
            })
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn select(&mut self) {
        if self.args.kata_names_args.is_some() {
            let kata_names = self.args.kata_names_args.clone().unwrap();
            // TODO: fix this so that it receives Katas instead of just kata_names
            self.copy_katas(&kata_names);
            return;
        }

        let chosen_katas: Vec<String> = self.open_prompt();

        if chosen_katas.is_empty() {
            println!("No katas selected, use the SPACE key to select katas");
            std::process::exit(1);
        }

        for kata in chosen_katas.clone().into_iter() {
            if !self.workspace.contains(&kata) {
                println!(
                    "Kata {} not found in {}",
                    kata,
                    get_kata_path(&kata, self.workspace.katas_dir.clone()).display()
                );
                std::process::exit(1);
            }
        }
        self.copy_katas(&chosen_katas);
    }

    pub fn find_kata_path(&self, kata_name: &str) -> PathBuf {
        info!("Finding kata path for {}", kata_name);
        self.all_katas
            .iter()
            .find(|k| k.name == kata_name)
            .unwrap_or_else(|| {
                println!("Kata \"{}\" not found", kata_name);
                std::process::exit(1);
            })
            .path
            .clone()
    }

    /// copies katas from the katas_dir to a new day in days_dir
    pub fn copy_katas(&mut self, kata_names: &Vec<String>) {
        let dst = self.workspace.nextday_path();
        info!(
            "Copying {} to {}",
            kata_names.join(", "),
            dst.clone().display()
        );
        for kata_name in kata_names {
            let src = self.find_kata_path(kata_name);
            if !src.exists() {
                println!("Kata {} does not exist", kata_name);
                std::process::exit(1);
            }
            if !dst.exists() {
                fs::create_dir_all(&dst).expect("failed to create the days folder");
            }
            match fs_extra::copy_items(&[src.clone()], dst.clone(), &CopyOptions::new()) {
                Ok(_) => println!("Copying {} to {}...", kata_name, basename(&dst)),
                Err(e) => println!("Error: {}", e),
            }

            create_command(dst.clone().join(kata_name))
        }
    }

    /// runs the katas in the current day
    pub fn run(&self, kata_names: Option<Vec<String>>, command: Option<String>) {
        let kata_names = kata_names.unwrap_or_else(|| self.workspace.curday_katas());

        for (i, kata_name) in kata_names.iter().enumerate() {
            let curday_kata_path = self.workspace.curday_kata_path(kata_name);
            let run_str = format!("\n> Running {} [{}/{}]", kata_name, i + 1, kata_names.len());
            println!("{}\n{}", run_str, "-".repeat(run_str.len()));

            let child = if let Some(command) = &command {
                run_custom_command(command, curday_kata_path)
            } else {
                run_using_makefile(curday_kata_path)
            };
            if let Some(mut c) = child {
                c.wait().expect("failed to wait on child process");
            }
        }
    }

    /// returns a vector of random katas from the katas.toml file or the katas folder
    pub fn get_random_katas(&self, num_katas_wanted: u8) -> Vec<String> {
        let random_katas = if let Some(local_config_file) = &self.cfg.local_config_file {
            local_config_file.get_random_katas_from_config()
        } else {
            info!("no katac.json config found, reading local katas folder for random katas");
            let mut local_katas = self.workspace.get_kata_names();
            local_katas.shuffle(&mut thread_rng());
            local_katas
        };

        if num_katas_wanted > random_katas.len() as u8 {
            println!("random katas wanted number is higher than the number of katas found");
            std::process::exit(1);
        }
        random_katas[0..num_katas_wanted as usize].to_vec()
    }

    /// adds a new kata in the kata_dir folder or the given path
    pub fn add(&mut self, kata_name: String) {
        let kata = self.workspace.add(&kata_name);

        if USE_MAKEFILE && make_is_installed() {
            create_makefile(kata.path);
            return;
        }

        create_os_run_file(kata.path);
    }

    pub fn random_katas(&mut self, num_katas_wanted: u8) {
        self.copy_katas(&self.get_random_katas(num_katas_wanted));
    }

    pub fn add_workspace(
        &mut self,
        name: Option<String>,
        path: Option<String>,
        remote: Option<String>,
    ) {
        let path = if let Some(path) = path {
            PathBuf::from(path)
        } else {
            std::env::current_dir().unwrap()
        };

        let name = if let Some(name) = name {
            name
        } else {
            path.file_name().unwrap().to_str().unwrap().to_string()
        };

        let mut workspace = Workspace::new_with(&self.args, &name, path);
        if let Some(remote) = remote {
            workspace.clone_from_remote(&remote);
        }
        self.cfg.add_workspace(&workspace);
    }

    pub fn list_workspaces(&self) {
        self.cfg.list_workspaces();
    }

    pub fn remove_workspace(&mut self, name: &str) {
        self.cfg.remove_workspace(name);
    }

    pub fn list_katas(&self, workspace_name: Option<String>) {
        let workspace = if let Some(name) = workspace_name {
            self.cfg.find_workspace(&name).expect("Workspace not found")
        } else {
            self.workspace.clone()
        };

        for kata in workspace.katas.clone() {
            println!("{}", kata.name);
        }
    }

    pub fn list_all_katas(&self) {
        for kata in self.all_katas.clone() {
            println!("{}", kata.name);
        }
    }
}

/// returns the basename of a path
fn basename(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

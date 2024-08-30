pub mod args;
pub mod commands;
pub mod config;
pub mod files;
pub mod workspaces;

use args::Args;
use config::{merge_local_and_global_katas, Config};
use fs_extra::dir::CopyOptions;
use inquire::{InquireError, MultiSelect};
use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use workspaces::Workspace;

use crate::commands::{
    create_makefile, create_os_run_file, make_is_installed, run_custom_command, run_using_makefile,
    USE_MAKEFILE,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Kata {
    pub name: String,
    pub path: PathBuf,
    // TODO: add workplace field so that new folders are clean, katac repos (shows list, then
    // copies, add an option to clean state file for a workplace, workplace NEEDS to be added
    // manually, katac add workspace <name> <path> to add a new workplace)
    // this way even if I'm in the wrong folder it will always copy to the correct place,
    // katac practice is organized, having more than one workplace is weird so having it as
    // a command that is manual is good, do a katac init to create a new workspace? sounds good
    // have a global state for workspaces, and the one currently selected (how about a .katac
    // folder?, is cool because it will be local to the project, and we won't need to worry about
    // being in the wrong folder (just look for parents until you find a .katac folder)
    // pub workplace: Vec<String>, // maybe not needed if we use a .katac folder in the project root

    // chose to use workplaces instead of .katac folder in each project
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
        let workspace = Workspace::new(args);

        let mut cfg = Config::new(args);
        // TODO: check this logic, it's not working as expected
        if !cfg.is_new_workspace(&workspace.name) {
            info!("Workspace {} found in global config file", workspace.name);
            cfg.add_workspace(&workspace);
        }

        let global_katas = cfg.global_katas();
        let all_katas = merge_local_and_global_katas(workspace.katas.clone(), global_katas.clone());
        Self {
            args: args.clone(),
            cfg,
            all_katas,
            workspace,
        }
    }

    pub fn open_prompt(&self) -> Vec<String> {
        let options: Vec<&str> = self.all_katas.iter().map(|k| k.name.as_str()).collect();
        MultiSelect::new("Choose the katas you want:", options)
            .prompt()
            .unwrap_or_else(|err| match err {
                InquireError::OperationInterrupted => {
                    println!("User interrupted");
                    std::process::exit(1);
                }
                InquireError::NotTTY => {
                    println!("Not a TTY");
                    std::process::exit(1);
                }
                _ => {
                    println!("Unknown error");
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
            self.copy_katas(&kata_names);
            return;
        }

        let chosen_katas: Vec<String> = self.open_prompt();

        if chosen_katas.is_empty() {
            println!("No katas selected, use the SPACE key to select katas");
            std::process::exit(1);
        }

        for kata in chosen_katas.clone().into_iter() {
            if !self.workspace.local_kata_path(&kata).exists() {
                println!(
                    "Kata {} not found in {}",
                    kata,
                    self.workspace.local_kata_path(&kata).display()
                );
                std::process::exit(1);
            }
        }
        self.copy_katas(&chosen_katas);
    }

    pub fn find_kata_path(&self, kata_name: &str) -> PathBuf {
        self.all_katas
            .iter()
            .find(|k| k.name == kata_name)
            .unwrap()
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

            if !self.cfg.is_saved_in_globals(kata_name) {
                self.cfg.save_in_globals(Kata {
                    name: kata_name.clone(),
                    path: self.workspace.kata_absolute_path(kata_name),
                });
            }
        }
    }

    /// runs the katas in the current day
    pub fn run(&self, kata_names: Option<Vec<String>>, command: Option<String>) {
        let kata_names = kata_names.unwrap_or_else(|| self.workspace.curday_katas());

        for (i, kata_name) in kata_names.iter().enumerate() {
            let curday_kata_path = self.workspace.curday_kata_path(kata_name);
            let run_str = format!("\n> Running {} [{}/{}]", kata_name, i + 1, kata_names.len());
            println!("{}\n{}", run_str, "-".repeat(run_str.len()));

            let mut child = if let Some(command) = &command {
                run_custom_command(command, curday_kata_path).expect("failed to run the kata")
            } else {
                run_using_makefile(curday_kata_path).expect("failed to run the kata")
            };
            child.wait().expect("failed to wait on child");
        }
    }

    /// returns a vector of random katas from the katas.toml file or the katas folder
    pub fn get_random_katas(&self, num_katas_wanted: u8) -> Vec<String> {
        let random_katas = if self.cfg.local_config_file.random.is_some() {
            self.cfg.local_config_file.get_random_katas_from_config()
        } else {
            info!("no katac_config.toml found, reading katas folder for random katas");
            let mut local_katas: Vec<String> = self
                .workspace
                .katas
                .iter()
                .map(|k| k.name.clone())
                .collect();
            local_katas.shuffle(&mut thread_rng());
            local_katas
        };

        if num_katas_wanted > random_katas.len() as u8 {
            println!("random katas wanted number is higher than the number of katas found");
            std::process::exit(1);
        }
        random_katas[0..num_katas_wanted as usize].to_vec()
    }
    /// creates a new kata in the kata_dir folder or the given path
    pub fn create(&self, kata_name: String) {
        let kata_path = self.workspace.local_kata_path(&kata_name);
        if kata_path.exists() {
            println!("Kata {} already exists", kata_name);
            std::process::exit(1);
        }
        fs::create_dir_all(&kata_path).expect("failed to create the kata folder");
        println!("{} created in {}.", kata_name, dirname(&kata_path));

        if USE_MAKEFILE && make_is_installed() {
            create_makefile(kata_path);
            return;
        }

        create_os_run_file(kata_path);
    }

    pub fn random_katas(&mut self, num_katas_wanted: u8) {
        self.copy_katas(&self.get_random_katas(num_katas_wanted));
    }
}

/// returns the basename of a path
fn basename(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

/// returns the dirname of a path
fn dirname(path: &Path) -> String {
    path.parent().unwrap().to_str().unwrap().to_string()
}

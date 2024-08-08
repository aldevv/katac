pub mod args;
pub mod config;
pub mod state_file;

use crate::config::Config;
use crate::state_file::read_random_katas_from_config_file;
use args::Args;
use fs_extra::dir::CopyOptions;
use inquire::{InquireError, MultiSelect};
use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const USE_MAKEFILE: bool = true;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Kata {
    pub name: String,
    pub path: PathBuf,
}

pub struct Katac {
    pub cfg: Config,
    pub katas: Vec<Kata>,
}

impl Katac {
    pub fn new(args: &Args) -> Self {
        let cfg = Config::new(args);
        let katas = cfg.katas();
        Self { cfg, katas }
    }

    pub fn choose(&self) -> Vec<String> {
        let options: Vec<&str> = self.katas.iter().map(|k| k.name.as_str()).collect();
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

    pub fn save_and_copy_prompt(&mut self) {
        let chosen: Vec<String> = self.choose();

        if chosen.is_empty() {
            println!("No katas selected, use the SPACE key to select katas");
            std::process::exit(1);
        }

        for a in chosen.clone().into_iter() {
            if !self.cfg.local_kata_path(&a).exists() {
                println!(
                    "Kata {} not found in {}",
                    a,
                    self.cfg.local_kata_path(&a).display()
                );
                std::process::exit(1);
            }

            if !self.cfg.is_saved(&a) {
                self.cfg.save(&a)
            }
        }

        self.copy_katas(&chosen);
    }

    pub fn kata_path(&self, kata_name: &str) -> PathBuf {
        self.katas
            .iter()
            .find(|k| k.name == kata_name)
            .unwrap()
            .path
            .clone()
    }

    /// copies katas from the katas_dir to a new day in days_dir
    pub fn copy_katas(&self, kata_names: &Vec<String>) {
        let dst = self.cfg.nextday_path();
        info!(
            "Copying {} to {}",
            kata_names.join(", "),
            dst.clone().display()
        );
        for kata_name in kata_names {
            let src = self.kata_path(kata_name);
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
        }
    }

    /// runs the katas in the current day
    pub fn run_katas(&self, kata_names: Option<Vec<String>>, command: Option<String>) {
        let kata_names = kata_names.unwrap_or_else(|| self.cfg.curday_katas());

        for (i, kata_name) in kata_names.iter().enumerate() {
            let curday_kata_path = self.cfg.curday_kata_path(kata_name);
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

    pub fn random_katas(&self, number_of_katas: u8) -> Vec<String> {
        let mut kata_names: Vec<String>;
        if std::path::Path::new(&self.cfg.config_file_path).exists() {
            kata_names = read_random_katas_from_config_file(&self.cfg.config_file_path);
            if number_of_katas > kata_names.len() as u8 {
                println!(
                    "random number is higher than the number of katas found in the katas.toml file"
                );
                std::process::exit(1);
            }
        } else {
            info!("no katas.toml found, reading katas folder for random katas");
            // kata_names becomes all files inside the katas folder
            kata_names = self.katas.iter().map(|k| k.name.clone()).collect();
            kata_names.shuffle(&mut thread_rng())
        }
        kata_names[0..number_of_katas as usize].to_vec()
    }
    /// creates a new kata in the kata_dir folder or the given path
    pub fn new_kata(&self, kata_name: String) {
        let kata_path = self.cfg.local_kata_path(&kata_name);
        if kata_path.exists() {
            println!("Kata {} already exists", kata_name);
            std::process::exit(1);
        }
        fs::create_dir_all(&kata_path).expect("failed to create the kata folder");
        println!("{} created in {}.", kata_name, dirname(&kata_path));

        if USE_MAKEFILE
            && Command::new("make")
                .arg("--version")
                .stdout(std::process::Stdio::null())
                .status()
                .is_ok()
        {
            create_makefile(kata_path);
            return;
        }

        create_os_run_file(kata_path);
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

fn run_custom_command(command: &str, kata_path: PathBuf) -> Option<std::process::Child> {
    let mut command = command.split_whitespace();
    Some(
        Command::new(command.next().unwrap())
            .args(command)
            .current_dir(kata_path)
            .spawn()
            .expect("failed to run the kata"),
    )
}

/// runs the kata in the given path
fn run_using_makefile(curday_kata_path: PathBuf) -> Option<std::process::Child> {
    if USE_MAKEFILE
        && Command::new("make")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .status()
            .is_ok()
    {
        return run_make_command(curday_kata_path);
    }
    run_os_command(curday_kata_path)
}

/// runs the kata in the given path using the make command
fn run_make_command(mut path: PathBuf) -> Option<std::process::Child> {
    let path_str = path
        .to_str()
        .expect("failed to convert path to string")
        .to_string();

    path.push("Makefile");
    if !path.exists() {
        println!("No Makefile found in {}", path_str);
        return None;
    }

    Some(
        Command::new("make")
            .arg("run")
            .arg("-s")
            .current_dir(path_str)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to run the kata"),
    )
}

/// runs the kata in the given path using an OS specific file (run.sh or run.bat)
fn run_os_command(run_path: PathBuf) -> Option<std::process::Child> {
    let run_path_str = run_path
        .to_str()
        .expect("failed to convert path to string")
        .to_string();

    if cfg!(target_os = "windows") {
        let bat_file = run_path.join("run.bat");
        if !bat_file.exists() {
            println!("No run.bat file found in {}", run_path_str);
            return None;
        }

        return Some(
            Command::new("cmd")
                .arg("/C")
                .arg(format!("cd {} && run.bat", run_path_str))
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("failed to run the kata"),
        );
    }

    let sh_file = run_path.join("run.sh");
    if !sh_file.exists() {
        println!("No run.sh file found in {}", run_path_str);
        return None;
    }

    return Some(
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && ./run.sh", run_path_str))
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to run the kata"),
    );
}

/// returns a vector of random katas from the katas.toml file or the katas folder
// TODO: fix random katas

/// creates a new Makefile in the given path
fn create_makefile(mut path: PathBuf) {
    let content = "run:\n\t@echo \"TODO: add your run command here\"";
    path.push("Makefile");
    let mut f = fs::File::create(path).expect("failed to create the Makefile");
    f.write_all(content.as_bytes())
        .expect("failed to write to the Makefile");
}

/// creates a new run.sh or run.bat file in the given path
fn create_os_run_file(mut kata_path: PathBuf) {
    if cfg!(target_os = "windows") {
        let content = "TODO: add your run command here";
        kata_path.push("run.bat");
        let mut f =
            std::fs::File::create(kata_path).expect("failed to create the windows run file");
        f.write_all(content.as_bytes())
            .expect("failed to write to the windows run.bat file");
        return;
    }

    let content = "#!/usr/bin/env bash\n\n# TODO: replace this line with  your run command (example: npm run test)";
    kata_path.push("run.sh");
    let mut f = std::fs::File::create(&kata_path).expect("failed to create the linux run file");

    f.write_all(content.as_bytes())
        .expect("failed to write to the linux run.sh file");

    #[cfg(unix)]
    {
        use std::os::unix::prelude::PermissionsExt;
        std::fs::set_permissions(kata_path, fs::Permissions::from_mode(0o755))
            .expect("failed to set permissions on the linux run file");
    }
}

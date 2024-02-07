use clap::{Parser, Subcommand};
use fs_extra::dir::CopyOptions;
use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const KATAS_DIR: &str = "katas";
const DAYS_DIR: &str = "days";
const CONFIG_FILE_NAME: &str = "katac.toml";

const USE_MAKEFILE: bool = true;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
/// Katac is a tool to help you do katas everyday
pub struct Args {
    /// Custom directory to copy katas from (default: ./katas)
    #[arg(short, long)]
    pub katas_dir: Option<String>,

    /// Custom directory to copy katas to everyday (default: ./days)
    #[arg(short, long)]
    pub days_dir: Option<String>,

    /// Custom config file (default: ./katac.toml)
    #[arg(short, long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub subcommand: Option<Subcommands>,

    /// Katas you want to do today
    #[arg(num_args = 1..)]
    pub kata_names: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Katas you want to run today (requires a makefile with the  'run' target in the kata's root folder)
    Run {
        /// Katas to run
        #[arg(required = false, num_args = 1..)]
        kata_names: Option<Vec<String>>,

        /// Run custom command for given kata
        #[arg(short, long)]
        command: Option<String>,
    },

    /// Number of katas you want to do today, randomly taken from katas.toml
    Random {
        /// Katas to run
        #[arg(required = true, num_args = 1..)]
        number_of_katas: u8,
    },

    /// Create a new kata
    New {
        /// Name of the kata you want to create
        #[arg(required = true, num_args = 1..)]
        kata_name: String,
    },
}

#[derive(Deserialize, Debug)]
struct Data {
    katas: Katas,
}

/// config file structure
#[derive(Deserialize, Debug)]
struct Katas {
    random: Option<Vec<String>>,
    katas_dir: Option<String>,
    days_dir: Option<String>,
}

// returns the current day number
fn curday(days_dir: &str) -> u32 {
    match fs::read_dir(days_dir) {
        Err(_) => 0,
        Ok(dir) => dir
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .map(|e| e.trim_start_matches("day").to_string())
            .filter_map(|e| e.parse::<u32>().ok())
            .max()
            .unwrap(),
    }
}

/// priorities are:
/// --katas-dir arg
/// KATAS_DIR env var
/// katas_dir config file property
/// default value
fn katas_dir(args: &Args) -> String {
    match args.katas_dir.clone() {
        Some(katas_dir) => katas_dir,
        None => match std::env::var("KATAS_DIR") {
            Ok(getenv) => getenv,
            Err(_) => {
                let config_file_name = match args.config.clone() {
                    Some(config_file) => config_file,
                    None => CONFIG_FILE_NAME.to_string(),
                };
                match read_config_file(config_file_name).katas.katas_dir {
                    Some(katas_dir) => katas_dir,
                    None => KATAS_DIR.to_string(),
                }
            }
        },
    }
}

/// priorities are:
/// --days-dir arg
/// DAYS_DIR env var
/// days_dir config file property
/// default value
fn days_dir(args: &Args) -> String {
    match args.days_dir.clone() {
        Some(days_dir) => days_dir,
        None => match std::env::var("DAYS_DIR") {
            Ok(getenv) => getenv,
            Err(_) => {
                let config_file_name = match args.config.clone() {
                    Some(config_file) => config_file,
                    None => CONFIG_FILE_NAME.to_string(),
                };
                match read_config_file(config_file_name).katas.days_dir {
                    Some(days_dir) => days_dir,
                    None => DAYS_DIR.to_string(),
                }
            }
        },
    }
}

/// copies katas from the katas_dir to a new day in days_dir
pub fn copy_katas(args: &Args, kata_names: &Vec<String>) {
    let dst = nextday_path(&days_dir(args));
    for kata_name in kata_names {
        let src = kata_path(kata_name, &katas_dir(args));
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

/// returns the basename of a path
fn basename(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

/// returns the dirname of a path
fn dirname(path: &Path) -> String {
    path.parent().unwrap().to_str().unwrap().to_string()
}

/// runs the katas in the current day
pub fn run_katas(args: &Args, kata_names: Option<Vec<String>>, command: Option<String>) {
    let days_dir = days_dir(args);
    let curday_path = curday_path(&days_dir);

    let kata_names = match kata_names {
        Some(kata_names) => kata_names,
        None => curday_katas(curday_path),
    };

    for (i, kata_name) in kata_names.iter().enumerate() {
        let curday_kata_path = curday_kata_path(&days_dir, kata_name);
        let run_str = format!("\n> Running {} [{}/{}]", kata_name, i + 1, kata_names.len());
        println!("{}", run_str);
        let width = run_str.chars().count();
        println!("{}", "-".repeat(width));

        if let Some(command) = command.clone() {
            let mut command = command.split_whitespace();
            let mut child = Command::new(command.next().unwrap())
                .args(command)
                .current_dir(curday_kata_path)
                .spawn()
                .expect("failed to run the kata");
            child.wait().expect("failed to wait on child");
            continue;
        }

        match run(curday_kata_path) {
            Some(mut child) => child.wait().expect("failed to wait on child"),
            None => continue,
        };
    }
}

/// runs the kata in the given path
fn run(curday_kata_path: PathBuf) -> Option<std::process::Child> {
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
pub fn random_katas(args: &Args, number_of_katas: u8) -> Vec<String> {
    let config_file = match args.config.clone() {
        Some(config_file) => config_file,
        None => CONFIG_FILE_NAME.to_string(),
    };

    let mut kata_names: Vec<String>;
    if std::path::Path::new(&config_file).exists() {
        kata_names = read_random_katas_from_config_file(config_file);
        if number_of_katas > kata_names.len() as u8 {
            println!(
                "random number is higher than the number of katas found in the katas.toml file"
            );
            std::process::exit(1);
        }
    } else {
        info!("no katas.toml found, reading katas folder for random katas");
        // kata_names becomes all files inside the katas folder
        kata_names = katas(&katas_dir(args));
        kata_names.shuffle(&mut thread_rng())
    }
    kata_names[0..number_of_katas as usize].to_vec()
}

/// creates a new kata in the kata_dir folder or the given path
pub fn new_kata(args: &Args, kata_name: String) {
    let kata_dir = &katas_dir(args);
    let kata_path = kata_path(&kata_name, kata_dir);
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

/// returns a vector of katas from the katas folder
fn katas(katas_dir: &String) -> Vec<String> {
    fs::read_dir(katas_dir)
        .expect("Unable to read katas folder")
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

/// returns a vector of katas from the current day folder
fn curday_katas(curday_path: PathBuf) -> Vec<String> {
    fs::read_dir(curday_path)
        .expect("Unable to read current day contents")
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

/// returns the path of the given kata
fn kata_path(kata_name: &str, katas_dir: &String) -> PathBuf {
    if kata_name.contains('/') {
        return PathBuf::from(kata_name.to_string());
    }
    PathBuf::from(format!("{}/{}", katas_dir, kata_name))
}

/// returns the path of the current day
fn curday_path(days_dir: &String) -> PathBuf {
    PathBuf::from(format!("{}/day{}", days_dir, curday(days_dir)))
}

/// returns the path of the next day
fn nextday_path(days_dir: &str) -> PathBuf {
    PathBuf::from(format!("{}/day{}", days_dir, curday(days_dir) + 1))
}

/// returns the path of the given kata in the current day
fn curday_kata_path(days_dir: &String, kata_name: &String) -> PathBuf {
    curday_path(days_dir).join(kata_name)
}

/// reads the katas.toml file and returns a Data struct
fn read_config_file(config_file_name: String) -> Data {
    info!("Reading katas.toml file");

    let str =
        fs::read_to_string(config_file_name).expect("Something went wrong reading the config file");
    toml::from_str(&str).expect("Something went wrong reading the config file")
}

/// reads the katas.toml file and returns a vector of random katas
fn read_random_katas_from_config_file(config_file: String) -> Vec<String> {
    let tom = read_config_file(config_file);

    let mut kata_names = tom.katas.random.unwrap_or_default();
    kata_names.shuffle(&mut thread_rng());
    if kata_names.is_empty() {
        println!("config file is empty");
        std::process::exit(1);
    }
    kata_names
}

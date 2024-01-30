use clap::{Parser, Subcommand};
use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::io::Write;
use std::process::Command;
use std::{fs, path::PathBuf};

const KATAS_DIR: &str = "katas";
const DAYS_DIR: &str = "days";
const CONFIG_FILE: &str = "katac.toml";

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

fn curday(days_dir: &String) -> u32 {
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

pub fn copy_katas(args: &Args, kata_names: &Vec<String>) {
    let katas_dir = katas_dir(args.katas_dir.clone());
    let days_dir = days_dir(args.days_dir.clone());

    create_day(&days_dir);
    let copy_options = fs_extra::dir::CopyOptions::new();
    for kata_name in kata_names {
        let src = PathBuf::from(&kata_path(kata_name, &katas_dir));
        let dst = PathBuf::from(curday_path(&days_dir));
        match fs_extra::copy_items(&[src], dst, &copy_options) {
            Ok(_) => println!("Copying {} to day{}...", kata_name, curday(&days_dir)),
            Err(e) => println!("Error: {}", e),
        }
    }
}

pub fn run_katas(args: &Args, kata_names: Option<Vec<String>>) {
    let days_dir = days_dir(args.days_dir.clone());
    let curday_path = curday_path(&days_dir);

    let kata_names = match kata_names {
        Some(kata_names) => kata_names,
        None => read_curday(&curday_path),
    };

    for (i, kata_name) in kata_names.iter().enumerate() {
        let curday_kata_path = curday_kata_path(&days_dir, kata_name);
        let makefile_path = format!("{}/Makefile", curday_kata_path);
        let run_str = format!("\n> Running {} [{}/{}]", kata_name, i + 1, kata_names.len());
        let width = run_str.chars().count();

        println!("{}", run_str);
        println!("{}", "-".repeat(width));

        if !std::path::Path::new(&makefile_path).exists() {
            println!("No Makefile found in {}", curday_kata_path);
            continue;
        }

        match run(kata_name.to_string(), curday_kata_path) {
            Some(mut child) => child.wait().expect("failed to wait on child"),
            None => continue,
        };
    }
}

fn run(kata_name: String, curday_kata_path: String) -> Option<std::process::Child> {
    if Command::new("make")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .status()
        .is_ok()
    {
        return run_make_command(kata_name.to_string(), curday_kata_path);
    }
    run_os_command(kata_name.to_string(), curday_kata_path)
}

fn run_make_command(kata_name: String, path: String) -> Option<std::process::Child> {
    info!(
        "Running {}, in {}",
        kata_name,
        curday_path_short(path.clone()),
    );

    let makefile_path = format!("{}/Makefile", path);
    if !std::path::Path::new(&makefile_path).exists() {
        println!("No Makefile found in {}", path);
        return None;
    }

    Some(
        Command::new("make")
            .arg("run")
            .arg("-s")
            .current_dir(path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to run the kata"),
    )
}

fn run_os_command(kata_name: String, curday_kata_path: String) -> Option<std::process::Child> {
    info!(
        "Running {}, in {}",
        kata_name,
        curday_path_short(curday_kata_path.clone()),
    );

    if cfg!(target_os = "windows") {
        let bat_file_path = format!("{}/run.bat", curday_kata_path);
        if !std::path::Path::new(&bat_file_path).exists() {
            println!("No run.bat file found in {}", curday_kata_path);
            return None;
        }

        return Some(
            Command::new("cmd")
                .arg("/C")
                .arg(format!("cd {} && run.bat", curday_kata_path))
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("failed to run the kata"),
        );
    }

    let sh_file_path = format!("{}/run.sh", curday_kata_path);
    if !std::path::Path::new(&sh_file_path).exists() {
        println!("No run.sh file found in {}", curday_kata_path);
        return None;
    }

    return Some(
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && run.sh", curday_kata_path))
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to run the kata"),
    );
}

pub fn random_katas(args: &Args, number_of_katas: u8) -> Vec<String> {
    let config_file = match args.config.clone() {
        Some(config_file) => config_file,
        None => CONFIG_FILE.to_string(),
    };

    let mut kata_names: Vec<String>;
    if std::path::Path::new(&config_file).exists() {
        kata_names = read_config_file(config_file);
        if number_of_katas > kata_names.len() as u8 {
            println!(
                "random number is higher than the number of katas found in the katas.toml file"
            );
            std::process::exit(1);
        }
    } else {
        info!("no katas.toml found, reading katas folder for random katas");
        // kata_names becomes all files inside the katas folder
        kata_names = read_katas_dir(&katas_dir(args.katas_dir.clone()));
        kata_names.shuffle(&mut thread_rng())
    }
    kata_names[0..number_of_katas as usize].to_vec()
}

// creates a new kata in the kata_dir folder
pub fn new_kata(args: &Args, kata_name: String) {
    let kata_path = kata_path(&kata_name, &katas_dir(args.katas_dir.clone()));
    if std::path::Path::new(&kata_path).exists() {
        println!("Kata {} already exists", kata_name);
        std::process::exit(1);
    }
    std::fs::create_dir_all(&kata_path).expect("failed to create the kata folder");

    if Command::new("make")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .status()
        .is_ok()
    {
        create_makefile(&kata_path);
        return;
    }

    create_os_run_file(&kata_path);
}

fn create_makefile(kata_path: &String) {
    let content = "run:\n\t@echo \"TODO: add your run command here\"";
    let mut f = std::fs::File::create(format!("{}/Makefile", kata_path))
        .expect("failed to create the Makefile");
    f.write_all(content.as_bytes())
        .expect("failed to write to the Makefile");
}

fn create_os_run_file(kata_path: &String) {
    if cfg!(target_os = "windows") {
        let content = "TODO: add your run command here";
        let mut f = std::fs::File::create(format!("{}/run.bat", kata_path))
            .expect("failed to create the windows run file");
        f.write_all(content.as_bytes())
            .expect("failed to write to the windows run.bat file");
        return;
    }

    let content = "#!/usr/bin/env bash\n\n# TODO: replace this line with  your run command (example: npm run test)";
    let mut f = std::fs::File::create(format!("{}/run.sh", kata_path))
        .expect("failed to create the linux run file");

    f.write_all(content.as_bytes())
        .expect("failed to write to the linux run.sh file");

    #[cfg(unix)]
    {
        use std::os::unix::prelude::PermissionsExt;
        std::fs::set_permissions(
            format!("{}/run.sh", kata_path),
            fs::Permissions::from_mode(0o755),
        )
        .expect("failed to set permissions on the linux run file");
    }
}

fn read_katas_dir(katas_dir: &String) -> Vec<String> {
    std::fs::read_dir(katas_dir)
        .expect("Unable to read katas folder")
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

fn read_curday(curday_path: &String) -> Vec<String> {
    std::fs::read_dir(curday_path)
        .expect("Unable to read current day contents")
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

fn create_day(days_dir: &String) {
    let day_num = curday(days_dir);
    let path = format!("{}/day{}", days_dir, day_num + 1);
    fs::create_dir_all(path).expect("failed to create the day folder");
}

fn katas_dir(katas_dir: Option<String>) -> String {
    if let Some(katas_dir) = katas_dir {
        return katas_dir;
    }

    if let Ok(getenv) = std::env::var("KATAS_DIR") {
        return getenv;
    }
    KATAS_DIR.to_string()
}

fn days_dir(days_dir: Option<String>) -> String {
    if let Some(days_dir) = days_dir {
        return days_dir;
    }

    if let Ok(getenv) = std::env::var("DAYS_DIR") {
        return getenv;
    }
    DAYS_DIR.to_string()
}

fn kata_path(kata_name: &str, katas_dir: &String) -> String {
    format!("{}/{}", katas_dir, kata_name)
}

fn curday_path(days_dir: &String) -> String {
    let day = curday(days_dir);
    if day == 0 {
        println!("No kata to run was found, start a day first");
        std::process::exit(1);
    }
    format!("{}/day{}", days_dir, day)
}

fn curday_path_short(path: String) -> String {
    return path
        .split('/')
        .collect::<Vec<&str>>()
        .iter()
        .rev()
        .take(3)
        .rev()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("/");
}

fn curday_kata_path(days_dir: &String, kata_name: &String) -> String {
    format!("{}/{}", curday_path(days_dir), kata_name)
}

// Top level struct to hold the TOML data.
#[derive(Deserialize, Debug)]
struct Data {
    katas: Katas,
}

#[derive(Deserialize, Debug)]
struct Katas {
    random: Vec<String>,
}

fn read_config_file(config_file: String) -> Vec<String> {
    info!("Reading katas.toml file");

    let str =
        fs::read_to_string(config_file).expect("Something went wrong reading the config file");
    let tom: Data = toml::from_str(&str).expect("Something went wrong reading the config file");

    let mut kata_names = tom.katas.random;
    kata_names.shuffle(&mut thread_rng());
    if kata_names.is_empty() {
        println!("config file is empty");
        std::process::exit(1);
    }
    kata_names
}

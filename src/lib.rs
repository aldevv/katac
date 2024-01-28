use clap::{Parser, Subcommand};
use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::process::Command;
use std::{fs, path::PathBuf};
use toml;

const CONFIG_FILE: &str = "katac.toml";
const KATAS_DIR: &str = "katas";
const DAYS_DIR: &str = "days";

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
        #[arg(required = true, num_args = 1..)]
        kata_names: Vec<String>,
    },

    /// Number of katas you want to do today, randomly taken from katas.toml
    Random {
        /// Katas to run
        #[arg(required = true, num_args = 1..)]
        number_of_katas: u8,
    },
}

pub fn get_curday(days_dir: &String) -> u32 {
    match fs::read_dir(days_dir) {
        Err(_) => 0,
        Ok(dir) => dir
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .map(|e| e.trim_start_matches("day").to_string())
            .filter_map(|e| e.parse::<u32>().ok())
            .max()
            .unwrap_or(0),
    }
}

pub fn copy_katas(args: &Args, kata_names: &Vec<String>) {
    let katas_dir = katas_dir(args.katas_dir.clone());
    let days_dir = days_dir(args.days_dir.clone());

    create_day(&days_dir);
    let copy_options = fs_extra::dir::CopyOptions::new();
    for kata_name in kata_names {
        let src = src(kata_name, &katas_dir);
        let dst = dst(&days_dir);
        match fs_extra::copy_items(&[src], dst, &copy_options) {
            Ok(_) => println!("Copying {} to day{}...", kata_name, get_curday(&days_dir)),
            Err(e) => println!("Error: {}", e),
        }
    }
}

pub fn run_katas(args: &Args, kata_names: &Vec<String>) {
    for (i, kata_name) in kata_names.iter().enumerate() {
        let days_dir = days_dir(args.days_dir.clone());
        let path = format!("{}/{}", dst_path(&days_dir), &kata_name);
        let makefile_path = format!("{}/Makefile", path);
        if !std::path::Path::new(&makefile_path).exists() {
            println!("No Makefile found in {}", path);
            continue;
        }

        println!(
            "\n> Running {} [{}/{}]\n_______________________",
            kata_name,
            i + 1,
            kata_names.len()
        );
        let mut child = run_make_command(kata_name.to_string(), path);
        let code = child.wait().expect("failed to wait on child");
        assert!(code.success());
    }
}

pub fn random_katas(args: &Args, number_of_katas: u8) -> Vec<String> {
    let mut kata_names: Vec<String>;

    // let config_file = args.config.clone().unwrap_or(CONFIG_FILE.to_string());

    let config_file = match args.config.clone() {
        Some(config_file) => config_file,
        None => CONFIG_FILE.to_string(),
    };

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
        let katas_dir = katas_dir(args.katas_dir.clone());
        kata_names = std::fs::read_dir(katas_dir)
            .expect("Unable to read katas folder")
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();
        kata_names.shuffle(&mut thread_rng())
    }
    return kata_names[0..number_of_katas as usize].to_vec();
}

fn create_day(days_dir: &String) {
    let day_num = get_curday(&days_dir);
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
    return KATAS_DIR.to_string();
}

fn days_dir(days_dir: Option<String>) -> String {
    if let Some(days_dir) = days_dir {
        return days_dir;
    }

    if let Ok(getenv) = std::env::var("DAYS_DIR") {
        return getenv;
    }
    return DAYS_DIR.to_string();
}

fn src_filepath(kata_name: &str, katas_dir: &String) -> String {
    return format!("{}/{}", katas_dir, kata_name);
}

fn src(kata_name: &str, katas_dir: &String) -> PathBuf {
    return PathBuf::from(src_filepath(kata_name, katas_dir));
}

fn dst_path(days_dir: &String) -> String {
    let day = get_curday(&days_dir);
    if day == 0 {
        println!("No kata to run was found, start a day first");
        std::process::exit(1);
    }
    return format!("{}/day{}", days_dir, day);
}

fn dst(days_dir: &String) -> PathBuf {
    let dst_path = dst_path(&days_dir);
    return PathBuf::from(dst_path);
}

pub fn dst_short_filepath(path: String) -> String {
    return path
        .split("/")
        .collect::<Vec<&str>>()
        .iter()
        .rev()
        .take(3)
        .rev()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("/");
}

fn run_make_command(kata_name: String, path: String) -> std::process::Child {
    info!(
        "Running {}, in {}",
        kata_name,
        dst_short_filepath(path.clone()),
    );
    Command::new("make")
        .arg("run")
        .arg("-s")
        .current_dir(path)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("failed to run the kata")
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
        fs::read_to_string(config_file).expect("Something went wrong reading the katas.toml file");
    let tom: Data = toml::from_str(&str).expect("Something went wrong reading the katas.toml file");

    let mut kata_names = tom.katas.random;
    kata_names.shuffle(&mut thread_rng());
    if kata_names.len() == 0 {
        println!("katas.toml is empty");
        std::process::exit(1);
    }
    return kata_names;
}

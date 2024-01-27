use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::process::Command;
use std::{fs, path::PathBuf};
use toml;

pub mod args;

pub fn get_curday(day_folder_name: &String) -> u32 {
    // check if folder is empty
    match fs::read_dir(day_folder_name.clone()) {
        Err(_) => {
            fs::create_dir(day_folder_name.clone()).unwrap();
        }
        Ok(_) => {}
    }

    fs::read_dir(day_folder_name)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .map(|e| e.trim_start_matches("day").to_string())
        .filter_map(|e| e.parse::<u32>().ok())
        .max()
        .unwrap_or(0)
}

// Create a new day folder
pub fn create_day(days_dir: Option<String>) {
    let day_folder_name = days_dir.unwrap_or(get_days_dir());
    let day_num = get_curday(&day_folder_name);
    let path = format!("{}/day{}", day_folder_name, day_num + 1);
    fs::create_dir(path).expect("failed to create the day folder");
}

pub fn get_katas_dir() -> String {
    if let Ok(getenv) = std::env::var("KATAS_DIR") {
        return getenv;
    }

    let mut dir = std::env::current_dir().unwrap();
    dir.push("katas");
    dir.to_str().unwrap().to_string()
}

pub fn get_days_dir() -> String {
    if let Ok(getenv) = std::env::var("DAYS_DIR") {
        return getenv;
    }

    let mut dir = std::env::current_dir().unwrap();
    dir.push("days");
    dir.to_str().unwrap().to_string()
}

pub fn get_src_path(kata_name: &str, katas_dir: Option<String>) -> String {
    let katas_dir = katas_dir.unwrap_or(get_katas_dir());
    return format!("{}/{}", katas_dir, kata_name);
}

pub fn get_src(kata_name: &str, katas_dir: Option<String>) -> PathBuf {
    let src_path = get_src_path(kata_name, katas_dir);
    return PathBuf::from(src_path);
}

pub fn get_short_path(path: String) -> String {
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

pub fn get_dst_path(days_dir: Option<String>) -> String {
    let days_dir = days_dir.unwrap_or(get_days_dir());
    let day = get_curday(&days_dir);
    if day == 0 {
        println!("No kata to run was found, start a day first");
        std::process::exit(1);
    }
    return format!("{}/day{}", days_dir, day);
}

pub fn get_dst(days_dir: Option<String>) -> PathBuf {
    let dst_path = get_dst_path(days_dir);
    return PathBuf::from(dst_path);
}

pub fn run_make_command(kata_name: String, path: String) -> std::process::Child {
    info!("Running {}, in {}", kata_name, get_short_path(path.clone()),);
    Command::new("make")
        .arg("run")
        .arg("-s")
        .current_dir(path.clone())
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

pub fn read_katas_file(random: Option<u8>) -> Vec<String> {
    info!("Reading katas.toml file");
    let random = random.expect("random is not a valid number");

    let str =
        fs::read_to_string("katas.toml").expect("Something went wrong reading the katas.toml file");
    let tom: Data = toml::from_str(&str).expect("Something went wrong reading the katas.toml file");

    let mut kata_names = tom.katas.random;
    kata_names.shuffle(&mut thread_rng());
    if random > kata_names.len() as u8 {
        println!("random number is higher than the number of katas in the katas.toml file");
        return Vec::new();
    }
    return kata_names;
}

pub fn get_random_katas(random: Option<u8>, katas_dir: Option<String>) -> Vec<String> {
    let mut kata_names: Vec<String>;

    if std::path::Path::new("katas.toml").exists() {
        kata_names = read_katas_file(random)
    } else {
        info!("no katas.toml found, reading katas folder for random katas");
        // kata_names becomes all files inside the katas folder
        kata_names = std::fs::read_dir(katas_dir.clone().unwrap_or(get_katas_dir()))
            .expect("Unable to read katas folder")
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();
        kata_names.shuffle(&mut thread_rng())
    }
    return kata_names[0..random.unwrap() as usize].to_vec();
}

pub fn copy_kata(kata_names: Vec<String>, katas_dir: Option<String>, days_dir: Option<String>) {
    let copy_options = fs_extra::dir::CopyOptions::new();
    for kata_name in &kata_names {
        let src = get_src(kata_name, katas_dir.clone());
        let dst = get_dst(days_dir.clone());
        match fs_extra::copy_items(&[src], dst, &copy_options) {
            Ok(_) => println!(
                "Copying {} to day{}...",
                kata_name,
                get_curday(&days_dir.clone().unwrap_or(get_days_dir()))
            ),
            Err(e) => println!("Error: {}", e),
        }
    }
}

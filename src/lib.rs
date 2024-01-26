use log::info;
use std::process::Command;
use std::{fs, path::PathBuf};

pub fn get_curday() -> u32 {
    let day_folder_name = "days";

    // check if folder is empty
    match fs::read_dir(day_folder_name) {
        Err(_) => {
            fs::create_dir(day_folder_name).unwrap();
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
pub fn create_day() {
    let day_folder_name = "days";
    let day_num = get_curday();
    let path = format!("{}/day{}", day_folder_name, day_num + 1);
    fs::create_dir(path).unwrap();
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
    return format!("{}/day{}", days_dir, get_curday());
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

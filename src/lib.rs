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

pub fn get_src(kata_name: &str, katas_dir: Option<String>) -> PathBuf {
    let katas_dir = katas_dir.unwrap_or(get_katas_dir());
    let kata_path = format!("{}/{}", katas_dir, kata_name);
    return PathBuf::from(kata_path);
}

pub fn get_dst(days_dir: Option<String>) -> PathBuf {
    let days_dir = days_dir.unwrap_or(get_days_dir());
    let day_path = format!("{}/day{}", days_dir, get_curday());
    return PathBuf::from(day_path);
}

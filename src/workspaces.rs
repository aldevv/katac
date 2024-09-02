use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    args::Args,
    config::{DEF_DAYS_DIR, DEF_KATAS_DIR},
    Kata,
};

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Workspace {
    /// workspace name
    pub name: String,
    /// workspace fullpath
    pub path: PathBuf,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub remote: String,
    pub author: String,
    pub katas_dir: PathBuf,
    pub days_dir: PathBuf,
    pub katas: Vec<Kata>,
}

impl Workspace {
    pub fn new(args: &Args) -> Self {
        let path = std::env::current_dir().unwrap();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let katas_dir = katas_dir(args);
        let days_dir = days_dir(args);
        let remote = "";
        let author = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());

        let mut ws = Self {
            name,
            path,
            remote: remote.to_string(),
            author,
            katas_dir,
            days_dir,
            katas: vec![],
        };
        ws.katas = ws.get_katas();
        ws
    }

    pub fn get_katas(&self) -> Vec<Kata> {
        fs::read_dir(self.katas_dir.clone())
            .expect("Unable to read katas folder")
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .map(|e| Kata {
                name: e.clone(),
                path: get_kata_path(&e, self.katas_dir.clone()),
            })
            .collect()
    }

    pub fn contains(&self, kata_name: &str) -> bool {
        self.katas.iter().any(|k| k.name == kata_name)
    }

    pub fn new_kata(&self, kata_name: &str) -> Kata {
        let kata_path = get_kata_path(kata_name, self.katas_dir.clone());
        if kata_path.exists() {
            println!("Kata already exists");
            std::process::exit(1);
        }
        fs::create_dir_all(&kata_path).expect("Unable to create kata folder");
        println!("{} created in {}.", kata_name, dirname(&kata_path));
        Kata {
            name: kata_name.to_string(),
            path: kata_path,
        }
    }

    pub fn curday(&self) -> u32 {
        match fs::read_dir(self.days_dir.clone()) {
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

    /// returns the path of the current day
    pub fn curday_path(&self) -> PathBuf {
        PathBuf::from(format!("{}/day{}", self.days_dir.display(), self.curday()))
    }

    /// returns the path of the next day
    pub fn nextday_path(&self) -> PathBuf {
        PathBuf::from(format!(
            "{}/day{}",
            self.days_dir.display(),
            self.curday() + 1
        ))
    }

    /// returns the path of the given kata in the current day
    pub fn curday_kata_path(&self, kata_name: &String) -> PathBuf {
        self.curday_path().join(kata_name)
    }

    /// returns a vector of katas from the current day folder
    pub fn curday_katas(&self) -> Vec<String> {
        fs::read_dir(self.curday_path())
            .expect("Unable to read current day contents")
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect()
    }
}

/// priorities are:
/// --katas-dir arg
/// KATAS_DIR env var
/// katas_dir config file property
/// default value
pub fn katas_dir(args: &Args) -> PathBuf {
    PathBuf::from(
        args.katas_dir
            .clone()
            .or_else(|| std::env::var("KATAS_DIR").ok())
            .unwrap_or_else(|| DEF_KATAS_DIR.to_string()),
    )
}

/// priorities are:
/// --days-dir arg
/// DAYS_DIR env var
/// days_dir config file property
/// default value
pub fn days_dir(args: &Args) -> PathBuf {
    PathBuf::from(
        args.days_dir
            .clone()
            .or_else(|| std::env::var("DAYS_DIR").ok())
            .unwrap_or_else(|| DEF_DAYS_DIR.to_string()),
    )
}

/// returns the path of the kata in the katas folder
pub fn get_kata_path(kata_name: &str, katas_dir: PathBuf) -> PathBuf {
    if kata_name.contains('/') {
        return PathBuf::from(kata_name.to_string());
    }
    PathBuf::from(format!("{}/{}", katas_dir.display(), kata_name))
}

/// returns the dirname of a path
fn dirname(path: &Path) -> String {
    path.parent().unwrap().to_str().unwrap().to_string()
}

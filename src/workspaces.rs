use path_clean::PathClean;
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::{
    args::Args,
    commands::create_command,
    config::{DEF_DAYS_DIR, DEF_KATAS_DIR},
    Kata,
};

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Workspace {
    /// workspace name
    pub name: String,
    /// workspace fullpath
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
    pub author: String,
    pub katas_dir: PathBuf,
    pub days_dir: PathBuf,
    pub katas: Vec<Kata>,
}

impl Workspace {
    pub fn new(args: &Args) -> Self {
        let path = std::env::current_dir().expect("Unable to get current dir");
        let name = workspace_name(path.clone());
        // if arg not given, and env var not set, use default, UNLESS the workspace exists in
        // global config file
        let katas_dir = katas_dir(args);
        let days_dir = days_dir(args);
        let remote = None;
        let author = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());

        let mut ws = Self {
            name,
            path,
            remote,
            author,
            katas_dir,
            days_dir,
            katas: vec![],
        };
        ws.katas = ws.get_katas();
        ws
    }

    pub fn new_with(args: &Args, name: &str, path: PathBuf) -> Self {
        let katas_dir = katas_dir(args);
        let days_dir = days_dir(args);
        let remote = None;
        let author = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());

        let mut ws = Self {
            name: name.to_string(),
            path,
            remote,
            author,
            katas_dir,
            days_dir,
            katas: vec![],
        };
        ws.katas = ws.get_katas();
        ws
    }

    pub fn get_katas(&self) -> Vec<Kata> {
        if !self.katas_dir.exists() {
            return vec![];
        }

        fs::read_dir(self.katas_dir.clone())
            .map_err(|e| {
                println!("Unable to read katas folder: {}", e);
                std::process::exit(1);
            })
            .unwrap()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .map(|e| Kata {
                name: e.clone(),
                path: get_kata_path(&e, self.katas_dir.clone()),
            })
            .collect()
    }

    pub fn get_kata_names(&self) -> Vec<String> {
        self.katas.iter().map(|k| k.name.clone()).collect()
    }

    pub fn contains(&self, kata_name: &str) -> bool {
        self.katas.iter().any(|k| k.name == kata_name)
    }

    /// add a new kata to the workspace
    pub fn add(&mut self, kata_name: &str) -> Kata {
        let kata_path = get_kata_path(kata_name, self.katas_dir.clone());
        if kata_path.exists() {
            println!("Kata {} already exists", kata_name);
            std::process::exit(1);
        }
        fs::create_dir_all(&kata_path).expect("Unable to create kata folder");
        println!(
            "{} created in {}.",
            kata_name,
            dirname(&self.name, &kata_path)
        );
        let kata = Kata {
            name: kata_name.to_string(),
            path: kata_path,
        };
        self.katas.push(kata.clone());

        create_command(kata.path.clone());
        kata
    }

    pub fn clone_from_remote(&mut self, remote: &str) {
        println!("Cloning from remote: {}", remote);
        let output = std::process::Command::new("git")
            .arg("clone")
            .arg(remote)
            .arg(self.path.display().to_string())
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }

        self.remote = Some(remote.to_string());
    }

    pub fn curday(&self) -> u32 {
        if !self.days_dir.exists() {
            fs::create_dir_all(&self.days_dir).expect("Unable to create days folder");
        }
        match fs::read_dir(self.days_dir.clone()) {
            Err(_) => 0,
            Ok(dir) => dir
                .filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().into_string().ok())
                .map(|e| e.trim_start_matches("day").to_string())
                .filter_map(|e| e.parse::<u32>().ok())
                .max()
                .unwrap_or_default(),
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
        let curday_path = self.curday_path();
        if !curday_path.exists() {
            eprintln!("no day has been created for the {} workspace", self.name);
            std::process::exit(1);
        }
        fs::read_dir(self.curday_path())
            .expect("Unable to read current day contents")
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect()
    }
}

pub fn workspace_name(path: PathBuf) -> String {
    path.file_name()
        .expect("Unable to get current dir name")
        .to_str()
        .expect("Unable to convert to string")
        .to_string()
}

// TODO: only create the kata and days folder IF they don't exist AND
// the workspace is not in global file

/// priorities are:
/// --days-dir arg
/// DAYS_DIR env var
/// days_dir config file property
/// default value
pub fn days_dir(args: &Args) -> PathBuf {
    let path = PathBuf::from(
        args.days_dir
            .clone()
            .or_else(|| std::env::var("DAYS_DIR").ok())
            .unwrap_or_else(|| DEF_DAYS_DIR.to_string()),
    );

    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .expect("Unable to get current dir")
            .join(path)
    }
    .clean()
}

/// priorities are:
/// --katas-dir arg
/// KATAS_DIR env var
/// katas_dir config file property
/// default value
pub fn katas_dir(args: &Args) -> PathBuf {
    let path = PathBuf::from(
        args.katas_dir
            .clone()
            .or_else(|| std::env::var("KATAS_DIR").ok())
            .unwrap_or_else(|| DEF_KATAS_DIR.to_string()),
    );
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .expect("Unable to get current dir")
            .join(path)
    }
    .clean()
}

/// returns the path of the kata in the katas folder
pub fn get_kata_path(kata_name: &str, katas_dir: PathBuf) -> PathBuf {
    if kata_name.contains('/') {
        return PathBuf::from(kata_name.to_string());
    }
    PathBuf::from(format!("{}/{}", katas_dir.display(), kata_name))
}

pub fn workspace_relative_path(workspace_name: &str, path: PathBuf) -> PathBuf {
    let prefix = path
        .components()
        .position(|c| c.as_os_str() == workspace_name)
        .expect("Unable to find workspace name in kata dir");

    path.iter().skip(prefix + 1).collect::<PathBuf>()
}

/// returns the dirname of a path
fn dirname(workspace_name: &str, kata_dir: &Path) -> String {
    // find workspace_name in kata_dir and strip_prefix from there
    let kata_dir_relative = workspace_relative_path(workspace_name, kata_dir.to_path_buf());
    kata_dir_relative
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

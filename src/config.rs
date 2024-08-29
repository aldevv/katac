use crate::args::Args;
use crate::config_file::ConfigFile;
use crate::state_file::StateFile;
use crate::Kata;
use std::fs;
use std::path::PathBuf;

pub const DEF_KATAS_DIR: &str = "katas";
pub const DEF_DAYS_DIR: &str = "days";
pub const DEF_STATE_FILE_NAME: &str = "katac_state.toml";
pub const DEF_CONFIG_FILE_NAME: &str = "katac_config.toml";

pub struct Config {
    pub args: Args,
    pub katas_dir: String,
    pub days_dir: String,
    pub state_file: StateFile,
    pub config_file: ConfigFile,
    pub state_file_path: PathBuf,
    pub config_file_path: PathBuf,
}

impl Config {
    pub fn new(args: &Args) -> Self {
        let katas_dir = Config::katas_dir(args);
        let days_dir = Config::days_dir(args);

        let config_file_path = config_file_path(args);
        let config_file = ConfigFile::new(&config_file_path).unwrap_or_default();
        if !config_file_path.exists() {
            config_file
                .update(&config_file_path)
                .expect("Unable to update config file");
        }

        let state_file_path = state_file_path();
        let state_file = StateFile::new(&state_file_path).unwrap_or_default();
        if !state_file_path.exists() {
            state_file
                .update(&state_file_path)
                .expect("Unable to update config file");
        }

        Self {
            args: args.clone(),
            katas_dir,
            days_dir,
            state_file,
            state_file_path,
            config_file_path,
            config_file,
        }
    }

    // save in state file
    pub fn save(&mut self, kata_name: &str) {
        let mut katas = self.state_file.katas.clone().unwrap_or_default();
        katas.push(Kata {
            name: kata_name.to_string(),
            path: self.kata_absolute_path(kata_name),
        });

        let new_state_file = StateFile {
            katas: Some(katas),
            ..self.state_file.clone()
        };

        new_state_file
            .update(&self.state_file_path)
            .expect("Unable to update config file");
        self.state_file = new_state_file;
    }

    // is saved in state
    pub fn is_saved(&self, kata: &str) -> bool {
        self.state_file
            .katas
            .clone()
            .unwrap_or_default()
            .iter()
            .any(|k| k.name == kata)
    }

    pub fn local_katas(&self) -> Vec<Kata> {
        fs::read_dir(self.katas_dir.clone())
            .expect("Unable to read katas folder")
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .map(|e| Kata {
                name: e.clone(),
                path: self.local_kata_path(&e),
            })
            .collect()
    }

    pub fn global_katas(&self) -> Vec<Kata> {
        self.state_file
            .katas
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|k| Kata {
                name: k.name.clone(),
                path: k.path.clone(),
            })
            .collect()
    }

    /// priorities are:
    /// --katas-dir arg
    /// KATAS_DIR env var
    /// katas_dir config file property
    /// default value
    fn katas_dir(args: &Args) -> String {
        args.katas_dir
            .clone()
            .or_else(|| std::env::var("KATAS_DIR").ok())
            .unwrap_or_else(|| DEF_KATAS_DIR.to_string())
    }

    /// priorities are:
    /// --days-dir arg
    /// DAYS_DIR env var
    /// days_dir config file property
    /// default value
    fn days_dir(args: &Args) -> String {
        args.days_dir
            .clone()
            .or_else(|| std::env::var("DAYS_DIR").ok())
            .unwrap_or_else(|| DEF_DAYS_DIR.to_string())
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
        PathBuf::from(format!("{}/day{}", self.days_dir, self.curday()))
    }

    /// returns the path of the next day
    pub fn nextday_path(&self) -> PathBuf {
        PathBuf::from(format!("{}/day{}", self.days_dir, self.curday() + 1))
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

    /// returns the path of the given kata
    pub fn local_kata_path(&self, kata_name: &str) -> PathBuf {
        if kata_name.contains('/') {
            return PathBuf::from(kata_name.to_string());
        }
        PathBuf::from(format!("{}/{}", self.katas_dir, kata_name))
    }

    pub fn kata_absolute_path(&self, kata_name: &str) -> PathBuf {
        self.local_kata_path(kata_name).canonicalize().unwrap()
    }
}
pub fn config_file_path(args: &Args) -> PathBuf {
    PathBuf::from(
        args.config_file
            .clone()
            .unwrap_or_else(|| DEF_CONFIG_FILE_NAME.to_string()),
    )
}

pub fn state_file_path() -> PathBuf {
    let path = if cfg!(windows) {
        std::env::var("USERPROFILE").unwrap() + "/katac" // TODO: check this dst
    } else {
        std::env::var("HOME").unwrap() + "/.local/share/katac"
    };

    PathBuf::from(path + "/" + DEF_STATE_FILE_NAME)
}

/// returns a vector of katas from the katas folder and the config file
pub fn merge_local_and_global_katas(local_katas: Vec<Kata>, state_katas: Vec<Kata>) -> Vec<Kata> {
    // merge the two vectors
    let mut katas: Vec<Kata> = local_katas.clone();
    for kata in state_katas.iter() {
        if !local_katas.iter().any(|k| k.name == kata.name) {
            katas.push(kata.clone());
        }
    }
    katas
}

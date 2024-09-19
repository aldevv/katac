use std::path::PathBuf;

pub const DEF_CONFIG_FILENAME: &str = "katac.json";

pub fn share_dir() -> String {
    if cfg!(windows) {
        std::env::var("USERPROFILE").unwrap() + "/katac"
    } else {
        std::env::var("HOME").unwrap() + "/.local/share/katac"
    }
}

pub fn global_config_path() -> PathBuf {
    PathBuf::from(share_dir() + "/" + DEF_CONFIG_FILENAME)
}

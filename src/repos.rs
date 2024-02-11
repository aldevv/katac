// git clone repo

use std::env::consts;

use log::info;

const DEFAULT_REPO: &str = "https://github.com/aldevv/katac-repos";

fn username_as_repo_name(repo_url: &str) -> String {
    let mut user: String = "".to_string();
    if repo_url.starts_with("git@") {
        let rest = repo_url.split(':').collect::<Vec<&str>>()[1].to_string();
        return rest.split('/').collect::<Vec<&str>>()[0].to_string();
    }

    if repo_url.starts_with("https://") || repo_url.starts_with("http://") {
        let rest = repo_url.split('/').collect::<Vec<&str>>()[3].to_string();
        return rest.split('/').collect::<Vec<&str>>()[0].to_string();
    }
    user
}

pub fn clone_repo() {
    let mut dst = if consts::OS == "windows" {
        std::env::var("USERPROFILE").unwrap() + "katac-repos" // TODO: check this dst
    } else {
        std::env::var("HOME").unwrap() + "/.cache/katac-repos"
    };

    if !std::path::Path::new(&dst).exists() {
        std::fs::create_dir_all(&dst).unwrap();
    }

    // TODO: add logic for when user adds a custom repo
    let url = DEFAULT_REPO;
    dst.push_str(format!("/{}", username_as_repo_name(url)).as_str());

    let output = std::process::Command::new("git")
        .args(["clone", &url, &dst])
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        panic!(
            "failed to clone repo: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    info!("repo cloned to: {}", dst);
}

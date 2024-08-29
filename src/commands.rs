use std::fs;
use std::io::Write;
use std::path::PathBuf;

use std::{self, process::Command};

pub const USE_MAKEFILE: bool = true;

pub fn run_custom_command(command: &str, kata_path: PathBuf) -> Option<std::process::Child> {
    let mut command = command.split_whitespace();
    Some(
        Command::new(command.next().unwrap())
            .args(command)
            .current_dir(kata_path)
            .spawn()
            .expect("failed to run the kata"),
    )
}

/// runs the kata in the given path
pub fn run_using_makefile(curday_kata_path: PathBuf) -> Option<std::process::Child> {
    if USE_MAKEFILE
        && Command::new("make")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .status()
            .is_ok()
    {
        return run_make_command(curday_kata_path);
    }
    run_os_command(curday_kata_path)
}

/// runs the kata in the given path using the make command
pub fn run_make_command(mut path: PathBuf) -> Option<std::process::Child> {
    let path_str = path
        .to_str()
        .expect("failed to convert path to string")
        .to_string();

    path.push("Makefile");
    if !path.exists() {
        println!("No Makefile found in {}", path_str);
        return None;
    }

    Some(
        Command::new("make")
            .arg("run")
            .arg("-s")
            .current_dir(path_str)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to run the kata"),
    )
}

/// runs the kata in the given path using an OS specific file (run.sh or run.bat)
fn run_os_command(run_path: PathBuf) -> Option<std::process::Child> {
    let run_path_str = run_path
        .to_str()
        .expect("failed to convert path to string")
        .to_string();

    if cfg!(target_os = "windows") {
        let bat_file = run_path.join("run.bat");
        if !bat_file.exists() {
            println!("No run.bat file found in {}", run_path_str);
            return None;
        }

        return Some(
            Command::new("cmd")
                .arg("/C")
                .arg(format!("cd {} && run.bat", run_path_str))
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("failed to run the kata"),
        );
    }

    let sh_file = run_path.join("run.sh");
    if !sh_file.exists() {
        println!("No run.sh file found in {}", run_path_str);
        return None;
    }

    return Some(
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && ./run.sh", run_path_str))
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to run the kata"),
    );
}

/// creates a new Makefile in the given path
pub fn create_makefile(mut path: PathBuf) {
    let content = "run:\n\t@echo \"TODO: add your run command here\"";
    path.push("Makefile");
    let mut f = fs::File::create(path).expect("failed to create the Makefile");
    f.write_all(content.as_bytes())
        .expect("failed to write to the Makefile");
}

/// creates a new run.sh or run.bat file in the given path
pub fn create_os_run_file(mut kata_path: PathBuf) {
    if cfg!(target_os = "windows") {
        let content = "TODO: add your run command here";
        kata_path.push("run.bat");
        let mut f =
            std::fs::File::create(kata_path).expect("failed to create the windows run file");
        f.write_all(content.as_bytes())
            .expect("failed to write to the windows run.bat file");
        return;
    }

    let content = "#!/usr/bin/env bash\n\n# TODO: replace this line with  your run command (example: npm run test)";
    kata_path.push("run.sh");
    let mut f = std::fs::File::create(&kata_path).expect("failed to create the linux run file");

    f.write_all(content.as_bytes())
        .expect("failed to write to the linux run.sh file");

    #[cfg(unix)]
    {
        use std::os::unix::prelude::PermissionsExt;
        std::fs::set_permissions(kata_path, fs::Permissions::from_mode(0o755))
            .expect("failed to set permissions on the linux run file");
    }
}

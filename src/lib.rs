use clap::{Parser, Subcommand};
use fs_extra::dir::CopyOptions;
use include_dir::{include_dir, Dir};
use inquire::{MultiSelect, Select};
use log::info;
use rand::{self, seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const KATAS_DIR: &str = "katas";
const DAYS_DIR: &str = "days";
const CONFIG_FILE_NAME: &str = "katac.toml";

// Embed the example-katas directory at compile time
static EXAMPLE_KATAS: Dir = include_dir!("$CARGO_MANIFEST_DIR/example-katas");

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
    /// Initialize katas by selecting from example templates (uses embedded katas by default)
    Init {
        /// Optional path to external examples directory (default: uses embedded katas)
        #[arg(long)]
        examples_dir: Option<String>,

        /// Select katas without interactive prompt (for testing/automation)
        #[arg(long, hide = true)]
        select: Option<String>,
    },

    /// Start a new day and copy specified katas
    Start {
        /// Katas to copy to new day
        #[arg(required = true, num_args = 1..)]
        kata_names: Vec<String>,
    },

    /// Katas you want to run today (requires a makefile with the  'run' target in the kata's root folder)
    Run {
        /// Katas to run
        #[arg(required = false, num_args = 1..)]
        kata_names: Option<Vec<String>>,

        /// Run custom command for given kata
        #[arg(short, long)]
        command: Option<String>,
    },

    /// Create a new kata
    New {
        /// Name of the kata you want to create
        #[arg(required = true, num_args = 1..)]
        kata_name: String,
    },

    /// Number of katas you want to do today, randomly taken from katas.toml
    Random {
        /// Katas to run
        #[arg(required = true, num_args = 1..)]
        number_of_katas: u8,
    },

    /// Upgrade katac to the latest version
    Upgrade {
        /// Force reinstallation even if already on latest version
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Deserialize, Debug)]
struct Data {
    katas: Katas,
}

/// config file structure
#[derive(Deserialize, Debug)]
struct Katas {
    random: Option<Vec<String>>,
    katas_dir: Option<String>,
    days_dir: Option<String>,
}

// returns the current day number
fn curday(days_dir: &str) -> u32 {
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

/// Generic function to get directory with priority:
/// CLI arg > env var > config file > default value
fn get_dir(
    arg_value: &Option<String>,
    env_var: &str,
    config_extractor: fn(&Data) -> Option<String>,
    config_file: &Option<String>,
    default: &str,
) -> String {
    if let Some(dir) = arg_value {
        return dir.clone();
    }

    if let Ok(env_value) = std::env::var(env_var) {
        return env_value;
    }

    let config_file_name = config_file
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or(CONFIG_FILE_NAME);

    // Only read config file if it exists
    if Path::new(config_file_name).exists() {
        if let Some(config_value) =
            config_extractor(&read_config_file(config_file_name.to_string()))
        {
            return config_value;
        }
    }

    default.to_string()
}

/// priorities are:
/// --katas-dir arg
/// KATAS_DIR env var
/// katas_dir config file property
/// default value
fn katas_dir(args: &Args) -> String {
    get_dir(
        &args.katas_dir,
        "KATAS_DIR",
        |data| data.katas.katas_dir.clone(),
        &args.config,
        KATAS_DIR,
    )
}

/// priorities are:
/// --days-dir arg
/// DAYS_DIR env var
/// days_dir config file property
/// default value
fn days_dir(args: &Args) -> String {
    get_dir(
        &args.days_dir,
        "DAYS_DIR",
        |data| data.katas.days_dir.clone(),
        &args.config,
        DAYS_DIR,
    )
}

/// copies katas from the katas_dir to a new day in days_dir
pub fn copy_katas(args: &Args, kata_names: &Vec<String>) {
    if kata_names.is_empty() {
        eprintln!("Error: no katas specified");
        std::process::exit(1);
    }

    let dst = nextday_path(&days_dir(args));
    let mut errors = Vec::new();

    for kata_name in kata_names {
        // Validate kata name if it's not a path
        if !kata_name.contains('/') && kata_name.contains("..") {
            eprintln!("Error: kata name '{}' cannot contain '..'", kata_name);
            errors.push(kata_name.clone());
            continue;
        }

        let src = kata_path(kata_name, &katas_dir(args));
        if !src.exists() {
            eprintln!("Error: Kata '{}' does not exist", kata_name);
            errors.push(kata_name.clone());
            continue;
        }
        if !dst.exists() {
            if let Err(e) = fs::create_dir_all(&dst) {
                eprintln!("Error: Failed to create days folder: {}", e);
                std::process::exit(1);
            }
        }
        match fs_extra::copy_items(&[&src], &dst, &CopyOptions::new()) {
            Ok(_) => {
                println!("Copying {} to {}...", kata_name, basename(&dst));

                // Check if this is an embedded kata and ensure Makefile exists
                let copied_path = dst.join(kata_name);
                if let Some((language, _)) = is_embedded_kata(kata_name) {
                    ensure_makefile_exists(&copied_path, &language, kata_name);
                }
            }
            Err(e) => {
                eprintln!("Error copying '{}': {}", kata_name, e);
                errors.push(kata_name.clone());
            }
        }
    }

    if !errors.is_empty() {
        eprintln!("\nFailed to copy {} kata(s)", errors.len());
        std::process::exit(1);
    }
}

/// returns the basename of a path
fn basename(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(".")
        .to_string()
}

/// returns the dirname of a path
fn dirname(path: &Path) -> String {
    path.parent()
        .and_then(|p| p.to_str())
        .unwrap_or(".")
        .to_string()
}

/// runs the katas in the current day
pub fn run_katas(args: &Args, kata_names: &Option<Vec<String>>, command: &Option<String>) {
    let days_dir = days_dir(args);
    let curday_path = curday_path(&days_dir);

    let kata_names = match kata_names {
        Some(kata_names) => kata_names.clone(),
        None => curday_katas(curday_path),
    };

    for (i, kata_name) in kata_names.iter().enumerate() {
        let curday_kata_path = curday_kata_path(&days_dir, kata_name);
        let run_str = format!("\n> Running {} [{}/{}]", kata_name, i + 1, kata_names.len());
        println!("{}", run_str);
        let width = run_str.chars().count();
        println!("{}", "-".repeat(width));

        if let Some(ref cmd_str) = command {
            let mut command_parts = cmd_str.split_whitespace();
            match command_parts.next() {
                Some(cmd) => {
                    let mut child = Command::new(cmd)
                        .args(command_parts)
                        .current_dir(curday_kata_path)
                        .spawn()
                        .expect("failed to run the kata");
                    child.wait().expect("failed to wait on child");
                }
                None => {
                    println!("Error: empty command provided");
                }
            }
            continue;
        }

        match run(curday_kata_path) {
            Some(mut child) => child.wait().expect("failed to wait on child"),
            None => continue,
        };
    }
}

/// runs the kata in the given path
fn run(curday_kata_path: PathBuf) -> Option<std::process::Child> {
    if Command::new("make")
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
fn run_make_command(path: PathBuf) -> Option<std::process::Child> {
    let makefile_path = path.join("Makefile");

    if !makefile_path.exists() {
        // Check if this is an embedded example kata
        if let Some(kata_name) = path.file_name().and_then(|n| n.to_str()) {
            if let Some((language, _)) = is_embedded_kata(kata_name) {
                if let Some(run_cmd) = get_embedded_run_command(&language, kata_name) {
                    println!("Running embedded kata command: {}", run_cmd);
                    // Parse and execute the command
                    let mut parts = run_cmd.split_whitespace();
                    if let Some(cmd) = parts.next() {
                        return Some(
                            Command::new(cmd)
                                .args(parts)
                                .current_dir(path)
                                .stdout(std::process::Stdio::inherit())
                                .stderr(std::process::Stdio::inherit())
                                .spawn()
                                .expect("failed to run the kata"),
                        );
                    }
                }
            }
        }

        println!("No Makefile found in {}", path.display());
        return None;
    }

    Some(
        Command::new("make")
            .arg("run")
            .arg("-s")
            .current_dir(path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to run the kata"),
    )
}

/// runs the kata in the given path using an OS specific file (run.sh or run.bat)
fn run_os_command(run_path: PathBuf) -> Option<std::process::Child> {
    if cfg!(target_os = "windows") {
        let bat_file = run_path.join("run.bat");
        if !bat_file.exists() {
            println!("No run.bat file found in {}", run_path.display());
            return None;
        }

        return Some(
            Command::new("cmd")
                .arg("/C")
                .arg("run.bat")
                .current_dir(&run_path)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("failed to run the kata"),
        );
    }

    let sh_file = run_path.join("run.sh");
    if !sh_file.exists() {
        println!("No run.sh file found in {}", run_path.display());
        return None;
    }

    Some(
        Command::new("sh")
            .arg("./run.sh")
            .current_dir(&run_path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to run the kata"),
    )
}

/// returns a vector of random katas from the katas.toml file or the katas folder
pub fn random_katas(args: &Args, number_of_katas: u8) -> Vec<String> {
    if number_of_katas == 0 {
        eprintln!("Error: number of katas must be greater than 0");
        std::process::exit(1);
    }

    let config_file = match args.config.as_ref() {
        Some(config_file) => config_file.as_str(),
        None => CONFIG_FILE_NAME,
    };

    let mut kata_names: Vec<String>;
    if std::path::Path::new(&config_file).exists() {
        kata_names = read_random_katas_from_config_file(config_file.to_string());
        if number_of_katas > kata_names.len() as u8 {
            eprintln!(
                "Error: random number ({}) is higher than the number of katas found ({}) in the katas.toml file",
                number_of_katas,
                kata_names.len()
            );
            std::process::exit(1);
        }
    } else {
        info!("no katas.toml found, reading katas folder for random katas");
        // kata_names becomes all files inside the katas folder
        kata_names = katas(&katas_dir(args));
        if kata_names.is_empty() {
            eprintln!("Error: no katas found in the katas folder");
            std::process::exit(1);
        }
        if number_of_katas > kata_names.len() as u8 {
            eprintln!(
                "Error: random number ({}) is higher than the number of katas found ({}) in the katas folder",
                number_of_katas,
                kata_names.len()
            );
            std::process::exit(1);
        }
        kata_names.shuffle(&mut thread_rng());
    }
    kata_names[0..number_of_katas as usize].to_vec()
}

/// creates a new kata in the kata_dir folder or the given path
pub fn new_kata(args: &Args, kata_name: &str) {
    // Validate kata name if it's not a path
    if !kata_name.contains('/') {
        if kata_name.is_empty() {
            eprintln!("Error: kata name cannot be empty");
            std::process::exit(1);
        }
        if kata_name.contains("..") {
            eprintln!("Error: kata name cannot contain '..'");
            std::process::exit(1);
        }
        if kata_name.starts_with('.') {
            eprintln!("Error: kata name cannot start with '.'");
            std::process::exit(1);
        }
    }

    let kata_dir = &katas_dir(args);
    let kata_path = kata_path(kata_name, kata_dir);
    if kata_path.exists() {
        println!("Kata {} already exists", kata_name);
        std::process::exit(1);
    }
    fs::create_dir_all(&kata_path).expect("failed to create the kata folder");
    println!("{} created in {}.", kata_name, dirname(&kata_path));

    if Command::new("make")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .status()
        .is_ok()
    {
        create_makefile(kata_path);
    } else {
        create_os_run_file(kata_path);
    }
}

/// creates a new Makefile in the given path
fn create_makefile(mut path: PathBuf) {
    let content = "run:\n\t@echo \"TODO: add your run command here\"";
    path.push("Makefile");
    let mut f = fs::File::create(path).expect("failed to create the Makefile");
    f.write_all(content.as_bytes())
        .expect("failed to write to the Makefile");
}

/// creates a new run.sh or run.bat file in the given path
fn create_os_run_file(mut kata_path: PathBuf) {
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

/// returns a vector of katas from the katas folder
fn katas(katas_dir: &str) -> Vec<String> {
    fs::read_dir(katas_dir)
        .expect("Unable to read katas folder")
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

/// returns a vector of katas from the current day folder
fn curday_katas(curday_path: PathBuf) -> Vec<String> {
    fs::read_dir(curday_path)
        .expect("Unable to read current day contents")
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

/// returns the path of the given kata
fn kata_path(kata_name: &str, katas_dir: &str) -> PathBuf {
    if kata_name.contains('/') {
        return PathBuf::from(kata_name);
    }
    PathBuf::from(format!("{}/{}", katas_dir, kata_name))
}

/// returns the path of the current day
fn curday_path(days_dir: &str) -> PathBuf {
    PathBuf::from(format!("{}/day{}", days_dir, curday(days_dir)))
}

/// returns the path of the next day
fn nextday_path(days_dir: &str) -> PathBuf {
    PathBuf::from(format!("{}/day{}", days_dir, curday(days_dir) + 1))
}

/// returns the path of the given kata in the current day
fn curday_kata_path(days_dir: &str, kata_name: &str) -> PathBuf {
    curday_path(days_dir).join(kata_name)
}

/// reads the katas.toml file and returns a Data struct
fn read_config_file(config_file_name: String) -> Data {
    info!("Reading katas.toml file");

    let str =
        fs::read_to_string(config_file_name).expect("Something went wrong reading the config file");
    toml::from_str(&str).expect("Something went wrong reading the config file")
}

/// reads the katas.toml file and returns a vector of random katas
fn read_random_katas_from_config_file(config_file: String) -> Vec<String> {
    let tom = read_config_file(config_file);

    let mut kata_names = tom.katas.random.unwrap_or_default();
    kata_names.shuffle(&mut thread_rng());
    if kata_names.is_empty() {
        println!("config file is empty");
        std::process::exit(1);
    }
    kata_names
}

/// scans the embedded example katas and returns a list of (language, kata_name) tuples
fn scan_embedded_katas() -> Vec<(String, String)> {
    let mut katas = Vec::new();

    // Iterate through language directories in embedded katas
    for language_dir in EXAMPLE_KATAS.dirs() {
        let language = language_dir
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Iterate through kata directories within each language
        for kata_dir in language_dir.dirs() {
            let kata_name = kata_dir
                .path()
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            if !kata_name.is_empty() && !language.is_empty() {
                katas.push((language.clone(), kata_name));
            }
        }
    }

    katas.sort_by(|a, b| {
        // Sort by kata name first, then by language
        match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        }
    });

    katas
}

/// Checks if a kata is from embedded example-katas
fn is_embedded_kata(kata_name: &str) -> Option<(String, String)> {
    let available = scan_embedded_katas();
    available.into_iter().find(|(_, name)| name == kata_name)
}

/// Extracts the run command from an embedded kata's Makefile
fn get_embedded_run_command(language: &str, kata_name: &str) -> Option<String> {
    let makefile_path = format!("{}/{}/Makefile", language, kata_name);
    let makefile = EXAMPLE_KATAS.get_file(&makefile_path)?;
    let content = std::str::from_utf8(makefile.contents()).ok()?;

    // Parse Makefile to extract run target command
    for line in content.lines() {
        let trimmed = line.trim();
        // Skip the "run:" line, get the command (starts with tab)
        if line.starts_with('\t') && !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

/// Copies the Makefile from an embedded kata if it doesn't exist
fn ensure_makefile_exists(dest: &Path, language: &str, kata_name: &str) {
    let makefile_dest = dest.join("Makefile");

    // Don't overwrite existing Makefile
    if makefile_dest.exists() {
        return;
    }

    // Copy from embedded kata
    let makefile_path = format!("{}/{}/Makefile", language, kata_name);
    if let Some(makefile) = EXAMPLE_KATAS.get_file(&makefile_path) {
        if let Ok(()) = fs::write(&makefile_dest, makefile.contents()) {
            println!("  → Created Makefile for {}", kata_name);
        }
    }
}

/// Extracts unique language names from embedded katas
fn get_available_languages() -> Vec<String> {
    let mut languages: Vec<String> = EXAMPLE_KATAS
        .dirs()
        .filter_map(|dir| {
            dir.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
        })
        .collect();

    languages.sort();
    languages.dedup();
    languages
}

/// Filters katas by language
fn filter_katas_by_language(all_katas: &[(String, String)], language: &str) -> Vec<String> {
    all_katas
        .iter()
        .filter(|(lang, _)| lang == language)
        .map(|(_, kata)| kata.clone())
        .collect()
}

/// scans an external examples directory and returns a list of (language, kata_name) tuples
fn scan_external_katas(examples_dir: &str) -> Vec<(String, String)> {
    let mut katas = Vec::new();

    let examples_path = Path::new(examples_dir);
    if !examples_path.exists() {
        eprintln!(
            "Error: Examples directory '{}' does not exist",
            examples_dir
        );
        std::process::exit(1);
    }

    let entries = match fs::read_dir(examples_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error: Failed to read examples directory: {}", e);
            std::process::exit(1);
        }
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let language_path = entry.path();
        if !language_path.is_dir() {
            continue;
        }

        let language = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => continue,
        };

        let kata_entries = match fs::read_dir(&language_path) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for kata_entry in kata_entries.filter_map(|e| e.ok()) {
            if !kata_entry.path().is_dir() {
                continue;
            }

            if let Ok(kata_name) = kata_entry.file_name().into_string() {
                katas.push((language.clone(), kata_name));
            }
        }
    }

    katas.sort_by(|a, b| {
        // Sort by kata name first, then by language
        match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        }
    });

    katas
}

/// copies an embedded kata directory to the destination
fn copy_embedded_kata(language: &str, kata_name: &str, dest: &Path) -> std::io::Result<()> {
    // Get the embedded kata directory
    let kata_path_str = format!("{}/{}", language, kata_name);
    let kata_dir = EXAMPLE_KATAS.get_dir(&kata_path_str).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Embedded kata not found: {}", kata_path_str),
        )
    })?;

    // Recursively copy all files from the embedded directory
    fn copy_dir_recursive(dir: &include_dir::Dir, dest: &Path) -> std::io::Result<()> {
        // Create destination directory
        fs::create_dir_all(dest)?;

        // Copy all files in this directory
        for file in dir.files() {
            let file_path = dest.join(file.path().file_name().unwrap());
            fs::write(&file_path, file.contents())?;

            // Set executable permissions for run.sh files
            #[cfg(unix)]
            if file_path.file_name().and_then(|n| n.to_str()) == Some("run.sh") {
                use std::os::unix::prelude::PermissionsExt;
                fs::set_permissions(&file_path, fs::Permissions::from_mode(0o755))?;
            }
        }

        // Recursively copy subdirectories
        for subdir in dir.dirs() {
            let subdir_name = subdir.path().file_name().unwrap();
            let subdir_dest = dest.join(subdir_name);
            copy_dir_recursive(subdir, &subdir_dest)?;
        }

        Ok(())
    }

    copy_dir_recursive(kata_dir, dest)
}

/// initializes katas by selecting from example templates
pub fn init_from_examples(args: &Args, examples_dir: &Option<String>, select: &Option<String>) {
    let katas_path = katas_dir(args);

    // Determine whether to use embedded or external katas
    let use_embedded = examples_dir.is_none();

    // Scan for available example katas
    let available_katas = if use_embedded {
        scan_embedded_katas()
    } else {
        scan_external_katas(examples_dir.as_ref().unwrap())
    };

    if available_katas.is_empty() {
        if use_embedded {
            eprintln!("Error: No embedded example katas found");
        } else {
            eprintln!(
                "Error: No example katas found in '{}'",
                examples_dir.as_ref().unwrap()
            );
        }
        std::process::exit(1);
    }

    // Format options for display: "[language] KataName"
    let options: Vec<String> = available_katas
        .iter()
        .map(|(lang, kata)| format!("[{}] {}", lang, kata))
        .collect();

    // Get selections (either from --select flag or interactive prompt)
    let selections: Vec<usize> = if let Some(select_str) = select {
        // Non-interactive mode for testing/automation
        let selected_names: Vec<String> = select_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        options
            .iter()
            .enumerate()
            .filter(|(_, opt)| selected_names.iter().any(|sel| opt.contains(sel)))
            .map(|(i, _)| i)
            .collect()
    } else {
        // Interactive mode - Two-step process

        // Step 1: Select language
        let languages = get_available_languages();
        let selected_language = match Select::new("Choose a language", languages.clone()).prompt() {
            Ok(lang) => lang,
            Err(e) => {
                // Exit silently if user cancelled the operation
                if matches!(e, inquire::InquireError::OperationCanceled) {
                    std::process::exit(0);
                }
                eprintln!("Error: Failed to read user input: {}", e);
                std::process::exit(1);
            }
        };

        // Step 2: Fuzzy multi-select katas for chosen language
        let language_katas = filter_katas_by_language(&available_katas, &selected_language);

        if language_katas.is_empty() {
            eprintln!("Error: No katas found for language '{}'", selected_language);
            std::process::exit(1);
        }

        let selected_kata_names = match MultiSelect::new(
            format!(
                "Select {} katas (type to filter, SPACE to select, ENTER to confirm)",
                selected_language
            )
            .as_str(),
            language_katas.clone(),
        )
        .prompt()
        {
            Ok(selections) => selections,
            Err(e) => {
                // Exit silently if user cancelled the operation
                if matches!(e, inquire::InquireError::OperationCanceled) {
                    std::process::exit(0);
                }
                eprintln!("Error: Failed to read user input: {}", e);
                std::process::exit(1);
            }
        };

        // Map selected kata names back to indices in available_katas
        selected_kata_names
            .into_iter()
            .filter_map(|kata_name| {
                available_katas
                    .iter()
                    .position(|(lang, name)| lang == &selected_language && name == &kata_name)
            })
            .collect()
    };

    if selections.is_empty() {
        println!("No katas selected. Exiting.");
        return;
    }

    // Create katas directory if it doesn't exist
    if !Path::new(&katas_path).exists() {
        if let Err(e) = fs::create_dir_all(&katas_path) {
            eprintln!("Error: Failed to create katas directory: {}", e);
            std::process::exit(1);
        }
    }

    // Copy selected katas
    let mut errors = Vec::new();
    let mut copied_count = 0;
    let mut seen_names: HashSet<String> = HashSet::new();

    for &idx in &selections {
        let (language, kata_name) = &available_katas[idx];
        let language = language.as_str();
        let kata_name = kata_name.as_str();

        // Determine final destination name
        let final_dest_name = if seen_names.contains(kata_name) {
            format!("{}_{}", language, kata_name)
        } else {
            kata_name.to_string()
        };

        let final_dest = PathBuf::from(&katas_path).join(&final_dest_name);

        // Skip if final destination already exists
        if final_dest.exists() {
            println!(
                "Note: {} already exists, skipping [{}] {}",
                final_dest_name, language, kata_name
            );
            continue;
        }

        // Show note if we're renaming due to duplicate selection
        if final_dest_name != *kata_name {
            println!(
                "Note: {} was already selected, creating [{}] {} as {}",
                kata_name, language, kata_name, final_dest_name
            );
        }

        // Copy from embedded or external source
        let copy_result = if use_embedded {
            copy_embedded_kata(language, kata_name, &final_dest)
        } else {
            let src = PathBuf::from(examples_dir.as_ref().unwrap())
                .join(language)
                .join(kata_name);

            // For external, use the temp directory approach
            let temp_name = format!(".tmp_{}_{}", language, kata_name);
            let temp_path = PathBuf::from(&katas_path).join(&temp_name);

            if let Err(e) = fs::create_dir_all(&temp_path) {
                Err(std::io::Error::other(format!(
                    "Failed to create temp directory: {}",
                    e
                )))
            } else {
                match fs_extra::copy_items(&[&src], &temp_path, &CopyOptions::new()) {
                    Ok(_) => {
                        let copied_dir = temp_path.join(kata_name);
                        let result = fs::rename(&copied_dir, &final_dest);
                        let _ = fs::remove_dir_all(&temp_path);
                        result
                    }
                    Err(e) => {
                        let _ = fs::remove_dir_all(&temp_path);
                        Err(std::io::Error::other(e.to_string()))
                    }
                }
            }
        };

        match copy_result {
            Ok(_) => {
                println!(
                    "✓ Copied [{}] {} to {}/{}",
                    language, kata_name, katas_path, final_dest_name
                );

                // Ensure Makefile exists
                ensure_makefile_exists(&final_dest, language, kata_name);

                seen_names.insert(kata_name.to_string());
                copied_count += 1;
            }
            Err(e) => {
                eprintln!("Error copying [{}] {}: {}", language, kata_name, e);
                errors.push(format!("[{}] {}", language, kata_name));
            }
        }
    }

    println!("\nSuccessfully initialized {} kata(s)!", copied_count);

    if !errors.is_empty() {
        eprintln!("\nFailed to copy {} kata(s)", errors.len());
        std::process::exit(1);
    }
}

/// upgrades katac to the latest version from GitHub releases
pub fn upgrade_katac(force: bool) {
    const REPO: &str = "aldevv/katac";
    const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("Current version: {}", CURRENT_VERSION);
    println!("Checking for updates...");

    // Get latest version from GitHub
    let latest_version = match get_latest_github_version(REPO) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Failed to check for updates: {}", e);
            std::process::exit(1);
        }
    };

    println!("Latest version: {}", latest_version);

    // Compare versions
    let current = CURRENT_VERSION.trim_start_matches('v');
    let latest = latest_version.trim_start_matches('v');

    if !force && current == latest {
        println!("✓ Already on the latest version!");
        return;
    }

    if !force && is_version_newer(latest, current) == Some(false) {
        println!(
            "✓ Your version ({}) is newer than or equal to the latest release ({})!",
            current, latest
        );
        return;
    }

    println!("\nUpgrading from {} to {}...", current, latest);
    println!("Downloading katac {}...", latest_version);

    // Detect system
    let (os, arch) = match detect_system() {
        Ok(sys) => sys,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!(
                "Please install manually from: https://github.com/{}/releases",
                REPO
            );
            std::process::exit(1);
        }
    };

    let target = get_rust_target(&os, &arch);
    let ext = if os == "windows" { "zip" } else { "tar.gz" };
    let filename = format!("katac-{}.{}", target, ext);
    let url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        REPO, latest_version, filename
    );

    // Download to temp directory
    let temp_dir = match std::env::temp_dir().to_str() {
        Some(d) => PathBuf::from(d),
        None => {
            eprintln!("Error: Could not access temp directory");
            std::process::exit(1);
        }
    };

    let download_path = temp_dir.join(&filename);

    println!("Downloading from: {}", url);
    if let Err(e) = download_file(&url, &download_path) {
        eprintln!("Error: Failed to download: {}", e);
        std::process::exit(1);
    }

    // Extract
    println!("Extracting...");
    let extract_dir = temp_dir.join("katac_upgrade");
    let _ = fs::remove_dir_all(&extract_dir);
    if let Err(e) = fs::create_dir_all(&extract_dir) {
        eprintln!("Error: Failed to create extraction directory: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = extract_archive(&download_path, &extract_dir, ext) {
        eprintln!("Error: Failed to extract: {}", e);
        let _ = fs::remove_dir_all(&extract_dir);
        std::process::exit(1);
    }

    // Find the binary
    let binary_name = if os == "windows" {
        "katac.exe"
    } else {
        "katac"
    };
    let new_binary = extract_dir.join(binary_name);

    if !new_binary.exists() {
        eprintln!("Error: Binary not found in archive");
        let _ = fs::remove_dir_all(&extract_dir);
        std::process::exit(1);
    }

    // Get current executable path
    let current_exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: Could not determine current executable path: {}", e);
            let _ = fs::remove_dir_all(&extract_dir);
            std::process::exit(1);
        }
    };

    // Replace the binary
    println!("Installing to {}...", current_exe.display());

    // On Windows, we can't replace a running exe, so we rename it first
    #[cfg(target_os = "windows")]
    {
        let backup = current_exe.with_extension("exe.old");
        let _ = fs::remove_file(&backup);
        if let Err(e) = fs::rename(&current_exe, &backup) {
            eprintln!("Error: Failed to backup current binary: {}", e);
            let _ = fs::remove_dir_all(&extract_dir);
            std::process::exit(1);
        }

        if let Err(e) = fs::copy(&new_binary, &current_exe) {
            eprintln!("Error: Failed to install new binary: {}", e);
            let _ = fs::rename(&backup, &current_exe);
            let _ = fs::remove_dir_all(&extract_dir);
            std::process::exit(1);
        }

        let _ = fs::remove_file(&backup);
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;

        // Set executable permissions on the new binary
        if let Ok(metadata) = fs::metadata(&new_binary) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o755);
            let _ = fs::set_permissions(&new_binary, perms);
        }

        // On Unix, we need to remove the old binary first to avoid "Text file busy" error
        // The running process can continue executing from the deleted inode
        if let Err(e) = fs::remove_file(&current_exe) {
            eprintln!("Error: Failed to remove old binary: {}", e);
            let _ = fs::remove_dir_all(&extract_dir);
            std::process::exit(1);
        }

        if let Err(e) = fs::copy(&new_binary, &current_exe) {
            eprintln!("Error: Failed to install new binary: {}", e);
            let _ = fs::remove_dir_all(&extract_dir);
            std::process::exit(1);
        }
    }

    // Cleanup
    let _ = fs::remove_file(&download_path);
    let _ = fs::remove_dir_all(&extract_dir);

    println!("✓ Successfully upgraded to version {}!", latest);
    println!("Run 'katac --version' to verify");
}

fn get_latest_github_version(repo: &str) -> Result<String, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let output = Command::new("curl")
        .args(["-fsSL", &url])
        .output()
        .map_err(|e| format!("Failed to execute curl: {}", e))?;

    if !output.status.success() {
        return Err("Failed to fetch release info from GitHub".to_string());
    }

    let body = String::from_utf8_lossy(&output.stdout);
    for line in body.lines() {
        if line.contains("\"tag_name\":") {
            if let Some(version) = line.split('"').nth(3) {
                return Ok(version.to_string());
            }
        }
    }

    Err("Could not parse version from GitHub response".to_string())
}

fn detect_system() -> Result<(String, String), String> {
    let os_output = Command::new("uname")
        .arg("-s")
        .output()
        .map_err(|_| "Failed to detect OS")?;
    let os_raw = String::from_utf8_lossy(&os_output.stdout)
        .trim()
        .to_lowercase();

    let os = if os_raw.contains("linux") {
        "linux"
    } else if os_raw.contains("darwin") {
        "darwin"
    } else if os_raw.contains("mingw") || os_raw.contains("msys") || os_raw.contains("cygwin") {
        "windows"
    } else {
        return Err(format!("Unsupported OS: {}", os_raw));
    };

    let arch_output = Command::new("uname")
        .arg("-m")
        .output()
        .map_err(|_| "Failed to detect architecture")?;
    let arch_raw = String::from_utf8_lossy(&arch_output.stdout)
        .trim()
        .to_lowercase();

    let arch = match arch_raw.as_str() {
        "x86_64" | "amd64" => "x86_64",
        "aarch64" | "arm64" => "aarch64",
        "armv7l" => "armv7",
        "i686" | "i386" => "i686",
        _ => return Err(format!("Unsupported architecture: {}", arch_raw)),
    };

    Ok((os.to_string(), arch.to_string()))
}

fn get_rust_target(os: &str, arch: &str) -> String {
    match (os, arch) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("linux", "armv7") => "armv7-unknown-linux-gnueabihf",
        ("darwin", "x86_64") => "x86_64-apple-darwin",
        ("darwin", "aarch64") => "aarch64-apple-darwin",
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        ("windows", "i686") => "i686-pc-windows-msvc",
        ("windows", "aarch64") => "aarch64-pc-windows-msvc",
        _ => "x86_64-unknown-linux-gnu",
    }
    .to_string()
}

fn download_file(url: &str, dest: &Path) -> std::io::Result<()> {
    let output = Command::new("curl")
        .args(["-fsSL", url, "-o"])
        .arg(dest)
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::other("Download failed"));
    }

    Ok(())
}

fn extract_archive(archive: &Path, dest: &Path, ext: &str) -> std::io::Result<()> {
    if ext == "zip" {
        let output = Command::new("unzip")
            .arg("-q")
            .arg(archive)
            .arg("-d")
            .arg(dest)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::other("Extraction failed"));
        }
    } else {
        let output = Command::new("tar")
            .args(["xzf"])
            .arg(archive)
            .arg("-C")
            .arg(dest)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::other("Extraction failed"));
        }
    }

    Ok(())
}

fn is_version_newer(v1: &str, v2: &str) -> Option<bool> {
    let parse_version = |v: &str| -> Option<Vec<u32>> {
        v.split('.')
            .map(|s| s.parse::<u32>().ok())
            .collect::<Option<Vec<u32>>>()
    };

    let ver1 = parse_version(v1)?;
    let ver2 = parse_version(v2)?;

    Some(ver1 > ver2)
}

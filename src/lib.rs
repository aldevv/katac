use clap::{Parser, Subcommand};
use dialoguer::MultiSelect;
use fs_extra::dir::CopyOptions;
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
    /// Katas you want to run today (requires a makefile with the  'run' target in the kata's root folder)
    Run {
        /// Katas to run
        #[arg(required = false, num_args = 1..)]
        kata_names: Option<Vec<String>>,

        /// Run custom command for given kata
        #[arg(short, long)]
        command: Option<String>,
    },

    /// Number of katas you want to do today, randomly taken from katas.toml
    Random {
        /// Katas to run
        #[arg(required = true, num_args = 1..)]
        number_of_katas: u8,
    },

    /// Create a new kata
    New {
        /// Name of the kata you want to create
        #[arg(required = true, num_args = 1..)]
        kata_name: String,
    },

    /// Initialize katas by selecting from example templates
    Init {
        /// Optional path to examples directory (default: ./example-katas)
        #[arg(long)]
        examples_dir: Option<String>,

        /// Select katas without interactive prompt (for testing/automation)
        #[arg(long, hide = true)]
        select: Option<String>,
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
            Ok(_) => println!("Copying {} to {}...", kata_name, basename(&dst)),
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

/// scans the examples directory and returns a list of (language, kata_name) tuples
fn scan_example_katas(examples_dir: &str) -> Vec<(String, String)> {
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

/// initializes katas by selecting from example templates
pub fn init_from_examples(args: &Args, examples_dir: &Option<String>, select: &Option<String>) {
    const DEFAULT_EXAMPLES_DIR: &str = "example-katas";

    let examples_path = examples_dir.as_deref().unwrap_or(DEFAULT_EXAMPLES_DIR);
    let katas_path = katas_dir(args);

    // Scan for available example katas
    let available_katas = scan_example_katas(examples_path);

    if available_katas.is_empty() {
        eprintln!("Error: No example katas found in '{}'", examples_path);
        std::process::exit(1);
    }

    // Format options for display: "[language] KataName"
    let options: Vec<String> = available_katas
        .iter()
        .map(|(lang, kata)| format!("[{}] {}", lang, kata))
        .collect();

    // Get selections (either from --select flag or interactive prompt)
    let selections = if let Some(select_str) = select {
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
        // Interactive mode
        match MultiSelect::new()
            .with_prompt("Select katas to initialize (SPACE to select, ENTER to confirm)")
            .items(&options)
            .interact()
        {
            Ok(selections) => selections,
            Err(e) => {
                eprintln!("Error: Failed to read user input: {}", e);
                std::process::exit(1);
            }
        }
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
        let src = PathBuf::from(examples_path).join(language).join(kata_name);

        // Determine final destination name
        let final_dest_name = if seen_names.contains(kata_name) {
            format!("{}_{}", language, kata_name)
        } else {
            kata_name.clone()
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

        // Copy to a temporary unique name first to avoid conflicts during copy
        let temp_name = format!(".tmp_{}_{}", language, kata_name);
        let temp_path = PathBuf::from(&katas_path).join(&temp_name);

        // Create a temporary subdirectory to copy into
        if let Err(e) = fs::create_dir_all(&temp_path) {
            eprintln!(
                "Error creating temp directory for [{}] {}: {}",
                language, kata_name, e
            );
            errors.push(format!("[{}] {}", language, kata_name));
            continue;
        }

        // Copy items into the temp directory
        match fs_extra::copy_items(&[&src], &temp_path, &CopyOptions::new()) {
            Ok(_) => {
                // The copied directory is now at temp_path/kata_name
                let copied_dir = temp_path.join(kata_name);

                // Move to final destination
                if let Err(e) = fs::rename(&copied_dir, &final_dest) {
                    eprintln!(
                        "Error moving [{}] {} to final destination: {}",
                        language, kata_name, e
                    );
                    // Clean up temp directory
                    let _ = fs::remove_dir_all(&temp_path);
                    errors.push(format!("[{}] {}", language, kata_name));
                    continue;
                }

                // Clean up temp directory
                let _ = fs::remove_dir_all(&temp_path);

                println!(
                    "✓ Copied [{}] {} to {}/{}",
                    language, kata_name, katas_path, final_dest_name
                );
                seen_names.insert(kata_name.clone());
                copied_count += 1;
            }
            Err(e) => {
                eprintln!("Error copying [{}] {}: {}", language, kata_name, e);
                let _ = fs::remove_dir_all(&temp_path);
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

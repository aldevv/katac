use assert_cmd::Command;
use std::error::Error;
const DAY_FOLDER: &str = "tests/day_test";
const PRG: &str = "katac";

fn cleanup(day_folder: &str) {
    std::fs::remove_dir_all(day_folder).unwrap();
}

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn test_copy_kata() -> TestResult {
    let test_day_folder = format!("{}_copy", DAY_FOLDER);
    Command::cargo_bin(PRG)?
        .args([
            "foo",
            "--days-dir",
            &test_day_folder,
            "--katas-dir",
            "tests/example_katas",
        ])
        .assert()
        .stdout("Copying foo to day1...\n");
    cleanup(&test_day_folder);
    Ok(())
}

#[test]
fn test_run_kata() -> TestResult {
    let test_day_folder = format!("{}_run", DAY_FOLDER);
    Command::cargo_bin(PRG)?
        .args(["baz"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout("Copying baz to day1...\n");

    let cmd = Command::cargo_bin(PRG)?
        .args(["run", "baz"])
        .env("DAYS_DIR", &test_day_folder)
        .assert();

    let output = String::from_utf8(cmd.get_output().stdout.clone())?;
    assert!(output.contains("> Running baz [1/1]"));
    assert!(output.contains("console.log(\"hello world\")"));

    cleanup(&test_day_folder);
    Ok(())
}

#[test]
fn test_multiple_kata() -> TestResult {
    let test_day_folder = format!("{}_multiple", DAY_FOLDER);
    Command::cargo_bin(PRG)?
        .args([
            "--days-dir",
            &test_day_folder,
            "--katas-dir",
            "tests/example_katas",
            "foo",
            "bar",
            "baz",
        ])
        .assert()
        .stdout("Copying foo to day1...\nCopying bar to day1...\nCopying baz to day1...\n");
    cleanup(&test_day_folder);
    Ok(())
}

#[test]
fn test_random_with_config_file() -> TestResult {
    std::fs::write(
        "tests/katac.toml",
        r#"
[katas]
random = ["foo", "bar", "baz"]
"#,
    )
    .expect("Unable to write config file");

    let test_day_folder = format!("{}_random_with_config", DAY_FOLDER);
    let katas = ["foo", "bar", "baz"];
    for _ in 0..5 {
        Command::cargo_bin(PRG)?
            .args([
                "--days-dir",
                &test_day_folder,
                "--katas-dir",
                "tests/example_katas",
                "--config",
                "tests/katac.toml",
                "random",
                "2",
            ])
            .assert()
            .code(0);

        let day_folder = std::path::Path::new(&test_day_folder).join("day1");
        assert!(day_folder.read_dir().unwrap().count() == 2);

        for f in day_folder.read_dir().unwrap() {
            let folder = f.unwrap();
            let folder_name = folder.file_name().into_string().unwrap();
            assert!(katas.contains(&folder_name.as_str()));
        }

        cleanup(&test_day_folder);
    }
    std::fs::remove_file("tests/katac.toml").unwrap();
    Ok(())
}

#[test]
fn test_random_no_config_file() -> TestResult {
    let test_day_folder = format!("{}_random_no_config", DAY_FOLDER);

    let katas = ["foo", "bar", "baz"];
    for _ in 0..5 {
        Command::cargo_bin(PRG)?
            .args([
                "--days-dir",
                &test_day_folder,
                "--katas-dir",
                "tests/example_katas",
                "--config",
                "none",
                "random",
                "2",
            ])
            .assert()
            .code(0);

        let day_folder = std::path::Path::new(&test_day_folder).join("day1");
        assert!(day_folder.read_dir().unwrap().count() == 2);

        for f in day_folder.read_dir().unwrap() {
            let folder = f.unwrap();
            let folder_name = folder.file_name().into_string().unwrap();
            assert!(katas.contains(&folder_name.as_str()));
        }

        cleanup(&test_day_folder);
    }
    Ok(())
}

#[test]
fn test_run_kata_no_makefile() -> TestResult {
    let test_day_folder = format!("{}_run_no_makefile", DAY_FOLDER);
    Command::cargo_bin(PRG)?
        .args(["foo"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout("Copying foo to day1...\n");

    let cmd = Command::cargo_bin(PRG)?
        .args(["run", "foo"])
        .env("DAYS_DIR", &test_day_folder)
        .assert();

    let output = String::from_utf8(cmd.get_output().stdout.clone())?;
    assert!(output.contains("> Running foo [1/1]"));
    assert!(output.contains("No Makefile found"));
    assert!(output.contains(&test_day_folder));
    assert!(output.contains("foo"));

    cleanup(&test_day_folder);
    Ok(())
}

#[test]
fn test_run_all() -> TestResult {
    let test_day_folder = format!("{}_run_all", DAY_FOLDER);
    let cmd = Command::cargo_bin(PRG)?
        .args(["foo", "bar", "baz"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &test_day_folder)
        .assert();

    let copy_output = String::from_utf8(cmd.get_output().stdout.clone())?;
    for s in ["foo", "bar", "baz"].iter() {
        assert!(copy_output.contains(&format!("Copying {} to day1...", s)));
    }

    let cmd = Command::cargo_bin(PRG)?
        .args(["run"])
        .env("DAYS_DIR", &test_day_folder)
        .assert();
    let run_output = String::from_utf8(cmd.get_output().stdout.clone())?;
    for s in ["foo", "bar", "baz"].iter() {
        assert!(run_output.contains(&format!("> Running {}", s)));
        match s {
            &"foo" | &"bar" => {
                assert!(run_output.contains("No Makefile found"));
                assert!(run_output.contains(&test_day_folder));
            }
            &"baz" => {
                assert!(run_output.contains("console.log(\"hello world\")"));
            }
            _ => {}
        }
    }
    cleanup(&test_day_folder);
    Ok(())
}

#[test]
fn test_new_command() -> TestResult {
    let new_kata_folder = "tests/new_katas";
    Command::cargo_bin(PRG)?
        .args(["new", "foo2"])
        .env("KATAS_DIR", new_kata_folder)
        .assert()
        .stdout("foo2 created in tests/new_katas.\n");
    cleanup(new_kata_folder);
    Ok(())
}

#[test]
fn test_new_command_already_exists() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(["new", "foo"])
        .env("KATAS_DIR", "tests/example_katas")
        .assert()
        .stdout("Kata foo already exists\n");
    Ok(())
}

#[test]
fn test_init_command() -> TestResult {
    let test_katas_dir = format!("{}_init", DAY_FOLDER);

    // Test with --select flag to bypass interactive mode
    // Using real example-katas directory with language structure
    let cmd = Command::cargo_bin(PRG)?
        .args(["init", "--select", "Queue,Map"])
        .env("KATAS_DIR", &test_katas_dir)
        .args(["--examples-dir", "example-katas"])
        .assert()
        .code(0);

    let output = String::from_utf8(cmd.get_output().stdout.clone())?;
    assert!(output.contains("Successfully initialized"));
    assert!(output.contains("Queue") || output.contains("Map"));

    // Verify at least one kata was copied
    let katas_path = std::path::Path::new(&test_katas_dir);
    assert!(katas_path.exists());
    let count = std::fs::read_dir(katas_path)?.count();
    assert!(count > 0, "No katas were copied");

    cleanup(&test_katas_dir);
    Ok(())
}

#[test]
fn test_init_command_with_duplicates() -> TestResult {
    let test_katas_dir = format!("{}_init_dup", DAY_FOLDER);

    // Select the same kata from multiple languages (if available)
    // This tests the conflict resolution where second selection gets language prefix
    let cmd = Command::cargo_bin(PRG)?
        .args(["init", "--select", "ArrayList"])
        .env("KATAS_DIR", &test_katas_dir)
        .args(["--examples-dir", "example-katas"])
        .assert()
        .code(0);

    let output = String::from_utf8(cmd.get_output().stdout.clone())?;
    assert!(output.contains("Successfully initialized"));

    // Should have created at least one ArrayList variant
    let katas_path = std::path::Path::new(&test_katas_dir);
    let has_arraylist = katas_path.join("ArrayList").exists()
        || katas_path.join("go_ArrayList").exists()
        || katas_path.join("python_ArrayList").exists();
    assert!(has_arraylist, "No ArrayList variant was created");

    cleanup(&test_katas_dir);
    Ok(())
}

#[test]
fn test_start_command() -> TestResult {
    let test_day_folder = format!("{}_start", DAY_FOLDER);
    Command::cargo_bin(PRG)?
        .args(["start", "foo", "bar"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout("Copying foo to day1...\nCopying bar to day1...\n");

    // Verify day was created with katas
    let day1_path = std::path::Path::new(&test_day_folder).join("day1");
    assert!(day1_path.exists());
    assert!(day1_path.join("foo").exists());
    assert!(day1_path.join("bar").exists());

    cleanup(&test_day_folder);
    Ok(())
}

#[test]
fn test_init_creates_makefile() -> TestResult {
    let test_katas_dir = format!("{}_init_makefile", DAY_FOLDER);

    // Use init to copy an embedded kata
    Command::cargo_bin(PRG)?
        .args(["init", "--select", "Queue"])
        .env("KATAS_DIR", &test_katas_dir)
        .assert()
        .code(0);

    // Verify Makefile was created (check for Queue or language-prefixed variant)
    let katas_path = std::path::Path::new(&test_katas_dir);
    let queue_dir = if katas_path.join("Queue").exists() {
        katas_path.join("Queue")
    } else if katas_path.join("go_Queue").exists() {
        katas_path.join("go_Queue")
    } else if katas_path.join("python_Queue").exists() {
        katas_path.join("python_Queue")
    } else {
        panic!("No Queue kata was created");
    };

    let makefile = queue_dir.join("Makefile");
    assert!(makefile.exists(), "Makefile should be created by init");

    // Verify it has the run target
    let content = std::fs::read_to_string(&makefile)?;
    assert!(content.contains("run:"));

    cleanup(&test_katas_dir);
    Ok(())
}

#[test]
fn test_copy_creates_makefile() -> TestResult {
    let test_katas_dir = format!("{}_copy_makefile", DAY_FOLDER);
    let test_day_folder = format!("{}_copy_makefile_days", DAY_FOLDER);

    // First, init an embedded kata into katas dir
    Command::cargo_bin(PRG)?
        .args(["init", "--select", "Map"])
        .env("KATAS_DIR", &test_katas_dir)
        .assert()
        .code(0);

    // Find which variant was created
    let katas_path = std::path::Path::new(&test_katas_dir);
    let map_name = if katas_path.join("Map").exists() {
        "Map"
    } else if katas_path.join("go_Map").exists() {
        "go_Map"
    } else if katas_path.join("python_Map").exists() {
        "python_Map"
    } else {
        panic!("No Map kata was created");
    };

    // Remove the Makefile from katas dir
    let makefile_in_katas = katas_path.join(map_name).join("Makefile");
    if makefile_in_katas.exists() {
        std::fs::remove_file(&makefile_in_katas)?;
    }

    // Now copy to a day folder - it should recreate the Makefile
    Command::cargo_bin(PRG)?
        .args([map_name])
        .env("KATAS_DIR", &test_katas_dir)
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .code(0);

    // Verify Makefile was created in the day folder
    let day_kata_path = std::path::Path::new(&test_day_folder).join("day1").join(map_name);
    let makefile = day_kata_path.join("Makefile");
    assert!(makefile.exists(), "Makefile should be created when copying embedded kata");

    cleanup(&test_katas_dir);
    cleanup(&test_day_folder);
    Ok(())
}

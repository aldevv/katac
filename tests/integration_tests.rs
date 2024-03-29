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

    Command::cargo_bin(PRG)?
        .args(["run", "baz"])
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout(
            r#"
> Running baz [1/1]
--------------------
console.log("hello world");
"#,
        );
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

    Command::cargo_bin(PRG)?
        .args(["run", "foo"])
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout(
            r#"
> Running foo [1/1]
--------------------
No Makefile found in tests/day_test_run_no_makefile/day1/foo
"#,
        );
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
                assert!(run_output.contains("No Makefile found in tests/day_test_run_all/day1"));
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

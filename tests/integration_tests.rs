mod path;

use assert_cmd::Command;
use path::global_config_path;

use std::error::Error;
const DAY_FOLDER: &str = "tests/day_test";
const PRG: &str = "katac";

struct Test {
    day_folder: String,
    katas_dir: String,
}
impl Drop for Test {
    fn drop(&mut self) {
        if !self.day_folder.is_empty() {
            std::fs::remove_dir_all(&self.day_folder).unwrap();
        }

        if !self.katas_dir.is_empty() {
            std::fs::remove_dir_all(&self.katas_dir).unwrap();
        }

        let config_path = global_config_path();
        if config_path.exists() {
            std::fs::remove_file(global_config_path()).unwrap_or_default();
        }
    }
}

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn test_copy_kata() -> TestResult {
    let t = Test {
        katas_dir: "".to_string(),
        day_folder: format!("{}_copy", DAY_FOLDER),
    };
    Command::cargo_bin(PRG)?
        .args([
            "foo",
            "--days-dir",
            &t.day_folder,
            "--katas-dir",
            "tests/example_katas",
        ])
        .assert()
        .stdout("Copying foo to day1...\n");
    Ok(())
}

#[test]
fn test_run_kata() -> TestResult {
    let t = Test {
        katas_dir: "".to_string(),
        day_folder: format!("{}_run", DAY_FOLDER),
    };
    Command::cargo_bin(PRG)?
        .args(["--state", "false", "baz"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &t.day_folder)
        .assert()
        .stdout("Copying baz to day1...\n");

    Command::cargo_bin(PRG)?
        .args(["--state", "false", "run", "baz"])
        .env("DAYS_DIR", &t.day_folder)
        .assert()
        .stdout(
            r#"
> Running baz [1/1]
--------------------
console.log("hello world");
"#,
        );
    Ok(())
}

#[test]
fn test_multiple_kata() -> TestResult {
    let t = Test {
        katas_dir: "".to_string(),
        day_folder: format!("{}_multiple", DAY_FOLDER),
    };
    Command::cargo_bin(PRG)?
        .args([
            "--state",
            "false",
            "--days-dir",
            &t.day_folder,
            "--katas-dir",
            "tests/example_katas",
            "foo",
            "bar",
            "baz",
        ])
        .assert()
        .stdout("Copying foo to day1...\nCopying bar to day1...\nCopying baz to day1...\n");
    Ok(())
}

#[test]
fn test_random_with_config_file() -> TestResult {
    std::fs::write(
        "tests/katac.json",
        r#"
{
    "random": [
        "foo",
        "bar",
        "baz"
    ]
}
"#,
    )
    .expect("Unable to write config file");

    let t = Test {
        katas_dir: "".to_string(),
        day_folder: format!("{}_random_with_config", DAY_FOLDER),
    };
    let katas = ["foo", "bar", "baz"];
    for _ in 0..5 {
        Command::cargo_bin(PRG)?
            .args([
                "--state",
                "false",
                "--days-dir",
                &t.day_folder,
                "--katas-dir",
                "tests/example_katas",
                "--config-file",
                "tests/katac.json",
                "random",
                "2",
            ])
            .assert()
            .code(0);

        let day_folder = std::path::Path::new(&t.day_folder).join("day1");
        assert!(day_folder.read_dir().unwrap().count() == 2);

        for f in day_folder.read_dir().unwrap() {
            let folder = f.unwrap();
            let folder_name = folder.file_name().into_string().unwrap();
            assert!(katas.contains(&folder_name.as_str()));
        }
    }
    std::fs::remove_file("tests/katac.json").unwrap();
    Ok(())
}

#[test]
fn test_random_no_config_file() -> TestResult {
    let t = Test {
        katas_dir: "".to_string(),
        day_folder: format!("{}_random_no_config", DAY_FOLDER),
    };

    let katas = ["foo", "bar", "baz"];
    for _ in 0..5 {
        Command::cargo_bin(PRG)?
            .args([
                "--state",
                "false",
                "--days-dir",
                &t.day_folder,
                "--katas-dir",
                "tests/example_katas",
                "--config-file",
                "none",
                "random",
                "2",
            ])
            .assert()
            .code(0);

        let day_folder = std::path::Path::new(&t.day_folder).join("day1");
        assert!(day_folder.read_dir().unwrap().count() == 2);

        for f in day_folder.read_dir().unwrap() {
            let folder = f.unwrap();
            let folder_name = folder.file_name().into_string().unwrap();
            assert!(katas.contains(&folder_name.as_str()));
        }
    }
    Ok(())
}

#[test]
fn test_run_kata_no_makefile() -> TestResult {
    let t = Test {
        katas_dir: "".to_string(),
        day_folder: format!("{}_run_no_makefile", DAY_FOLDER),
    };
    Command::cargo_bin(PRG)?
        .args(["--state", "false", "foo"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &t.day_folder)
        .assert()
        .stdout("Copying foo to day1...\n");

    // remove the Makefile
    let day1 = std::path::Path::new(&t.day_folder).join("day1");
    std::fs::remove_file(day1.join("foo").join("Makefile")).unwrap();

    Command::cargo_bin(PRG)?
        .args(["--state", "false", "run", "foo"])
        .env("DAYS_DIR", &t.day_folder)
        .assert()
        .stdout(
            r#"
> Running foo [1/1]
--------------------
No Makefile found in tests/day_test_run_no_makefile/day1/foo
"#,
        );
    Ok(())
}

#[test]
fn test_run_all() -> TestResult {
    let t = Test {
        katas_dir: "".to_string(),
        day_folder: format!("{}_run_all", DAY_FOLDER),
    };
    let cmd = Command::cargo_bin(PRG)?
        .args(["--state", "false", "foo", "bar", "baz"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &t.day_folder)
        .assert();

    let copy_output = String::from_utf8(cmd.get_output().stdout.clone())?;
    for s in ["foo", "bar", "baz"].iter() {
        println!("{}", copy_output);
        assert!(copy_output.contains(&format!("Copying {} to day1...", s)));
    }

    let cmd = Command::cargo_bin(PRG)?
        .args(["--state", "false", "run"])
        .env("DAYS_DIR", &t.day_folder)
        .assert();
    let run_output = String::from_utf8(cmd.get_output().stdout.clone())?;
    println!("run_output: {}", run_output);
    for s in ["foo", "bar", "baz"].iter() {
        assert!(run_output.contains(&format!("> Running {}", s)));
        match s {
            &"foo" | &"bar" => {
                assert!(run_output.contains("TODO: add your run command here"));
            }
            &"baz" => {
                assert!(run_output.contains("console.log(\"hello world\")"));
            }
            _ => {}
        }
    }
    Ok(())
}

#[test]
fn test_add_kata() -> TestResult {
    let t = Test {
        katas_dir: "tests/new_katas".to_string(),
        day_folder: "".to_string(),
    };
    Command::cargo_bin(PRG)?
        .args(["--state", "false", "add", "foo2"])
        .env("KATAS_DIR", &t.katas_dir)
        .assert()
        .stdout("foo2 created in tests/new_katas.\n");
    Ok(())
}

#[test]
fn test_add_kata_already_exists() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(["add", "foo"])
        .env("KATAS_DIR", "tests/example_katas")
        .assert()
        .stdout("Kata foo already exists\n");
    Ok(())
}

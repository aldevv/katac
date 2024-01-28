use assert_cmd::Command;
const DAY_FOLDER: &str = "tests/day_test";
const PRG: &str = "katac";

fn cleanup(day_folder: &String) {
    std::fs::remove_dir_all(&day_folder).unwrap();
}

#[test]
fn test_copy_kata() {
    let test_day_folder = format!("{}_copy", DAY_FOLDER);
    Command::new(PRG)
        .args(&[
            "foo",
            "--days-dir",
            &test_day_folder,
            "--katas-dir",
            "tests/example_katas",
        ])
        .assert()
        .stdout("Copying foo to day1...\n");
    cleanup(&test_day_folder);
}

#[test]
fn test_run_kata() {
    let test_day_folder = format!("{}_run", DAY_FOLDER);
    Command::new(PRG)
        .args(&["baz"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout("Copying baz to day1...\n");

    Command::new(PRG)
        .args(&["run", "baz"])
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout(
            r#"
> Running baz [1/1]
_______________________
console.log("hello world");
"#,
        );
    cleanup(&test_day_folder);
}

#[test]
fn test_multiple_kata() {
    let test_day_folder = format!("{}_multiple", DAY_FOLDER);
    Command::new(PRG)
        .args(&[
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
}

#[test]
fn test_random_with_config_file() {
    std::fs::write(
        "tests/katac.toml",
        r#"
[katas]
random = ["foo", "bar", "baz"]
"#,
    )
    .expect("Unable to write config file");

    let test_day_folder = format!("{}_random_with_config", DAY_FOLDER);
    let katas = vec!["foo", "bar", "baz"];
    for _ in 0..5 {
        Command::new(PRG)
            .args(&[
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
}

#[test]
fn test_random_no_config_file() {
    let test_day_folder = format!("{}_random_no_config", DAY_FOLDER);

    let katas = vec!["foo", "bar", "baz"];
    for _ in 0..5 {
        Command::new(PRG)
            .args(&[
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
}

#[test]
fn test_run_kata_no_makefile() {
    let test_day_folder = format!("{}_run_no_makefile", DAY_FOLDER);
    Command::new(PRG)
        .args(&["foo"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout("Copying foo to day1...\n");

    Command::new(PRG)
        .args(&["run", "foo"])
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout(
            r#"
> Running foo [1/1]
_______________________
No Makefile found in tests/day_test_run_no_makefile/day1/foo
"#,
        );
    cleanup(&test_day_folder);
}

#[test]
fn test_run_all() {
    let test_day_folder = format!("{}_run_all", DAY_FOLDER);
    Command::new(PRG)
        .args(&["foo", "bar", "baz"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout(
            r#"Copying foo to day1...
Copying bar to day1...
Copying baz to day1...
"#,
        );

    Command::new(PRG)
        .args(&["run"])
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout(
            r#"
> Running foo [1/3]
_______________________
No Makefile found in tests/day_test_run_all/day1/foo

> Running baz [2/3]
_______________________
console.log("hello world");

> Running bar [3/3]
_______________________
No Makefile found in tests/day_test_run_all/day1/bar
"#,
        );
    cleanup(&test_day_folder);
}

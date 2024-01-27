use assert_cmd::Command;
const DAY_FOLDER: &str = "tests/day_test";

#[test]
fn test_copy_kata() {
    let test_day_folder = format!("{}_copy", DAY_FOLDER);
    Command::new("katac")
        .args(&[
            "hello_world",
            "--days-dir",
            &test_day_folder,
            "--katas-dir",
            "tests/example_katas",
        ])
        .assert()
        .stdout("Copying hello_world to day1...\n");
    std::fs::remove_dir_all(&test_day_folder).unwrap();
}

#[test]
fn test_run_kata() {
    let test_day_folder = format!("{}_run", DAY_FOLDER);
    Command::new("katac")
        .args(&["hello_world"])
        .env("KATAS_DIR", "tests/example_katas")
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout("Copying hello_world to day1...\n");

    Command::new("katac")
        .args(&["run", "hello_world"])
        .env("DAYS_DIR", &test_day_folder)
        .assert()
        .stdout(
            r#"
> Running hello_world [1/1]
_______________________
console.log("hello world");
"#,
        );
    std::fs::remove_dir_all(&test_day_folder).unwrap();
}

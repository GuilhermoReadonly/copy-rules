use std::process::Command;
use std::path::Path;
use std::fs;
use std::io;

const TEST_DIR: &str = "./tests/test_dir/";
const DIR_1: &str = "test_dir_1/";
const TEST_DIR_1: &str = format!("{}{}",TEST_DIR, DIR_1).to_borowed();
const TEST_DIR_2: &str = "./tests/test_dir/test_dir_2/";
const TEST_DIR_RESULT: &str = "./tests/test_dir/test_dir_2/test_dir_1/";
const TEST_FILE: &str = "./tests/test_dir/test_dir_1/test.txt";
const TEST_FILE_RESULT: &str = "./tests/test_dir/test_dir_2/test_dir_1/test.txt";

#[test]
fn test_with_basic_config() {
    let result_setup = setup();
    assert!(result_setup.is_ok(), "##### {:#?}", result_setup);

    let mut copy_rules = Command::new("./target/debug/copy_rules");
    copy_rules.arg("./tests/config.test.json");

    copy_rules.status().expect("process failed to execute");

    assert!(Path::new(TEST_DIR_RESULT).exists());
    assert!(Path::new(TEST_FILE_RESULT).exists());

    let result_clean = clean();
    assert!(result_clean.is_ok());
}

fn setup() -> Result<(), io::Error>{
    fs::create_dir_all(TEST_DIR_1)?;
    fs::OpenOptions::new().create(true).write(true).open(TEST_FILE)?;
    fs::create_dir_all(TEST_DIR_2)?;
    Ok(())
}

fn clean() -> Result<(), io::Error>{
    fs::remove_dir_all(TEST_DIR)?;
    Ok(())
}

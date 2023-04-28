use assert_cmd::cmd::Command; // Add methods on commands
use tempdir::TempDir;
use std::fs;
use std::ffi::OsString;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_cli_file_rotation() -> TestResult {
    let dir = TempDir::new("test")?;
    let mut cmd = Command::cargo_bin("logrotator")?;
    let assert = cmd.args(["--dir", dir.path().as_os_str().to_str().unwrap(), "--num-files", "2", "--size-files", "0.00001"]) // 10 bytes per file
        .write_stdin("000000000000000\n1111111111")
        .assert()
        .success();
    let output = assert.get_output();
    println!("stdout {:?}", std::str::from_utf8(&(output.stdout))?);
    let mut filenames = fs::read_dir(dir.path())?.map(|e| e.unwrap().file_name()).collect::<Vec<_>>();
    filenames.sort();
    println!("filenames {:?}", filenames);
    assert!(filenames == vec![OsString::from("log0"), OsString::from("log1")]);
    Ok(())
}
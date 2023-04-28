use crate::{Args, create_log_file};
use std::{fs, io::Write};
use tempdir::TempDir;

#[test]
fn test_file_name_create() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new("test")?;
    println!("tmpdir path {:?}", dir.path());
    let args = Args {
        num_files: 1,
        size_files: 1.0,
        dir: dir.path().into(),
    };
    let mut file = create_log_file(&args, 1);
    file.write(String::from("foo").as_bytes())?;
    file.flush()?;
    if let Ok(dir_entry) = fs::read_dir(dir.path())?.next().unwrap() {
        println!("found dir entry with file name {:?}", dir_entry.file_name());
        assert!(dir_entry.file_name() == "log1");
    }
    Ok(())
}

pub mod cli;
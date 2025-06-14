use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.txt");
    
    let mut file = File::create(&file_path)?;
    writeln!(file, "This is a test file")?;
    writeln!(file, "with multiple lines")?;
    writeln!(file, "and some test content")?;

    let mut cmd = Command::cargo_bin("blink")?;
    cmd.arg("test")
        .arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test file"))
        .stdout(predicate::str::contains("test content"));

    Ok(())
}

#[test]
fn binary_file_detection() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("binary.bin");
    
    let mut file = File::create(&file_path)?;
    // Write a larger binary file with multiple null bytes
    let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    file.write_all(&data)?;

    let mut cmd = Command::cargo_bin("blink")?;
    cmd.arg("test")
        .arg(&file_path);
    
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Binary file detected:"))
        .stderr(predicate::str::contains("binary.bin"));

    Ok(())
} 
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
use std::path::Path;

// Helper function to create a new command instance
fn cmd() -> Result<Command, Box<dyn std::error::Error>> {
    Ok(Command::cargo_bin("blink")?)
}

mod basic_search {
    use super::*;

    #[test]
    fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        
        let mut file = File::create(&file_path)?;
        writeln!(file, "This is a test file")?;
        writeln!(file, "with multiple lines")?;
        writeln!(file, "and some test content")?;

        cmd()?
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test file"))
            .stdout(predicate::str::contains("test content"));

        Ok(())
    }

    #[test]
    fn case_sensitive_search() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        
        let mut file = File::create(&file_path)?;
        writeln!(file, "TEST content")?;
        writeln!(file, "test content")?;

        cmd()?
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test content"))
            .stdout(predicate::str::contains("test content").count(1));

        Ok(())
    }
}

mod binary_handling {
    use super::*;

    #[test]
    fn binary_file_detection() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("binary.bin");
        
        let mut file = File::create(&file_path)?;
        // Write a larger binary file with multiple null bytes
        let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
        file.write_all(&data)?;

        cmd()?
            .arg("test")
            .arg(&file_path)
            .assert()
            .success()
            .stderr(predicate::str::contains("Binary file detected:"))
            .stderr(predicate::str::contains("binary.bin"));

        Ok(())
    }

    #[test]
    fn skip_binary_search_results() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create a text file
        let text_path = temp_dir.path().join("text.txt");
        let mut text_file = File::create(&text_path)?;
        writeln!(text_file, "test content")?;

        // Create a binary file
        let bin_path = temp_dir.path().join("data.bin");
        let mut bin_file = File::create(&bin_path)?;
        bin_file.write_all(&[0u8; 100])?;

        cmd()?
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("text.txt"))
            .stdout(predicate::str::contains("data.bin").count(0))
            .stderr(predicate::str::contains("Binary file detected:"));

        Ok(())
    }
}

mod file_exclusion {
    use super::*;

    #[test]
    fn exclude_single_directory() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create a file in main directory
        let main_file = temp_dir.path().join("main.txt");
        let mut file = File::create(&main_file)?;
        writeln!(file, "test in main")?;

        // Create a file in benchmark directory
        let bench_dir = temp_dir.path().join("benchmark");
        std::fs::create_dir(&bench_dir)?;
        let bench_file = bench_dir.join("bench.txt");
        let mut file = File::create(&bench_file)?;
        writeln!(file, "test in benchmark")?;

        cmd()?
            .arg("test")
            .arg("-x")
            .arg("benchmark")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("main.txt"))
            .stdout(predicate::str::contains("bench.txt").count(0));

        Ok(())
    }

    #[test]
    fn exclude_multiple_directories() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create files in different directories
        let main_file = temp_dir.path().join("main.txt");
        let mut file = File::create(&main_file)?;
        writeln!(file, "test in main")?;

        let test_dir = temp_dir.path().join("test");
        std::fs::create_dir(&test_dir)?;
        let test_file = test_dir.join("test.txt");
        let mut file = File::create(&test_file)?;
        writeln!(file, "test in test")?;

        let bench_dir = temp_dir.path().join("benchmark");
        std::fs::create_dir(&bench_dir)?;
        let bench_file = bench_dir.join("bench.txt");
        let mut file = File::create(&bench_file)?;
        writeln!(file, "test in benchmark")?;

        cmd()?
            .arg("test")
            .arg("-x")
            .arg("benchmark,test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("main.txt"))
            .stdout(predicate::str::contains("test.txt").count(0))
            .stdout(predicate::str::contains("bench.txt").count(0));

        Ok(())
    }

    #[test]
    fn combine_exclude_with_extensions() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create various files
        let main_rs = temp_dir.path().join("main.rs");
        let mut file = File::create(&main_rs)?;
        writeln!(file, "test in rust")?;

        let main_txt = temp_dir.path().join("main.txt");
        let mut file = File::create(&main_txt)?;
        writeln!(file, "test in text")?;

        let test_dir = temp_dir.path().join("test");
        std::fs::create_dir(&test_dir)?;
        let test_rs = test_dir.join("test.rs");
        let mut file = File::create(&test_rs)?;
        writeln!(file, "test in test rust")?;

        cmd()?
            .arg("test")
            .arg("-x")
            .arg("test")
            .arg("-e")
            .arg("rs")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("main.rs"))
            .stdout(predicate::str::contains("main.txt").count(0))
            .stdout(predicate::str::contains("test.rs").count(0));

        Ok(())
    }
}

mod case_sensitivity {
    use super::*;

    #[test]
    fn case_sensitive_by_default() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        
        let mut file = File::create(&file_path)?;
        writeln!(file, "TEST PATTERN")?;
        writeln!(file, "test pattern")?;
        writeln!(file, "Test Pattern")?;

        cmd()?
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test pattern"))
            .stdout(predicate::str::contains("TEST PATTERN").count(0))
            .stdout(predicate::str::contains("Test Pattern").count(0));

        Ok(())
    }

    #[test]
    fn case_insensitive_search() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        
        let mut file = File::create(&file_path)?;
        writeln!(file, "TEST PATTERN")?;
        writeln!(file, "test pattern")?;
        writeln!(file, "Test Pattern")?;

        cmd()?
            .arg("-i")
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test pattern"))
            .stdout(predicate::str::contains("TEST PATTERN"))
            .stdout(predicate::str::contains("Test Pattern"));

        Ok(())
    }
}

mod hidden_files {
    use super::*;

    fn create_hidden_file(dir: &Path, name: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = dir.join(name);
        let mut file = File::create(&file_path)?;
        writeln!(file, "{}", content)?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o400);
            std::fs::set_permissions(&file_path, perms)?;
        }
        
        Ok(())
    }

    #[test]
    fn skip_hidden_by_default() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create regular file
        let mut file = File::create(temp_dir.path().join("normal.txt"))?;
        writeln!(file, "test in normal file")?;

        // Create hidden file
        create_hidden_file(temp_dir.path(), ".hidden.txt", "test in hidden file")?;

        cmd()?
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("normal.txt"))
            .stdout(predicate::str::contains(".hidden.txt").count(0));

        Ok(())
    }

    #[test]
    fn include_hidden_with_flag() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create regular file
        let mut file = File::create(temp_dir.path().join("normal.txt"))?;
        writeln!(file, "test in normal file")?;

        // Create hidden file
        create_hidden_file(temp_dir.path(), ".hidden.txt", "test in hidden file")?;

        cmd()?
            .arg("-H")
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("normal.txt"))
            .stdout(predicate::str::contains(".hidden.txt"));

        Ok(())
    }
}

mod extension_filtering {
    use super::*;

    #[test]
    fn filter_single_extension() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create files with different extensions
        let mut rs_file = File::create(temp_dir.path().join("code.rs"))?;
        writeln!(rs_file, "test in rust file")?;
        
        let mut txt_file = File::create(temp_dir.path().join("notes.txt"))?;
        writeln!(txt_file, "test in text file")?;

        cmd()?
            .arg("-e")
            .arg("rs")
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("code.rs"))
            .stdout(predicate::str::contains("notes.txt").count(0));

        Ok(())
    }

    #[test]
    fn filter_multiple_extensions() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create files with different extensions
        let mut rs_file = File::create(temp_dir.path().join("code.rs"))?;
        writeln!(rs_file, "test in rust file")?;
        
        let mut txt_file = File::create(temp_dir.path().join("notes.txt"))?;
        writeln!(txt_file, "test in text file")?;
        
        let mut py_file = File::create(temp_dir.path().join("script.py"))?;
        writeln!(py_file, "test in python file")?;

        cmd()?
            .arg("-e")
            .arg("rs,txt")
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("code.rs"))
            .stdout(predicate::str::contains("notes.txt"))
            .stdout(predicate::str::contains("script.py").count(0));

        Ok(())
    }

    #[test]
    fn no_matching_extensions() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        let mut file = File::create(temp_dir.path().join("test.txt"))?;
        writeln!(file, "test content")?;

        cmd()?
            .arg("-e")
            .arg("rs,py")
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::is_empty());

        Ok(())
    }

    #[test]
    fn combine_with_case_insensitive() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        let mut rs_file = File::create(temp_dir.path().join("code.rs"))?;
        writeln!(rs_file, "TEST in rust")?;
        
        let mut txt_file = File::create(temp_dir.path().join("notes.txt"))?;
        writeln!(txt_file, "TEST in text")?;

        cmd()?
            .arg("-i")
            .arg("-e")
            .arg("rs")
            .arg("test")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("code.rs"))
            .stdout(predicate::str::contains("TEST in rust"))
            .stdout(predicate::str::contains("notes.txt").count(0));

        Ok(())
    }
} 
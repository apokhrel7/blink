use crate::error::{FastFindError, Result};
use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use std::sync::Mutex;

pub struct Match {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
    pub start: usize,
    pub end: usize,
}

/// Checks if a file is likely to be binary by looking for null bytes
fn is_binary(path: &Path) -> Result<bool> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];
    
    match reader.read(&mut buffer) {
        Ok(n) if n > 0 => Ok(buffer[..n].contains(&0)),
        Ok(_) => Ok(false),
        Err(e) => Err(e.into()),
    }
}

/// Checks if an entry should be processed based on configuration
fn should_process(entry: &DirEntry, hidden: bool, extensions: &[String], exclusions: &[String]) -> bool {
    // Skip hidden files unless explicitly included
    if !hidden && entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false) {
        return false;
    }

    // Skip non-files
    if !entry.file_type().is_file() {
        return false;
    }

    // Check exclusions
    if !exclusions.is_empty() {
        let path_str = entry.path().to_string_lossy();
        if exclusions.iter().any(|excl| path_str.contains(excl)) {
            return false;
        }
    }

    // Apply extension filter if specified
    if !extensions.is_empty() {
        entry.path()
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| extensions.iter().any(|e| e == ext))
            .unwrap_or(false)
    } else {
        true
    }
}

pub fn search_files(
    pattern: &Regex,
    paths: &[PathBuf],
    hidden: bool,
    extensions: &[String],
    exclusions: &[String],
) -> Result<Vec<Match>> {
    // Collect all matching files first
    let files: Vec<PathBuf> = paths.iter()
        .flat_map(|path| {
            WalkDir::new(path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| should_process(e, hidden, extensions, exclusions))
                .map(|e| e.path().to_owned())
                .collect::<Vec<_>>()
        })
        .collect();

    // Process files in parallel
    let matches = Mutex::new(Vec::new());
    let binary_files = Mutex::new(Vec::new());

    files.par_iter()
        .try_for_each(|path| -> Result<()> {
            match search_file(pattern, path) {
                Ok(file_matches) => {
                    matches.lock().unwrap().extend(file_matches);
                    Ok(())
                },
                Err(FastFindError::BinaryFile(path)) => {
                    binary_files.lock().unwrap().push(path);
                    Ok(())
                },
                Err(e) => Err(e),
            }
        })?;

    // Report binary files
    //for path in binary_files.into_inner().unwrap() {
        //eprintln!("Binary file detected: {}", path);
    //}

    Ok(matches.into_inner().unwrap())
}

fn search_file(pattern: &Regex, path: &Path) -> Result<Vec<Match>> {
    // Check for binary files
    if is_binary(path)? {
        return Err(FastFindError::BinaryFile(path.display().to_string()));
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut matches = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) if e.kind() == std::io::ErrorKind::InvalidData => {
                // Skip lines that aren't valid UTF-8
                continue;
            }
            Err(e) => return Err(e.into()),
        };
        
        for m in pattern.find_iter(&line) {
            matches.push(Match {
                path: path.to_owned(),
                line_number: line_number + 1,
                line: line.clone(),
                start: m.start(),
                end: m.end(),
            });
        }
    }

    Ok(matches)
} 
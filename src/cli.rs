use clap::Parser;
use regex::Regex;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The pattern to search for (a regular expression)
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    /// The paths to search in (defaults to current directory)
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Perform case-insensitive matching
    #[arg(short = 'i', long)]
    pub case_insensitive: bool,

    /// Include hidden files and directories
    #[arg(short = 'H', long = "hidden")]
    pub hidden: bool,

    /// Filter by file extension (e.g., "rs,txt")
    #[arg(short = 'e', long, value_name = "EXTENSIONS")]
    pub extensions: Option<String>,

    /// Number of context lines to show before and after matches
    #[arg(short = 'C', long, default_value_t = 0)]
    pub context_lines: usize,

    /// Number of worker threads (defaults to number of CPU cores)
    #[arg(short = 'j', long)]
    pub threads: Option<usize>,
}

impl Cli {
    /// Builds a regex from the pattern, taking into account case sensitivity
    pub fn build_regex(&self) -> Result<Regex, regex::Error> {
        let mut pattern = self.pattern.clone();
        if self.case_insensitive {
            pattern = format!("(?i){}", pattern);
        }
        Regex::new(&pattern)
    }

    /// Gets the file extensions to filter by, if any
    pub fn extensions(&self) -> Vec<String> {
        self.extensions
            .as_ref()
            .map(|e| e.split(',').map(str::to_string).collect())
            .unwrap_or_default()
    }
} 
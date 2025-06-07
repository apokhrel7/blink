use clap::Parser;
use std::path::PathBuf;

mod cli;
mod error;
mod output;
mod search;

use cli::Cli;
use error::{FastFindError, Result};
use output::Printer;

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Set number of threads if specified
    if let Some(threads) = cli.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .unwrap();
    }

    // Use current directory if no paths specified
    let paths = if cli.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.paths.clone()
    };

    // Build the regex
    let pattern = cli.build_regex()?;
    
    // Get file extensions to filter
    let extensions = cli.extensions();

    // Search for matches
    match search::search_files(
        &pattern,
        &paths,
        cli.hidden,
        &extensions,
    ) {
        Ok(matches) => {
            // Print matches
            let printer = Printer::new(cli.context_lines);
            for m in matches {
                printer.print_match(&m)?;
            }
            Ok(())
        }
        Err(FastFindError::BinaryFile(path)) => {
            eprintln!("Binary file detected: {}", path);
            Ok(())
        }
        Err(e) => Err(e),
    }
} 
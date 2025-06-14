use crate::search::Match;
use colored::*;
use std::io::{self, Write};

pub struct Printer {}

impl Printer {
    pub fn new(_context_lines: usize) -> Self {
        Self {}
    }

    pub fn print_match(&self, m: &Match) -> io::Result<()> {
        let path_str = m.path.display().to_string();
        let line_num = m.line_number.to_string();
        
        // Print file path and line number in a modern format
        print!("{}", path_str.bright_blue());
        print!(":");
        print!("{}", line_num.bright_yellow());
        print!(":");
        
        // Print the matching line with the match highlighted
        let before = &m.line[..m.start];
        let matched = &m.line[m.start..m.end];
        let after = &m.line[m.end..];

        // Print parts of the line with proper highlighting
        print!("{}", before);
        print!("{}", matched.black().on_bright_green().bold());
        print!("{}", after);
        println!();

        io::stdout().flush()
    }
} 
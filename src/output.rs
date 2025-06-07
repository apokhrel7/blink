use crate::search::Match;
use colored::*;
use std::io::{self, Write};

pub struct Printer {
    context_lines: usize,
}

impl Printer {
    pub fn new(context_lines: usize) -> Self {
        Self { context_lines }
    }

    pub fn print_match(&self, m: &Match) -> io::Result<()> {
        let path_str = m.path.display().to_string();
        let line_num = m.line_number.to_string();
        
        // Print file:line header in a distinct color
        println!(
            "{}:{}:",
            path_str.bright_blue().bold(),
            line_num.bright_yellow()
        );

        // Print the matching line with the match highlighted
        let before = &m.line[..m.start];
        let matched = &m.line[m.start..m.end];
        let after = &m.line[m.end..];

        print!("{}", before);
        print!("{}", matched.on_bright_red());
        println!("{}", after);

        io::stdout().flush()
    }
} 
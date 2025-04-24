use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};

/// Console output formatting and utilities
pub struct Console;

impl Console {
    /// Print a header (section title)
    pub fn print_header(text: &str) {
        println!("\n{}", text.bold().bright_blue());
        println!("{}", "=".repeat(text.len()).bright_blue());
    }
    
    /// Print a subheader (subsection title)
    pub fn print_subheader(text: &str) {
        println!("\n{}", text.bold().bright_cyan());
        println!("{}", "-".repeat(text.len()).bright_cyan());
    }
    
    /// Print a key-value pair
    pub fn print_key_value(key: &str, value: &str) {
        println!("{}: {}", key.bold(), value);
    }
    
    /// Print a success message
    pub fn print_success(message: &str) {
        println!("{} {}", "✓".green().bold(), message.green());
    }
    
    /// Print an error message
    pub fn print_error(message: &str) {
        eprintln!("{} {}", "✗".red().bold(), message.red());
    }
    
    /// Print a warning message
    pub fn print_warning(message: &str) {
        println!("{} {}", "!".yellow().bold(), message.yellow());
    }
    
    /// Print an info message
    pub fn print_info(message: &str) {
        println!("{} {}", "ℹ".bright_blue().bold(), message);
    }
    
    /// Create a progress bar with the default style
    #[must_use] pub fn create_progress_bar(total: u64) -> ProgressBar {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .expect("Invalid progress bar template")
                .progress_chars("##-"),
        );
        pb
    }
    
    /// Format a number with K/M/B suffixes
    #[must_use] pub fn format_number(num: usize) -> String {
        if num >= 1_000_000_000 {
            format!("{:.2}B", num as f64 / 1_000_000_000.0)
        } else if num >= 1_000_000 {
            format!("{:.2}M", num as f64 / 1_000_000.0)
        } else if num >= 1_000 {
            format!("{:.2}K", num as f64 / 1_000.0)
        } else {
            num.to_string()
        }
    }
    
    /// Format a duration in a human-readable format
    #[must_use] pub fn format_duration(duration: std::time::Duration) -> String {
        let total_secs = duration.as_secs();
        
        if total_secs < 60 {
            format!("{}.{:03}s", total_secs, duration.subsec_millis())
        } else if total_secs < 3600 {
            let mins = total_secs / 60;
            let secs = total_secs % 60;
            format!("{mins}m {secs}s")
        } else {
            let hours = total_secs / 3600;
            let mins = (total_secs % 3600) / 60;
            let secs = total_secs % 60;
            format!("{hours}h {mins}m {secs}s")
        }
    }
    
    /// Prompt for user input
    pub fn prompt(message: &str) -> io::Result<String> {
        print!("{}: ", message.bold());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }
    
    /// Prompt for yes/no input
    pub fn confirm(message: &str) -> io::Result<bool> {
        print!("{} (y/n): ", message.bold());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
    }
}
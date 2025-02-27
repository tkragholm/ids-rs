use colored::{ColoredString, Colorize};

/// Console output utilities for structured information display
pub struct ConsoleOutput;

impl ConsoleOutput {
    /// Print a section header
    pub fn section(title: &str) {
        println!("\n{}", title.green().bold());
        println!("{}", "═".repeat(title.len()).green());
    }
    
    /// Print a subsection header
    pub fn subsection(title: &str) {
        println!("\n{}", title.blue().bold());
        println!("{}", "─".repeat(title.len()).blue());
    }
    
    /// Print a key-value pair with optional formatting
    pub fn key_value(key: &str, value: &str) {
        println!("{}: {}", key.bold(), value);
    }
    
    /// Print a key-value pair with colored value
    pub fn key_value_colored(key: &str, value: &str, success: bool) {
        let colored_value = if success {
            value.green()
        } else {
            value.red()
        };
        println!("{}: {}", key.bold(), colored_value);
    }
    
    /// Print a success message
    pub fn success(message: &str) {
        println!("{} {}", "✓".green().bold(), message);
    }
    
    /// Print an error message
    pub fn error(message: &str) {
        eprintln!("{} {}", "✗".red().bold(), message);
    }
    
    /// Print a warning message
    pub fn warning(message: &str) {
        println!("{} {}", "!".yellow().bold(), message);
    }
    
    /// Format a percentage with appropriate color based on value
    pub fn format_percentage(value: f64) -> ColoredString {
        let percentage = format!("{:.2}%", value * 100.0);
        if value >= 0.9 {
            percentage.green()
        } else if value >= 0.7 {
            percentage.yellow()
        } else {
            percentage.red()
        }
    }
    
    /// Format a number with appropriate units (K, M, B)
    pub fn format_number(num: usize) -> String {
        if num < 1_000 {
            num.to_string()
        } else if num < 1_000_000 {
            format!("{:.2}K", num as f64 / 1_000.0)
        } else if num < 1_000_000_000 {
            format!("{:.2}M", num as f64 / 1_000_000.0)
        } else {
            format!("{:.2}B", num as f64 / 1_000_000_000.0)
        }
    }
    
    /// Print a progress status
    pub fn status(step: usize, total: usize, description: &str) {
        let progress = format!("[{}/{}]", step, total).blue();
        println!("{} {}", progress, description);
    }
    
    /// Print a table with headers and rows
    pub fn table(headers: &[&str], rows: &[Vec<String>]) {
        // Determine column widths
        let mut widths = headers.iter().map(|h| h.len()).collect::<Vec<_>>();
        
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        
        // Print headers
        print!("│ ");
        for (i, header) in headers.iter().enumerate() {
            let padding = " ".repeat(widths[i].saturating_sub(header.len()));
            print!("{}{} │ ", header.bold(), padding);
        }
        println!();
        
        // Print separator
        print!("├─");
        #[allow(clippy::needless_range_loop)]
        for i in 0..headers.len() {
            print!("{}┼─", "─".repeat(widths[i] + 1));
        }
        println!();
        
        // Print rows
        for row in rows {
            print!("│ ");
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    let padding = " ".repeat(widths[i].saturating_sub(cell.len()));
                    print!("{}{} │ ", cell, padding);
                }
            }
            println!();
        }
    }
}

/// Timer formatting utilities
pub fn format_duration_short(duration: std::time::Duration) -> String {
    let total_secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if total_secs == 0 {
        format!("{}ms", millis)
    } else if total_secs < 60 {
        format!("{}.{:03}s", total_secs, millis)
    } else {
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        if mins < 60 {
            format!("{}m {}s", mins, secs)
        } else {
            let hours = mins / 60;
            let mins = mins % 60;
            format!("{}h {}m", hours, mins)
        }
    }
}
use crate::error::{logging_error, Context, Result};
use colored::Colorize;
use log::{Level, LevelFilter, Record};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

/// Simple logger that writes to both console and a file
pub struct SimpleLogger {
    log_file: Option<File>,
    console_level: LevelFilter,
    file_level: LevelFilter,
}

impl SimpleLogger {
    /// Create a new logger with the specified log file and level filters
    pub fn new(
        log_path: Option<&Path>,
        console_level: LevelFilter,
        file_level: LevelFilter,
    ) -> Result<Self> {
        let log_file = if let Some(path) = log_path {
            // Create the directory structure if it doesn't exist
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create log directory: {parent:?}"))?;
            }

            Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .with_context(|| format!("Failed to open log file: {path:?}"))?,
            )
        } else {
            None
        };

        Ok(Self {
            log_file,
            console_level,
            file_level,
        })
    }

    /// Write to the log file if configured
    fn write_to_file(&mut self, record: &Record) {
        if let Some(ref mut file) = self.log_file {
            if record.level() <= self.file_level {
                let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
                let message = format!(
                    "[{}] {} [{}:{}] {}\n",
                    timestamp,
                    record.level(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.args()
                );

                let _ = file.write_all(message.as_bytes());
                let _ = file.flush();
            }
        }
    }

    /// Write to the console if the level matches
    fn write_to_console(&self, record: &Record) {
        if record.level() <= self.console_level {
            match record.level() {
                Level::Error => eprintln!("{} {}", "ERROR:".bright_red().bold(), record.args()),
                Level::Warn => eprintln!("{} {}", "WARN:".yellow().bold(), record.args()),
                Level::Info => println!("{} {}", "INFO:".bright_blue().bold(), record.args()),
                Level::Debug => println!("{} {}", "DEBUG:".bright_cyan(), record.args()),
                Level::Trace => println!("{} {}", "TRACE:".dimmed(), record.args()),
            }
        }
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.console_level || metadata.level() <= self.file_level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut logger = Self {
                log_file: None,
                console_level: self.console_level,
                file_level: self.file_level,
            };

            if let Some(ref file) = self.log_file {
                if let Ok(fd) = file.try_clone() {
                    logger.log_file = Some(fd);
                }
            }

            logger.write_to_file(record);
            self.write_to_console(record);
        }
    }

    fn flush(&self) {
        if let Some(ref file) = self.log_file {
            let _ = file.sync_all();
        }
    }
}

/// Configure and initialize the logger
pub fn setup_logger(
    log_file: Option<&Path>,
    console_level: LevelFilter,
    file_level: LevelFilter,
) -> Result<()> {
    let logger = SimpleLogger::new(log_file, console_level, file_level)?;

    // Initialize the global logger
    match log::set_boxed_logger(Box::new(logger)) {
        Ok(()) => {
            // Set the maximum log level based on both console and file levels
            log::set_max_level(std::cmp::max(console_level, file_level));
            log::info!(
                "Logger initialized with console level: {console_level:?}, file level: {file_level:?}"
            );
            Ok(())
        }
        Err(_) => Err(logging_error(
            "Failed to initialize logger: logger already set",
        )),
    }
}

/// A performance timer for logging execution times
pub struct PerformanceTimer {
    name: String,
    start: Instant,
    checkpoints: Vec<(String, Duration)>,
    silent: bool,
}

impl PerformanceTimer {
    /// Create a new performance timer with the given operation name
    #[must_use] pub fn new(operation_name: &str) -> Self {
        Self {
            name: operation_name.to_string(),
            start: Instant::now(),
            checkpoints: Vec::new(),
            silent: false,
        }
    }

    /// Create a new performance timer that doesn't log its results
    #[must_use] pub fn silent(operation_name: &str) -> Self {
        Self {
            name: operation_name.to_string(),
            start: Instant::now(),
            checkpoints: Vec::new(),
            silent: true,
        }
    }

    /// Record a checkpoint in the timer
    pub fn checkpoint(&mut self, checkpoint_name: &str) {
        self.checkpoints
            .push((checkpoint_name.to_string(), self.start.elapsed()));
    }

    /// Complete the timer and log the performance data
    pub fn finish(&mut self) -> Duration {
        let total_duration = self.start.elapsed();

        if !self.silent {
            // Log the total time
            log::debug!(
                "{} {} {} {}",
                "PERF:".bright_magenta().bold(),
                self.name.yellow(),
                "completed in".dimmed(),
                format!("{total_duration:.2?}").green()
            );

            // Log checkpoints if any
            if !self.checkpoints.is_empty() {
                let mut checkpoint_logs = Vec::new();
                let mut last_time = Duration::from_secs(0);

                for (name, time) in &self.checkpoints {
                    let segment_duration = *time - last_time;
                    checkpoint_logs.push(format!("{}: {:.2?}", name.blue(), segment_duration));
                    last_time = *time;
                }

                // Add the final segment if there's a gap
                if last_time < total_duration {
                    let final_segment = total_duration - last_time;
                    checkpoint_logs.push(format!("{}: {:.2?}", "final".blue(), final_segment));
                }

                log::debug!(
                    "{} {} {}",
                    "PERF:".bright_magenta().bold(),
                    self.name.yellow(),
                    format!("checkpoints: {}", checkpoint_logs.join(", ")).dimmed()
                );
            }
        }

        total_duration
    }

    /// Get the elapsed time without finishing the timer
    #[must_use] pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Format a time duration in a human-readable format
    #[must_use] pub fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs();
        let millis = duration.subsec_millis();

        if secs == 0 {
            format!("{millis} ms")
        } else if secs < 60 {
            format!("{secs}.{millis:03} s")
        } else {
            let mins = secs / 60;
            let secs = secs % 60;
            format!("{mins}m {secs}s")
        }
    }
}

/// Shorthand function to time a closure and log its execution time
pub fn time_operation<F, T>(operation_name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let timer = Instant::now();
    let result = f();
    let duration = timer.elapsed();

    log::debug!(
        "{} {} {} {}",
        "PERF:".bright_magenta().bold(),
        operation_name.yellow(),
        "completed in".dimmed(),
        format!("{duration:.2?}").green()
    );

    result
}

/// Measure memory usage before and after an operation (currently stub - platform dependent)
pub fn measure_memory_usage<F, T>(operation_name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    // In a real implementation, we would measure memory before

    let result = f();

    // And measure memory after, then log the difference
    // This is platform dependent and would require additional dependencies

    log::debug!(
        "{} {} {}",
        "MEM:".bright_cyan().bold(),
        operation_name.yellow(),
        "(Memory measurement not implemented)".dimmed()
    );

    result
}

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
        let colored_value = if success { value.green() } else { value.red() };
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
    #[must_use] pub fn format_percentage(value: f64) -> colored::ColoredString {
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
    #[must_use] pub fn format_number(num: usize) -> String {
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
        let progress = format!("[{step}/{total}]").blue();
        println!("{progress} {description}");
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
        for (_, width) in widths.iter().enumerate().take(headers.len()) {
            print!("{}┼─", "─".repeat(width + 1));
        }
        println!();

        // Print rows
        for row in rows {
            print!("│ ");
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    let padding = " ".repeat(widths[i].saturating_sub(cell.len()));
                    print!("{cell}{padding} │ ");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, error, info, trace, warn};
    use std::io::Read;
    use std::thread::sleep;
    use tempfile::NamedTempFile;

    #[test]
    fn test_logger_levels() {
        let temp_file = NamedTempFile::new().unwrap();

        // Initialize logger with different levels for console and file
        let result = setup_logger(
            Some(temp_file.path()),
            LevelFilter::Info,  // Console: only info and higher
            LevelFilter::Debug, // File: debug and higher
        );

        assert!(result.is_ok());

        // Log messages at different levels
        error!("This is an error");
        warn!("This is a warning");
        info!("This is an info message");
        debug!("This is a debug message");
        trace!("This is a trace message");

        // Verify log contents
        let mut file_content = String::new();
        let mut file = OpenOptions::new()
            .read(true)
            .open(temp_file.path())
            .unwrap();

        file.read_to_string(&mut file_content).unwrap();

        // Check that error, warn, info, and debug messages are in the file
        assert!(file_content.contains("ERROR"));
        assert!(file_content.contains("WARN"));
        assert!(file_content.contains("INFO"));
        assert!(file_content.contains("DEBUG"));

        // Check that trace messages are NOT in the file
        assert!(!file_content.contains("TRACE"));
    }

    #[test]
    fn test_performance_timer() {
        // Setup logging to see the output
        let temp_file = NamedTempFile::new().unwrap();
        let _ = setup_logger(
            Some(temp_file.path()),
            LevelFilter::Debug,
            LevelFilter::Debug,
        );

        let mut timer = PerformanceTimer::new("test_operation");

        // Simulate some work
        sleep(Duration::from_millis(10));
        timer.checkpoint("step1");

        sleep(Duration::from_millis(20));
        timer.checkpoint("step2");

        sleep(Duration::from_millis(15));
        let duration = timer.finish();

        assert!(
            duration.as_millis() >= 45,
            "Timer should record at least 45ms"
        );

        // Test the time_operation helper
        let result = time_operation("simple_add", || {
            sleep(Duration::from_millis(10));
            2 + 2
        });

        assert_eq!(result, 4);

        // Test memory measurement stub
        let result = measure_memory_usage("memory_test", || {
            let mut v = Vec::new();
            for i in 0..1000 {
                v.push(i);
            }
            v.len()
        });

        assert_eq!(result, 1000);
    }
}

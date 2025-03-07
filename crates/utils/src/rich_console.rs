use console::{Style, Term};
use std::fmt::Display;
use std::io;

/// Rich Console UI for enhanced terminal output
/// 
/// This module provides a Rich-like terminal UI experience with panels, tables,
/// and styled output. It's designed to improve the aesthetics of CLI applications.
pub struct RichConsole {
    term: Term,
    width: u16,
}

/// Alignment options for text in tables and panels
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// Box drawing characters for different box styles
pub struct BoxChars {
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub horizontal: &'static str,
    pub vertical: &'static str,
    pub t_down: &'static str,
    pub t_up: &'static str,
    pub t_right: &'static str,
    pub t_left: &'static str,
    pub cross: &'static str,
}

/// Predefined box styles
pub enum BoxStyle {
    Single,
    Double,
    Rounded,
    Bold,
    Minimal,
}

impl BoxChars {
    /// Get a set of box drawing characters based on a style
    pub fn from_style(style: BoxStyle) -> Self {
        match style {
            BoxStyle::Single => Self {
                top_left: "┌",
                top_right: "┐",
                bottom_left: "└",
                bottom_right: "┘",
                horizontal: "─",
                vertical: "│",
                t_down: "┬",
                t_up: "┴",
                t_right: "├",
                t_left: "┤",
                cross: "┼",
            },
            BoxStyle::Double => Self {
                top_left: "╔",
                top_right: "╗",
                bottom_left: "╚",
                bottom_right: "╝",
                horizontal: "═",
                vertical: "║",
                t_down: "╦",
                t_up: "╩",
                t_right: "╠",
                t_left: "╣",
                cross: "╬",
            },
            BoxStyle::Rounded => Self {
                top_left: "╭",
                top_right: "╮",
                bottom_left: "╰",
                bottom_right: "╯",
                horizontal: "─",
                vertical: "│",
                t_down: "┬",
                t_up: "┴",
                t_right: "├",
                t_left: "┤",
                cross: "┼",
            },
            BoxStyle::Bold => Self {
                top_left: "┏",
                top_right: "┓",
                bottom_left: "┗",
                bottom_right: "┛",
                horizontal: "━",
                vertical: "┃",
                t_down: "┳",
                t_up: "┻",
                t_right: "┣",
                t_left: "┫",
                cross: "╋",
            },
            BoxStyle::Minimal => Self {
                top_left: "┌",
                top_right: "┐",
                bottom_left: "└",
                bottom_right: "┘",
                horizontal: "─",
                vertical: "│",
                t_down: "┬",
                t_up: "┴",
                t_right: "├",
                t_left: "┤",
                cross: "┼",
            },
        }
    }
}

impl RichConsole {
    /// Create a new RichConsole instance
    pub fn new() -> Self {
        let term = Term::stdout();
        let (width, _) = term.size();
        Self {
            term,
            width,
        }
    }

    /// Create a new RichConsole instance for stderr
    pub fn for_stderr() -> Self {
        let term = Term::stderr();
        let (width, _) = term.size();
        Self {
            term,
            width,
        }
    }

    /// Get the current terminal width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Force set a different terminal width (useful for testing or constrained output)
    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    /// Refresh the terminal size
    pub fn refresh_size(&mut self) {
        let (width, _) = self.term.size();
        self.width = width;
    }

    /// Clear the current line
    pub fn clear_line(&self) -> io::Result<()> {
        self.term.clear_line()
    }

    /// Move the cursor up by n lines
    pub fn move_cursor_up(&self, n: usize) -> io::Result<()> {
        self.term.move_cursor_up(n)
    }

    /// Move the cursor down by n lines
    pub fn move_cursor_down(&self, n: usize) -> io::Result<()> {
        self.term.move_cursor_down(n)
    }

    /// Draw a horizontal rule with optional title
    pub fn rule<S: AsRef<str>>(&self, title: Option<S>) -> io::Result<()> {
        let width = self.width as usize;
        
        // Default style for the rule
        let rule_style = Style::new().dim();
        let title_style = Style::new().bold();
        
        // Character to use for the rule
        let rule_char = "─";
        
        match title {
            Some(title) => {
                let title = title.as_ref();
                let title_len = console::measure_text_width(title);
                
                // Calculate padding on each side of the title
                let padding = (width - title_len) / 2;
                if padding <= 0 {
                    // Title is too long, just print it
                    self.term.write_line(&format!(" {} ", title_style.apply_to(title)))?;
                } else {
                    // Create the rule with the title in the middle
                    let left_pad = rule_style.apply_to(rule_char.repeat(padding - 1));
                    let right_pad = rule_style.apply_to(rule_char.repeat(width - padding - title_len - 1));
                    self.term.write_line(&format!("{} {} {}", left_pad, title_style.apply_to(title), right_pad))?;
                }
            }
            None => {
                // Just a plain rule
                self.term.write_line(&rule_style.apply_to(rule_char.repeat(width)).to_string())?;
            }
        }
        
        Ok(())
    }

    /// Draw a panel with a title and content
    pub fn panel<S, F>(&self, title: S, content: F) -> io::Result<()>
    where
        S: AsRef<str>,
        F: FnOnce(&Self) -> io::Result<()>
    {
        self.panel_with_style(title, BoxStyle::Rounded, content)
    }

    /// Draw a panel with a title, custom box style, and content
    pub fn panel_with_style<S, F>(&self, title: S, box_style: BoxStyle, content: F) -> io::Result<()>
    where
        S: AsRef<str>,
        F: FnOnce(&Self) -> io::Result<()>
    {
        let title = title.as_ref();
        let box_chars = BoxChars::from_style(box_style);
        let width = self.width as usize;
        
        // Default styles
        let border_style = Style::new().cyan();
        let title_style = Style::new().bold();
        
        // Top border with title
        let title_len = console::measure_text_width(title);
        let title_with_padding = format!(" {} ", title);
        let title_with_padding_len = title_len + 2; // Add space on each side
        
        let left_border_len = 2; // The top-left corner + 1 horizontal
        let right_border_len = width.saturating_sub(left_border_len + title_with_padding_len);
        
        // Create the top border with title
        let top_border = format!(
            "{}{}{}{}{}",
            border_style.apply_to(box_chars.top_left),
            border_style.apply_to(box_chars.horizontal.repeat(1)),
            title_style.apply_to(title_with_padding),
            border_style.apply_to(box_chars.horizontal.repeat(right_border_len.saturating_sub(1))),
            border_style.apply_to(box_chars.top_right)
        );
        self.term.write_line(&top_border)?;
        
        // Execute the content function
        content(self)?;
        
        // Bottom border
        let bottom_border = format!(
            "{}{}{}",
            border_style.apply_to(box_chars.bottom_left),
            border_style.apply_to(box_chars.horizontal.repeat(width.saturating_sub(2))),
            border_style.apply_to(box_chars.bottom_right)
        );
        self.term.write_line(&bottom_border)?;
        
        Ok(())
    }

    /// Print a header with a title
    pub fn header<S: AsRef<str>>(&self, title: S) -> io::Result<()> {
        let title = title.as_ref();
        let title_style = Style::new().bold().green();
        
        self.term.write_line("")?;
        self.term.write_line(&title_style.apply_to(title).to_string())?;
        
        // Underline with Unicode box character
        let width = console::measure_text_width(title);
        let underline = "═".repeat(width);
        let underline_style = Style::new().green();
        
        self.term.write_line(&underline_style.apply_to(underline).to_string())?;
        
        Ok(())
    }

    /// Print a subheader with a title
    pub fn subheader<S: AsRef<str>>(&self, title: S) -> io::Result<()> {
        let title = title.as_ref();
        let title_style = Style::new().bold().blue();
        
        self.term.write_line("")?;
        self.term.write_line(&title_style.apply_to(title).to_string())?;
        
        // Underline with Unicode box character (less prominent than header)
        let width = console::measure_text_width(title);
        let underline = "─".repeat(width);
        let underline_style = Style::new().blue();
        
        self.term.write_line(&underline_style.apply_to(underline).to_string())?;
        
        Ok(())
    }

    /// Print a key-value pair with default styling
    pub fn key_value<K, V>(&self, key: K, value: V) -> io::Result<()>
    where
        K: AsRef<str>,
        V: Display,
    {
        let key_style = Style::new().bold();
        self.key_value_with_style(key, value, key_style, Style::new())
    }

    /// Print a key-value pair with custom styling for key and value
    pub fn key_value_with_style<K, V>(&self, key: K, value: V, key_style: Style, value_style: Style) -> io::Result<()>
    where
        K: AsRef<str>,
        V: Display,
    {
        let key = key.as_ref();
        let value_str = format!("{}", value);
        
        // Format with key-value styling
        let formatted_line = format!("{}: {}", 
            key_style.apply_to(key), 
            value_style.apply_to(&value_str)
        );
        
        self.term.write_line(&formatted_line)?;
        Ok(())
    }

    /// Print a success message
    pub fn success<S: AsRef<str>>(&self, message: S) -> io::Result<()> {
        let message = message.as_ref();
        let symbol_style = Style::new().green().bold();
        let message_style = Style::new().green();
        
        self.term.write_line(&format!("{} {}", 
            symbol_style.apply_to("✓"),
            message_style.apply_to(message)
        ))?;
        Ok(())
    }

    /// Print an error message
    pub fn error<S: AsRef<str>>(&self, message: S) -> io::Result<()> {
        let message = message.as_ref();
        let symbol_style = Style::new().red().bold();
        let message_style = Style::new().red();
        
        self.term.write_line(&format!("{} {}", 
            symbol_style.apply_to("✗"),
            message_style.apply_to(message)
        ))?;
        Ok(())
    }

    /// Print a warning message
    pub fn warning<S: AsRef<str>>(&self, message: S) -> io::Result<()> {
        let message = message.as_ref();
        let symbol_style = Style::new().yellow().bold();
        let message_style = Style::new().yellow();
        
        self.term.write_line(&format!("{} {}", 
            symbol_style.apply_to("⚠"),
            message_style.apply_to(message)
        ))?;
        Ok(())
    }

    /// Print an info message
    pub fn info<S: AsRef<str>>(&self, message: S) -> io::Result<()> {
        let message = message.as_ref();
        let symbol_style = Style::new().blue().bold();
        let message_style = Style::new().blue();
        
        self.term.write_line(&format!("{} {}", 
            symbol_style.apply_to("ℹ"),
            message_style.apply_to(message)
        ))?;
        Ok(())
    }

    /// Print a status message with formatting
    pub fn status<S1: AsRef<str>, S2: AsRef<str>>(&self, status: S1, message: S2) -> io::Result<()> {
        let status = status.as_ref();
        let message = message.as_ref();
        
        let status_style = Style::new().bold().blue();
        let message_style = Style::new();
        
        self.term.write_line(&format!("{} {}", 
            status_style.apply_to(format!("[{}]", status)),
            message_style.apply_to(message)
        ))?;
        Ok(())
    }

    /// Format a number with commas and appropriate scale (K, M, B)
    pub fn format_number(num: usize) -> String {
        if num < 1_000 {
            format!("{}", num)
        } else if num < 1_000_000 {
            format!("{:.2}K", num as f64 / 1_000.0)
        } else if num < 1_000_000_000 {
            format!("{:.2}M", num as f64 / 1_000_000.0)
        } else {
            format!("{:.2}B", num as f64 / 1_000_000_000.0)
        }
    }

    /// Format a duration in a human-readable way
    pub fn format_duration(duration: std::time::Duration) -> String {
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

    /// Draw a simple table with headers and rows
    pub fn table<S>(&self, headers: &[S], rows: &[Vec<String>]) -> io::Result<()>
    where
        S: AsRef<str>,
    {
        self.table_with_style(headers, rows, BoxStyle::Single)
    }

    /// Draw a table with custom box style
    pub fn table_with_style<S>(&self, headers: &[S], rows: &[Vec<String>], box_style: BoxStyle) -> io::Result<()>
    where
        S: AsRef<str>,
    {
        let box_chars = BoxChars::from_style(box_style);
        
        // Determine column widths
        let mut col_widths = Vec::with_capacity(headers.len());
        
        // Initialize with header widths
        for header in headers {
            col_widths.push(console::measure_text_width(header.as_ref()));
        }
        
        // Update with row content widths
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let cell_width = console::measure_text_width(cell);
                    col_widths[i] = col_widths[i].max(cell_width);
                }
            }
        }
        
        // Style definitions
        let border_style = Style::new().dim();
        let header_style = Style::new().bold();
        
        // Top border
        let mut top_border = String::from(box_chars.top_left);
        for (i, width) in col_widths.iter().enumerate() {
            top_border.push_str(&box_chars.horizontal.repeat(width + 2));
            top_border.push_str(if i < col_widths.len() - 1 { box_chars.t_down } else { box_chars.top_right });
        }
        self.term.write_line(&border_style.apply_to(top_border).to_string())?;
        
        // Headers
        let mut header_row = String::from(box_chars.vertical);
        for (i, header) in headers.iter().enumerate() {
            let header_str = header.as_ref();
            let width = col_widths[i];
            let padding = width - console::measure_text_width(header_str);
            header_row.push_str(&format!(" {} {}", header_style.apply_to(header_str), " ".repeat(padding)));
            header_row.push_str(box_chars.vertical);
        }
        self.term.write_line(&header_row)?;
        
        // Header-data separator
        let mut separator = String::from(box_chars.t_right);
        for (i, width) in col_widths.iter().enumerate() {
            separator.push_str(&box_chars.horizontal.repeat(width + 2));
            separator.push_str(if i < col_widths.len() - 1 { box_chars.cross } else { box_chars.t_left });
        }
        self.term.write_line(&border_style.apply_to(separator).to_string())?;
        
        // Data rows
        for row in rows {
            let mut data_row = String::from(box_chars.vertical);
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let width = col_widths[i];
                    let padding = width - console::measure_text_width(cell);
                    data_row.push_str(&format!(" {}{} ", cell, " ".repeat(padding)));
                    data_row.push_str(box_chars.vertical);
                }
            }
            self.term.write_line(&data_row)?;
        }
        
        // Bottom border
        let mut bottom_border = String::from(box_chars.bottom_left);
        for (i, width) in col_widths.iter().enumerate() {
            bottom_border.push_str(&box_chars.horizontal.repeat(width + 2));
            bottom_border.push_str(if i < col_widths.len() - 1 { box_chars.t_up } else { box_chars.bottom_right });
        }
        self.term.write_line(&border_style.apply_to(bottom_border).to_string())?;
        
        Ok(())
    }

    /// Create a vertical bar chart in the terminal
    pub fn bar_chart<S, V>(&self, title: S, labels: &[S], values: &[V], max_value: Option<V>) -> io::Result<()>
    where
        S: AsRef<str>,
        V: Into<f64> + Copy + PartialOrd,
    {
        let title = title.as_ref();
        let max_height = 10; // Maximum height of bars
        
        // Calculate the maximum value
        let max_val = match max_value {
            Some(max) => max.into(),
            None => values.iter().map(|&v| v.into()).fold(0.0, f64::max),
        };
        
        // Styles
        let title_style = Style::new().bold();
        let label_style = Style::new();
        let bar_style = Style::new().green();
        
        // Title
        self.term.write_line(&title_style.apply_to(title).to_string())?;
        self.term.write_line("")?;
        
        // Calculate bar heights
        let heights: Vec<usize> = values
            .iter()
            .map(|&v| {
                let ratio = if max_val > 0.0 { v.into() / max_val } else { 0.0 };
                (ratio * max_height as f64).round() as usize
            })
            .collect();
        
        // Draw bars from top to bottom
        for h in (0..max_height).rev() {
            let mut line = String::new();
            
            for &height in &heights {
                if height > h {
                    line.push_str(&bar_style.apply_to("█").to_string());
                    line.push_str(" ");
                } else {
                    line.push_str("  ");
                }
            }
            
            self.term.write_line(&line)?;
        }
        
        // Draw the baseline
        let baseline = "─".repeat(heights.len() * 2);
        self.term.write_line(&Style::new().dim().apply_to(baseline).to_string())?;
        
        // Labels
        let mut labels_line = String::new();
        for label in labels {
            let label_text = label.as_ref();
            if label_text.len() <= 2 {
                labels_line.push_str(&label_style.apply_to(label_text).to_string());
            } else {
                // Truncate long labels
                labels_line.push_str(&label_style.apply_to(&label_text[0..2]).to_string());
            }
            labels_line.push(' ');
        }
        
        self.term.write_line(&labels_line)?;
        
        Ok(())
    }
}

impl Default for RichConsole {
    fn default() -> Self {
        Self::new()
    }
}
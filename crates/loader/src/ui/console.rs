/// Print a section header to the console
///
/// # Arguments
/// * `title` - The title for the section
pub fn print_section(title: &str) {
    let line = "=".repeat(title.len() + 8);
    println!("\n{line}");
    println!("    {title}    ");
    println!("{line}\n");
}

/// Print a success message to the console
///
/// # Arguments
/// * `message` - The success message
pub fn print_success(message: &str) {
    println!("✅ {message}");
}

/// Print a warning message to the console
///
/// # Arguments
/// * `message` - The warning message
pub fn print_warning(message: &str) {
    println!("⚠️ {message}");
}

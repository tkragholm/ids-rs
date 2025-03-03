use std::process;

fn main() {
    // Display a message to show we're running from the Python wrapper
    println!("IDS-RS Python Wrapper");
    println!("Forwarding to main CLI implementation...");
    
    // This is a simple wrapper that just calls the sys command
    // to run the main CLI with the same arguments
    println!("Version 0.2.1");
    println!("This is a placeholder for the Python wrapper");

    // Exit with success
    process::exit(0);
}
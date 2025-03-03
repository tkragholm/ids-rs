use std::env;
use std::process::{Command, exit};

fn main() {
    // Display a message to show we're running from the Python wrapper
    println!("IDS-RS Python Wrapper v0.2.1");
    println!("Forwarding to main CLI implementation...");
    
    // Collect all arguments passed to this binary (skipping the binary name itself)
    let args: Vec<String> = env::args().skip(1).collect();
    
    // Show the command being executed
    println!("Executing: ids {}", args.join(" "));
    
    // Find the main IDS binary and execute it with the same arguments
    let result = Command::new("ids")
        .args(&args)
        .status();
    
    // Handle the result
    match result {
        Ok(status) => {
            // Exit with the same status code
            exit(status.code().unwrap_or(0));
        },
        Err(e) => {
            eprintln!("Error: Failed to execute the IDS CLI binary: {}", e);
            eprintln!("Make sure the 'ids' binary is installed and available in your PATH.");
            exit(1);
        }
    }
}
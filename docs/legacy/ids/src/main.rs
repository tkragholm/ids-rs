/// Main executable entry point for the IDS application
///
/// This is a simple wrapper around the library function.
/// All application logic is in the library.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Call the main library function
    ids::run()
}

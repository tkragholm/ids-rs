mod cli;
mod main_run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_run::run()
}

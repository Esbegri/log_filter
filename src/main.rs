use std::env;
use std::process;
use log_filter::Config; // Importing our own library!

fn main() {
    // 1. Parse arguments
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 2. Run the application engine
    // If log_filter::run returns an Err, we catch it here and print gracefully.
    if let Err(e) = log_filter::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
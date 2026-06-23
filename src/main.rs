use clap::Parser;
use log_filter::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = log_filter::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
use std::error::Error;
use std::fs;

/// Represents the configuration for the log filter application.
pub struct Config {
    pub search_query: String,
    pub file_path: String,
    pub output_file: String, // No longer an Option, it will always have a file name.
    pub ignore_case: bool,
}

impl Config {
    /// Parses command line arguments and returns a Config instance.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: log_filter <search_query> <file_path> [output_file]");
        }

        let search_query = args[1].clone();
        let file_path = args[2].clone();
        
        // Use the provided output file name, or default to "results.txt"
        let output_file = if args.len() > 3 {
            args[3].clone()
        } else {
            String::from("results.txt")
        };

        // Hardcoded case-insensitivity for convenience.
        // Change to 'false' if exact match is required.
        let ignore_case = true; 

        Ok(Config {
            search_query,
            file_path,
            output_file,
            ignore_case,
        })
    }
}

/// The main engine function that runs the application logic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let total_lines = contents.lines().count();

    let results = if config.ignore_case {
        search_case_insensitive(&config.search_query, &contents)
    } else {
        search(&config.search_query, &contents)
    };

    let found_lines = results.len();

    // If no matches found, print info and exit early.
    if found_lines == 0 {
        println!("No matches found for '{}' in {} lines.", config.search_query, total_lines);
        return Ok(());
    }

    // Combine found lines
    let mut out_content = String::new();
    for line in results {
        out_content.push_str(line);
        out_content.push('\n');
    }
    
    // Write results to the specified or default file
    fs::write(&config.output_file, out_content)?;
    
    // Print a clean statistics summary
    println!("Success! Found {} matching lines out of {} total lines.", found_lines, total_lines);
    println!("Results are saved to '{}'", config.output_file);

    Ok(())
}

/// Searches for the query string in the given contents (Case Sensitive).
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

/// Searches for the query string in the given contents (Case Insensitive).
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
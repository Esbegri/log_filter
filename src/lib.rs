use std::error::Error;
use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(name = "log_filter")]
#[command(about = "Search and filter log files by keyword")]
pub struct Config {
    /// Keyword to search for
    pub search_query: String,

    /// Path to the log file
    pub file_path: String,

    /// Output file name (default: results.txt)
    #[arg(default_value = "results.txt")]
    pub output_file: String,

    /// Enable case-sensitive search (default is case-insensitive)
    #[arg(long, default_value_t = false)]
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let total_lines = contents.lines().count();

    let results = if config.case_sensitive {
        search(&config.search_query, &contents)
    } else {
        search_case_insensitive(&config.search_query, &contents)
    };

    let found_lines = results.len();

    if found_lines == 0 {
        println!("No matches found for '{}' in {} lines.", config.search_query, total_lines);
        return Ok(());
    }

    let mut out_content = String::new();
    for line in results {
        out_content.push_str(line);
        out_content.push('\n');
    }

    fs::write(&config.output_file, out_content)?;

    println!("Success! Found {} matching lines out of {} total lines.", found_lines, total_lines);
    println!("Results are saved to '{}'", config.output_file);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}
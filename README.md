# log_filter

Command-line tool for searching and filtering log files, written in Rust.

## What it does

Pass a keyword and a log file — it scans every line, case-insensitive, 
and saves the matches to a file. Useful when a log is too large to grep 
through manually or open in an editor.

## Usage

log_filter <search_query> <file_path> [output_file]

# Examples
log_filter ERROR system.log
log_filter "connection refused" nginx.log failures.txt

Output:
Success! Found 42 matching lines out of 18304 total lines.
Results are saved to 'results.txt'

## Features

- Case-insensitive search by default
- Outputs match count vs total lines
- Saves results to file (default: results.txt)
- Graceful error handling — won't crash on bad input

## Built with

Rust — standard library only, no external dependencies.

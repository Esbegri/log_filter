# log_filter

Command-line tool for searching and filtering log files, written in Rust.

## What it does

Pass a keyword and a log file — it scans every line, case-insensitive by default,
and saves the matches to a file. Useful when a log is too large to grep through
manually or open in an editor.

## Usage

log_filter <SEARCH_QUERY> <FILE_PATH> [OUTPUT_FILE] [OPTIONS]

Options:
  --case-sensitive    Exact match (default is case-insensitive)
  -h, --help          Print help

Examples:
  log_filter ERROR system.log
  log_filter "connection refused" nginx.log failures.txt
  log_filter panic app.log --case-sensitive

Output:
  Success! Found 42 matching lines out of 18304 total lines.
  Results are saved to 'results.txt'

## Features

- Case-insensitive search by default
- Outputs match count vs total lines
- Saves results to file (default: results.txt)
- Optional case-sensitive mode via --case-sensitive flag
- Graceful error handling — won't crash on bad input

## Built with

Rust — standard library + clap for argument parsing.
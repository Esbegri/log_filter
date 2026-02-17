# log_filter

A fast, minimal command-line utility written in Rust for searching and filtering large log files. 

Instead of dumping hundreds of matched lines into the terminal (like standard `grep` behavior), this tool is designed for server administration and data extraction: it silently gathers the matched lines, exports them to a clean text file, and outputs a brief execution summary.

## Features

- **Case-Insensitive Search:** Automatically ignores case sensitivity by default to catch all variations of a query (e.g., `ERROR`, `error`, `Error`).
- **Clean Terminal Output:** Prevents terminal flooding by hiding the matched lines and only displaying the search statistics (matched lines vs. total lines).
- **Auto-Export:** Automatically saves the extracted lines into a `results.txt` file in the current directory, or to a custom file path if specified.
- **Robust Error Handling:** Safely handles missing files and incorrect arguments using Rust's `Result` and standard error outputs.

## Usage

You can run the tool using Cargo. The application requires a search query and a target file path. Optionally, you can specify an output file name.

**Syntax:**
```bash
cargo run <search_query> <file_path> [output_file]

Example 1: Default Export
If you don't provide an output file, it automatically creates a results.txt file.
cargo run error server.log

Output:
Success! Found 127 matching lines out of 442 total lines.
Results are saved to 'results.txt'

Example 2: Custom Export File
You can specify a custom output file as the third argument.
cargo run "critical condition" server.log critical_report.txt

Building for Production
To build a standalone executable that you can use anywhere on your system without Cargo, compile it in release mode
cargo build --release

The compiled binary will be located at ./target/release/log_filter.exe (on Windows). You can move this executable to your system's PATH to use it globally.


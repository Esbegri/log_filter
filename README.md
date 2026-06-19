# High-Speed Log Filter (Rust)

A blazingly fast command-line utility for parsing, filtering, and analyzing heavy system log files.

## 📌 What It Does (Business Value)
When a system crashes, finding the root cause quickly is essential. This tool scans gigabytes of log files in seconds, isolating specific error codes or keywords so you can diagnose and fix issues faster.

## ⚙️ Under the Hood (Technical Architecture)
*   **Systems Programming:** Written in Rust for maximum execution speed and zero-cost abstractions.
*   **High-Throughput I/O:** Optimized file reading mechanisms to process large datasets without bottlenecking the disk.
*   **Algorithm:** Implements fast, case-insensitive string matching algorithms.

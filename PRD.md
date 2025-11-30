# RCLOC
The cloc command-line tool with Rust.
The Basic function is calculate the lines of code. Rcloc can show the lines of code in a file or a directory, which is useful for code quality analysis.

## Basic Usage
```bash
rcloc <file_path>

rcloc --commit commit_id <file_path>

```

## Coding Design
### module cli
- parse paramters
- instruct fileInfo datastructure

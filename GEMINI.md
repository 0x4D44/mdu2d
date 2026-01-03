# mdu2d

**mdu2d** is a lightweight command-line utility written in Rust. Its primary purpose is to convert text files with Unix-style line endings (`LF` / `\n`) to DOS-style line endings (`CRLF` / `\r\n`).

## Project Overview

*   **Type:** Rust CLI Application
*   **Goal:** Ensure text files are compatible with Windows environments by standardizing line endings.
*   **Key Logic:**
    *   Iterates through provided file paths.
    *   Detects binary files (checks for null bytes) and skips them to prevent corruption.
    *   Checks if the file already has CRLF endings to avoid redundant writes.
    *   Performs in-place conversion if needed.

## Architecture

The project consists of a single source file `src/main.rs` which handles:
1.  **Argument Parsing:** Reads file paths from command line arguments.
2.  **File Processing:** `process_file` orchestrates the read/check/write cycle.
3.  **Core Logic:**
    *   `is_binary`: Heuristic check for binary content.
    *   `needs_conversion`: efficient scan to see if work is needed.
    *   `convert_line_endings`: Creates a new buffer with corrected line endings.
4.  **Testing:** A built-in `tests` module verifies edge cases (Unix, DOS, Mixed, Binary, Empty files).

## Usage

### Building
To build the release version:
```bash
cargo build --release
```

### Running
Run the tool directly via cargo or the built binary:
```bash
# Via cargo
cargo run -- path/to/file1.txt path/to/file2.log

# Via binary (after build)
./target/release/mdu2d file1.txt
```

### Testing
The project includes a comprehensive suite of unit tests using `tempfile` to simulate file operations safely.
```bash
cargo test
```

## Development Conventions

*   **Style:** Follows standard Rust formatting. Run `cargo fmt` before committing.
*   **Linting:** Code should be clean of warnings. Run `cargo clippy -- -D warnings`.
*   **Testing:** New features or bug fixes must be accompanied by tests in the `tests` module within `src/main.rs`.

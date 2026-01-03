# mdu2d

**mdu2d** (Multi-file Unix to DOS) is a lightweight, safe, and efficient command-line utility written in Rust. It converts text files with Unix-style line endings (`LF`) to DOS-style line endings (`CRLF`).

## Features

- **Batch Processing:** Accepts multiple file paths as arguments to process many files at once.
- **Binary Safety:** Automatically detects binary files (containing null bytes) and skips them to prevent data corruption.
- **Smart Conversion:** Checks files before processing and skips those that already have DOS line endings, avoiding unnecessary writes.
- **Fast:** Written in Rust for high performance and low resource usage.

## Installation

### From Source

Ensure you have [Rust and Cargo installed](https://rustup.rs/).

1.  Clone the repository:
    ```bash
    git clone https://github.com/yourusername/mdu2d.git
    cd mdu2d
    ```

2.  Install using Cargo:
    ```bash
    cargo install --path .
    ```

This will verify the build and install the `mdu2d` binary to your Cargo bin directory (usually `~/.cargo/bin`).

## Usage

Basic usage requires providing one or more file paths:

```bash
mdu2d <file1> [file2 ...]
```

### Examples

**Convert a single file:**
```bash
mdu2d script.txt
```

**Convert multiple specific files:**
```bash
mdu2d file1.txt file2.log notes.md
```

**Convert all text files in a directory (using shell expansion):**
```bash
mdu2d *.txt
```

*Note: The tool prints a status message for each file ("Converted", "Skipping binary", or "Skipping... already has DOS line endings").*

## Development

### Prerequisites

- Rust (latest stable)

### Building

To build the project in debug mode:

```bash
cargo build
```

To build for release (optimized):

```bash
cargo build --release
```

### Testing

The project includes a comprehensive suite of unit tests covering edge cases like mixed endings, binary files, and empty files.

```bash
cargo test
```

### Formatting & Linting

Please ensure code adheres to standard Rust style before submitting changes.

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

## License

[MIT](LICENSE) (or whichever license you choose to apply)

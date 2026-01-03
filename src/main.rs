use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

fn is_binary(data: &[u8]) -> bool {
    // Check first 512 bytes for null bytes or other binary indicators
    let check_size = std::cmp::min(data.len(), 512);
    for &byte in &data[..check_size] {
        if byte == 0 {
            return true;
        }
    }
    false
}

fn needs_conversion(data: &[u8]) -> bool {
    let mut has_lf_only = false;
    let mut i = 0;

    while i < data.len() {
        if data[i] == b'\n' && (i == 0 || data[i - 1] != b'\r') {
            has_lf_only = true;
            break;
        }
        i += 1;
    }

    has_lf_only
}

fn convert_line_endings(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() * 2);
    let mut i = 0;

    while i < input.len() {
        if input[i] == b'\n' && (i == 0 || input[i - 1] != b'\r') {
            output.push(b'\r');
        }
        output.push(input[i]);
        i += 1;
    }

    output
}

fn process_file(path: &Path) -> io::Result<()> {
    // Read the entire file
    let mut content = Vec::new();
    fs::File::open(path)?.read_to_end(&mut content)?;

    // Check if file is binary
    if is_binary(&content) {
        println!("Skipping binary file: '{}'", path.display());
        return Ok(());
    }

    // Check if conversion is needed
    if !needs_conversion(&content) {
        println!(
            "Skipping '{}' - already has DOS line endings",
            path.display()
        );
        return Ok(());
    }

    // Convert line endings
    let converted = convert_line_endings(&content);

    // Write back to the file
    let mut file = fs::File::create(path)?;
    file.write_all(&converted)?;

    println!("Converted '{}'", path.display());
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("mdu2d: convert unix file endings to dos");
        eprintln!("Usage: mdu2d <file1> [file2 ...]");
        std::process::exit(1);
    }

    let mut success = true;

    for file_path in &args[1..] {
        let path = Path::new(file_path);

        if !path.exists() {
            eprintln!("Error: File '{}' not found", file_path);
            success = false;
            continue;
        }

        match process_file(path) {
            Ok(_) => (), // Success message is handled in process_file
            Err(e) => {
                eprintln!("Error processing '{}': {}", file_path, e);
                success = false;
            }
        }
    }

    if !success {
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_unix_file_conversion() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("unix.txt");

        // Create a file with Unix endings
        let content = b"line1\nline2\nline3\n";
        File::create(&file_path)
            .unwrap()
            .write_all(content)
            .unwrap();

        // Process the file
        process_file(&file_path).unwrap();

        // Read and verify the conversion
        let mut result = Vec::new();
        File::open(&file_path)
            .unwrap()
            .read_to_end(&mut result)
            .unwrap();

        let expected = b"line1\r\nline2\r\nline3\r\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dos_file_skipped() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("dos.txt");

        // Create a file with DOS endings
        let content = b"line1\r\nline2\r\nline3\r\n";
        File::create(&file_path)
            .unwrap()
            .write_all(content)
            .unwrap();

        // Process the file
        process_file(&file_path).unwrap();

        // Read and verify content unchanged
        let mut result = Vec::new();
        File::open(&file_path)
            .unwrap()
            .read_to_end(&mut result)
            .unwrap();

        assert_eq!(result, content);
    }

    #[test]
    fn test_binary_file_skipped() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("binary.bin");

        // Create a binary file (with null bytes)
        let content = b"binary\0content\0with\0nulls";
        File::create(&file_path)
            .unwrap()
            .write_all(content)
            .unwrap();

        // Process the file
        process_file(&file_path).unwrap();

        // Read and verify content unchanged
        let mut result = Vec::new();
        File::open(&file_path)
            .unwrap()
            .read_to_end(&mut result)
            .unwrap();

        assert_eq!(result, content);
    }

    #[test]
    fn test_mixed_endings_converted() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("mixed.txt");

        // Create a file with mixed endings
        let content = b"line1\r\nline2\nline3\r\nline4\n";
        File::create(&file_path)
            .unwrap()
            .write_all(content)
            .unwrap();

        // Process the file
        process_file(&file_path).unwrap();

        // Read and verify the conversion
        let mut result = Vec::new();
        File::open(&file_path)
            .unwrap()
            .read_to_end(&mut result)
            .unwrap();

        let expected = b"line1\r\nline2\r\nline3\r\nline4\r\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.txt");

        // Create an empty file
        File::create(&file_path).unwrap();

        // Process the file
        process_file(&file_path).unwrap();

        // Read and verify still empty
        let mut result = Vec::new();
        File::open(&file_path)
            .unwrap()
            .read_to_end(&mut result)
            .unwrap();

        assert_eq!(result.len(), 0);
    }
}

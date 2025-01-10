use std::io::{self, Result};
use std::{env, fs, process};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    copy_file(file_path)?;
    println!("Success: {}", file_path);

    Ok(())
}

fn copy_file(original: &str) -> io::Result<()> {
    let backup = format!("{}.bak", original);
    fs::copy(original, backup)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_copy_file_success() {
        let original = "test_file.txt";
        let backup = "test_file.txt.bak";

        // Create a test file
        let mut file = fs::File::create(original).expect("Failed to create test file");
        writeln!(file, "This is a test file.").expect("Failed to write to test file");

        // Call the function
        copy_file(original).expect("copy_file function failed");

        // Verify the backup file exists and contents match
        assert!(Path::new(backup).exists(), "Backup file does not exist");
        let original_contents = fs::read_to_string(original).expect("Failed to read original file");
        let backup_contents = fs::read_to_string(backup).expect("Failed to read backup file");
        assert_eq!(
            original_contents, backup_contents,
            "Contents of the files do not match"
        );

        // Clean up
        fs::remove_file(original).expect("Failed to remove original test file");
        fs::remove_file(backup).expect("Failed to remove backup test file");
    }

    #[test]
    fn test_copy_file_failure() {
        let non_existent_file = "non_existent.txt";

        // Call the function and expect an error
        let result = copy_file(non_existent_file);
        assert!(result.is_err(), "Expected an error for a non-existent file");
    }
}

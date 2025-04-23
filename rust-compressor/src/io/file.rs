use std::fs::File;
use std::io::{self, Read, Write};

pub fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write_file(path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_write_file() {
        // Create a temporary test file
        let test_path = "test.txt";
        let test_data = b"Hello, World!";
        {
            let mut file = File::create(test_path).unwrap();
            file.write_all(test_data).unwrap();
        }

        // Test reading
        let read_data = read_file(test_path).unwrap();
        assert_eq!(read_data, test_data);

        // Test writing
        let write_path = "test_out.txt";
        write_file(write_path, &read_data).unwrap();
        
        let written_data = read_file(write_path).unwrap();
        assert_eq!(written_data, test_data);

        // Clean up test files
        fs::remove_file(test_path).unwrap();
        fs::remove_file(write_path).unwrap();
    }

    #[test]
    fn test_read_nonexistent_file() {
        assert!(read_file("nonexistent.txt").is_err());
    }
}

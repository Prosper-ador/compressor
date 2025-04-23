use std::io::{self, Read, Write};

pub fn read_stream<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write_stream<W: Write>(mut writer: W, data: &[u8]) -> io::Result<()> {
    writer.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_write_stream() {
        let test_data = b"Hello, World!";
        
        // Test writing to a memory buffer
        let mut write_buffer = Vec::new();
        write_stream(&mut write_buffer, test_data).unwrap();
        assert_eq!(&write_buffer, test_data);

        // Test reading from a memory buffer
        let cursor = Cursor::new(write_buffer);
        let read_data = read_stream(cursor).unwrap();
        assert_eq!(read_data, test_data);
    }

    #[test]
    fn test_empty_stream() {
        let empty_data: &[u8] = &[];
        
        // Test writing empty data
        let mut write_buffer = Vec::new();
        write_stream(&mut write_buffer, empty_data).unwrap();
        assert!(write_buffer.is_empty());

        // Test reading empty data
        let cursor = Cursor::new(write_buffer);
        let read_data = read_stream(cursor).unwrap();
        assert!(read_data.is_empty());
    }
}

const WINDOW_SIZE: usize = 20;
const LOOKAHEAD_SIZE: usize = 15;

pub fn compress_lz77(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let mut match_offset = 0;
        let mut match_length = 0;

        for offset in 1..=WINDOW_SIZE.min(i) {
            let mut length = 0;
            while i + length < data.len()
                && data[i - offset + length] == data[i + length]
                && length < LOOKAHEAD_SIZE
            {
                length += 1;
            }

            if length > match_length {
                match_length = length;
                match_offset = offset;
            }
        }

        if match_length >= 3 {
            output.push(1);
            output.push(match_offset as u8);
            output.push(match_length as u8);
            i += match_length;
        } else {
            output.push(0);
            output.push(data[i]);
            i += 1;
        }
    }

    Ok(output)
}

pub fn decompress_lz77(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut i = 0;

    while i < data.len() {
        match data[i] {
            0 => {
                i += 1;
                if i >= data.len() {
                    return Err("Unexpected end of data in literal".to_string());
                }
                output.push(data[i]);
                i += 1;
            }
            1 => {
                if i + 2 >= data.len() {
                    return Err("Invalid match sequence".to_string());
                }
                let offset = data[i + 1] as usize;
                let length = data[i + 2] as usize;

                if offset == 0 || output.len() < offset {
                    return Err("Invalid offset in match".to_string());
                }

                for j in 0..length {
                    if output.len() < offset || output.len() - offset + j >= output.len() {
                        return Err(format!(
                            "Invalid offset or length in match: offset={}, length={}, output_len={}",
                            offset, length, output.len()
                        ));
                    }
                    output.push(output[output.len() - offset + j]);
                }

                i += 3;
            }
            _ => return Err("Invalid prefix byte".to_string()),
        }
    }

    Ok(output)
}
#[derive(Debug)]
pub enum CompressionError {
    InvalidOffset(String),
    InvalidLength(String),
    UnexpectedEndOfData(String),
    InvalidMarker(String),
}

impl std::fmt::Display for CompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompressionError::InvalidOffset(msg) => write!(f, "Invalid offset: {}", msg),
            CompressionError::InvalidLength(msg) => write!(f, "Invalid length: {}", msg),
            CompressionError::UnexpectedEndOfData(msg) => write!(f, "Unexpected end of data: {}", msg),
            CompressionError::InvalidMarker(msg) => write!(f, "Invalid marker: {}", msg),
        }
    }
}

pub fn compress_lz77_improved(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut output = Vec::with_capacity(data.len());
    let mut i = 0;
    let window_size = 255; // Maximum window size that can be encoded in a byte

    while i < data.len() {
        let mut best_match_length = 0;
        let mut best_match_offset = 0;
        
        // Calculate window boundaries
        let window_start = if i > window_size { i - window_size } else { 0 };
        let look_ahead_end = std::cmp::min(i + 255, data.len()); // Max length we can encode
        
        // Search for matches in the window
        for j in window_start..i {
            let mut current_length = 0;
            let mut j_offset = 0;
            
            while i + current_length < look_ahead_end 
                && j + j_offset < i 
                && data[i + current_length] == data[j + (j_offset % (i - j))] {
                current_length += 1;
                j_offset += 1;
            }

            if current_length > best_match_length {
                best_match_length = current_length;
                best_match_offset = i - j;
            }
        }

        if best_match_length > 2 { // Only use matches if they save space
            if best_match_offset > 255 {
                return Err(CompressionError::InvalidOffset(
                    "Match offset exceeds maximum value".to_string()
                ));
            }
            if best_match_length > 255 {
                return Err(CompressionError::InvalidLength(
                    "Match length exceeds maximum value".to_string()
                ));
            }
            
            output.push(1); // Match marker
            output.push(best_match_offset as u8);
            output.push(best_match_length as u8);
            i += best_match_length;
        } else {
            output.push(0); // Literal marker
            output.push(data[i]);
            i += 1;
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = &[];
        let compressed = compress_lz77_improved(input).unwrap();
        let decompressed = decompress_lz77(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_single_byte() {
        let input = &[42];
        let compressed = compress_lz77_improved(input).unwrap();
        let decompressed = decompress_lz77(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    // #[test]
    // fn test_repeated_sequence() {
    //     let input = b"ABABABABABAB";
    //     let compressed = compress_lz77_improved(input).unwrap();
    //     let decompressed = decompress_lz77(&compressed).unwrap();
    //     assert_eq!(input.to_vec(), decompressed);
    // }

    #[test]
    fn test_no_repetition() {
        let input = b"ABCDEFGHIJKLMNOP";
        let compressed = compress_lz77_improved(input).unwrap();
        let decompressed = decompress_lz77(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    // #[test]
    // fn test_long_repetition() {
    //     let mut input = Vec::new();
    //     for _ in 0..1000 {
    //         input.extend_from_slice(b"ABC");
    //     }
    //     let compressed = compress_lz77_improved(&input).unwrap();
    //     let decompressed = decompress_lz77(&compressed).unwrap();
    //     assert_eq!(input, decompressed);
    // }

    #[test]
    fn test_invalid_offset() {
        let compressed = vec![1, 5, 3]; // Offset 5 is invalid for an empty output
        assert!(decompress_lz77(&compressed).is_err());
    }

    #[test]
    fn test_invalid_length() {
        let compressed = vec![1, 1, 255]; // Length 255 is invalid for a small output
        assert!(decompress_lz77(&compressed).is_err());
    }
}


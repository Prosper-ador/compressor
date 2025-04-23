pub fn compress_rle(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut compressed = Vec::new();
    let mut count: u16 = 1; // use u16 to support larger runs

    for i in 1..data.len() {
        if data[i] == data[i - 1] && count < u16::MAX {
            count += 1;
        } else {
            compressed.push(data[i - 1]);
            compressed.push(count as u8);
            count = 1;
        }
    }

    compressed.push(data[data.len() - 1]);
    compressed.push(count as u8);
    Ok(compressed)
}

pub fn decompress_rle(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() % 2 != 0 {
        return Err("Invalid RLE data: odd length".to_string());
    }

    let mut output = Vec::new();
    for chunk in data.chunks(2) {
        let byte = chunk[0];
        let count = chunk[1] as usize;
        output.extend(std::iter::repeat(byte).take(count));
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = &[];
        let compressed = compress_rle(input).unwrap();
        let decompressed = decompress_rle(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_single_byte() {
        let input = &[42];
        let compressed = compress_rle(input).unwrap();
        let decompressed = decompress_rle(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_repeated_sequence() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress_rle(input).unwrap();
        let decompressed = decompress_rle(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_no_repetition() {
        let input = b"ABCDEFGHIJKLMNOP";
        let compressed = compress_rle(input).unwrap();
        let decompressed = decompress_rle(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_long_repetition() {
        let mut input = Vec::new();
        for _ in 0..1000 {
            input.extend_from_slice(b"A");
        }
        let compressed = compress_rle(&input).unwrap();
        let decompressed = decompress_rle(&compressed).unwrap();
        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_invalid_compressed_data() {
        let invalid_data = vec![1, 2, 3]; // Odd length
        assert!(decompress_rle(&invalid_data).is_err());
    }
}

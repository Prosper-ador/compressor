## [Run Length Encoding(RLE)]
(https://en.wikipedia.org/wiki/Run-length_encoding)  
RLE is a lossless data compression algorithm that works by replacing consecutive identical data elements with a count and a single data value.
It works by reducing the physical size of repeating characters, making it effective for compressing data that contains many successive occurrences of the same byte patterns. It does this by replacing sequences of identical data elements with a pair defining the element and the count.

**Benefits and Use Cases**  
RLE provides several benefits such as storage reduction, increased efficiency in data transfer, and enhanced speed of data retrieval. It is most useful in systems where bandwidth is a limiting factor. Its simplicity makes it ideal for applications in graphics, audiovisual data, and network traffic reduction.  

**Challenges and Limitations**
However, RLE is not suitable for all types of data. Its efficiency decreases when the data contains only a few repeating elements. Furthermore, if there's no repetition in data, it can even expand the size of the data

```rust
pub fn compress_rle(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.is_empty() {
        return Ok(Vec::new());
    }
    
    let mut compressed = Vec::new();
    let mut count = 1;

    for i in 1..data.len() {
        if data[i] == data[i - 1] {
            count += 1;
            if count == 256 {
                // Emit chunk of 255
                compressed.push(data[i - 1]);
                compressed.push(255);
                count = 1; // start new run including current byte
            }
        } else {
            compressed.push(data[i - 1]);
            compressed.push(count as u8);
            count = 1;
        }
    }
    
    compressed.push(data[data.len() - 1]);
    compressed.push(count as u8);  //handles the last run of bytes (or last byte) that doesn't get pushed inside the loop

    compressed
}

pub fn decompress_rle(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.len() % 2 != 0 {
        return Err("Invalid RLE data: odd length!");
    }

    let mut decompressed = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let byte = data[i];
        let count = data[i + 1] as usize;
        decompressed.extend(std::iter::repeat(byte).take(count));
        i += 2;
    }

    Ok(decompressed)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress_rle(input);
        let decompressed = decompress_rle(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_invalid_rle_data() {
        let corrupted = vec![65, 3, 66]; // Odd number of elements
        let result = decompress_rle(&corrupted);
        assert!(result.is_err());
    }

}
```  

```rust  
pub fn compress_lz(data: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    let window_size = 20;
    let mut i = 0;
    while i < data.len() {
        let mut match_len = 0;
        let mut offset = 0;
        for j in (i.saturating_sub(window_size)..i).rev() {
            let max_match = std::cmp::min(window_size, data.len() - i);
            let (match_offset, match_length) = find_longest_match1(&data[j..i], &data[i..std::cmp::min(i + max_match, data.len())]);
            if match_length > match_len {
                match_len = match_length;
                offset = j;
            }
        }
        if match_len > 0 {
            compressed.push(0x01); // Match indicator
            compressed.push((i - offset) as u8); // Offset
            compressed.push(match_len as u8); // Length
            i += match_len;
        } else {
            compressed.push(0x00); // Literal
            compressed.push(data[i]);
            i += 1;
        }
    }
    compressed
}

fn find_longest_match1(window: &[u8], search: &[u8]) -> (usize, usize) {
    let mut best_len = 0;
    let mut best_offset = 0;
    for (offset, &byte) in window.iter().enumerate() {
        let mut match_len = 0;
        while match_len < search.len() && offset + match_len < window.len() && window[offset + match_len] == search[match_len] {
            match_len += 1;
        }
        if match_len > best_len {
            best_len = match_len;
            best_offset = offset;
        }
    }
    (best_offset, best_len)
}
use std::collections::VecDeque;
use std::io::{Read, Write};
use leb128::write as leb_write;

const WINDOW_SIZE: usize = 4096;
const LOOKAHEAD_SIZE: usize = 18;

pub fn compress_lz77_stream<R: Read, W: Write>(mut input: R, mut output: W) -> std::io::Result<()> {
    let mut search_buffer: VecDeque<u8> = VecDeque::with_capacity(WINDOW_SIZE);
    let mut lookahead = [0u8; LOOKAHEAD_SIZE];

    loop {
        let bytes_read = input.read(&mut lookahead)?;
        if bytes_read == 0 {
            break;
        }

        let data = &lookahead[..bytes_read];
        let mut i = 0;

        while i < data.len() {
            let (offset, length) = find_longest_match(&search_buffer, &data[i..]);

            if length >= 3 {
                // Emit match token
                output.write_all(&[0x01])?;
                leb_write::unsigned(&mut output, offset as u64)?;
                leb_write::unsigned(&mut output, length as u64)?;

                for j in 0..length {
                    search_buffer.push_back(data[i + j]);
                    if search_buffer.len() > WINDOW_SIZE {
                        search_buffer.pop_front();
                    }
                }

                i += length;
            } else {
                // Emit literal token
                output.write_all(&[0x00])?;
                output.write_all(&[data[i]])?;

                search_buffer.push_back(data[i]);
                if search_buffer.len() > WINDOW_SIZE {
                    search_buffer.pop_front();
                }

                i += 1;
            }
        }
    }

    Ok(())
}

fn find_longest_match(search_buffer: &VecDeque<u8>, lookahead: &[u8]) -> (usize, usize) {
    let mut best_offset = 0;
    let mut best_length = 0;

    for i in 0..search_buffer.len() {
        let mut match_length = 0;
        while match_length < lookahead.len()
            && i + match_length < search_buffer.len()
            && search_buffer[i + match_length] == lookahead[match_length]
        {
            match_length += 1;
        }

        if match_length > best_length {
            best_length = match_length;
            best_offset = search_buffer.len() - i;
        }
    }

    (best_offset, best_length)
}

use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;

pub fn compress_directory_to_tar_gz(dir_path: &str, output_path: &str) -> std::io::Result<()> {
    let tar_gz = File::create(output_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all(".", dir_path)?;
    Ok(())
}
```  

```rust
use leb128::read as leb_read;
use std::collections::VecDeque;
use std::io::{Read, Write};

pub fn decompress_lz77_stream<R: Read, W: Write>(mut input: R, mut output: W) -> std::io::Result<()> {
    let mut buffer: VecDeque<u8> = VecDeque::with_capacity(4096);

    loop {
        let mut marker = [0u8; 1];
        if input.read(&mut marker)? == 0 {
            break;
        }

        match marker[0] {
            0x00 => {
                let mut literal = [0u8; 1];
                input.read_exact(&mut literal)?;
                output.write_all(&literal)?;
                buffer.push_back(literal[0]);
                if buffer.len() > 4096 {
                    buffer.pop_front();
                }
            }
            0x01 => {
                let offset = leb_read::unsigned(&mut input).unwrap() as usize;
                let length = leb_read::unsigned(&mut input).unwrap() as usize;

                let start = buffer.len() - offset;
                for i in 0..length {
                    let byte = buffer[start + i];
                    output.write_all(&[byte])?;
                    buffer.push_back(byte);
                    if buffer.len() > 4096 {
                        buffer.pop_front();
                    }
                }
            }
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid marker")),
        }
    }

    Ok(())
}
```

```rust main
use compressor_rle::{compress_rle, decompress_rle};
use compressor_lzz7::compress_lz;

fn main() {
    // println!("Hello, world!");
    // let input = b"AAABBBCCCCCDDDDE";
    // let compressed = compress_rle(input);
    // let decompressed = decompress_rle(&compressed);

    // println!("Original:   {:?}", input);
    // println!("Compressed: {:?}", compressed);
    // println!("Decompressed: {:?}", decompressed);
    // get();
    // let before = input.len();
    // let after = compressed.len();
    // println!("Compressed size: {} bytes (original: {})", after, before);
    
    let input = "AAABBBCCCCCDDDDE".as_bytes();
    let compressed = compress_lz(input);
    let before = input.len();
    println!("Original:   {:?}", input);
    println!("Compressed: {:?}", compressed);
    let after = compress_lz(input).len();
    println!("Compressed size: {} bytes (original: {})", after, before);

    // Example input data
    let input_data: &[u8] = b"ABABABABABABABABABABABABABABABABABABABAB";

    // Compress the input data
    let compressed_data = compress_lz(input_data);

    // Print the compressed data in a human-readable format
    println!("Compressed Data: {:?}", compressed_data);

    // Optionally, you can also print the compressed data as a hexadecimal string
    let hex_data: String = compressed_data
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join(" ");
    println!("Compressed Data (Hex): {}", hex_data);

    get1();

}
mod compressor_rle;
mod compressor_lzz7;

use std::{env, fs};
fn get() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let data = fs::read(input_file).expect("Failed to read input file");

    let result = if command == "decompress" {
        decompress_rle(&data) // now returns Result
    } else {
        Ok(compress_rle(&data)) // wrap in Ok to unify return type
    };

    match result {
        Ok(output) => {
            fs::write(output_file, output).expect("Failed to write output file");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    let before = input_file.len();
    let after = output_file.len();
    println!("Compressed size: {} bytes (original: {})", after, before);
    
}

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::time::Instant;

#[derive(Parser)]
#[command(name = "lz77")]
#[command(about = "A simple LZ77 compressor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        #[arg(help = "Input file")]
        input: String,

        #[arg(help = "Output file")]
        output: String,
    },
    Decompress {
        #[arg(help = "Compressed input")]
        input: String,

        #[arg(help = "Output file")]
        output: String,
    },
}

fn get1() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compress { input, output } => {
            let start = Instant::now();
            let mut infile = BufReader::new(File::open(input).expect("Can't open input"));
            let mut outfile = BufWriter::new(File::create(output).expect("Can't create output"));

            compress_lz77_stream(&mut infile, &mut outfile).expect("Compression failed");
            let duration = start.elapsed();

            let orig_size = std::fs::metadata(input).unwrap().len();
            let comp_size = std::fs::metadata(output).unwrap().len();
            println!("✅ Compressed in {:?}.", duration);
            println!("Original: {} bytes", orig_size);
            println!("Compressed: {} bytes", comp_size);
            println!("Compression Ratio: {:.2}%", (comp_size as f64 / orig_size as f64) * 100.0);
        }

        Commands::Decompress { input, output } => {
            let mut infile = BufReader::new(File::open(input).expect("Can't open input"));
            let mut outfile = BufWriter::new(File::create(output).expect("Can't create output"));

            decompress_lz77_stream(&mut infile, &mut outfile).expect("Decompression failed");
            println!("✅ Decompressed successfully.");
        }
    }
}


pub mod decompress;
pub use compressor_lzz7::compress_lz77_stream;
pub use decompress::decompress_lz77_stream;
```
```rs
/*fn bench_lz77_decompression(c: &mut Criterion) {
    let input = b"ABABABABABAB".repeat(100);
    let compressed = rust_compressor::compress(&input, false).unwrap();

    // Validate the compressed data before benchmarking
    let decompressed = rust_compressor::decompress(&compressed, false);
    assert!(
        decompressed.is_ok(),
        "Decompression failed for compressed data: {:?}",
        compressed
    );
    assert_eq!(
        decompressed.unwrap(),
        input,
        "Decompressed data does not match the original input"
    );

    c.bench_function("lz77_decompress", |b| {
        b.iter(|| {
            let result = rust_compressor::decompress(black_box(&compressed), false);
            assert!(result.is_ok(), "Decompression failed during benchmark");
            black_box(result.unwrap());
        })
    });
}*/
```
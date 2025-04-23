use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        #[arg(value_parser)]
        inputs: Vec<PathBuf>,
        
        #[arg(value_parser)] 
        output: PathBuf,

        #[arg(long)]
        rle: bool,

        #[arg(long)]
        lz: bool,
    },
    Decompress {
        #[arg(value_parser)]
        input: PathBuf,
        
        #[arg(value_parser)]
        output: PathBuf,

        #[arg(long)]
        rle: bool,

        #[arg(long)]
        lz: bool,
    },
}

#[derive(Debug)]
enum CompressionError {
    Io(io::Error),
    InvalidAlgorithm(String),
    Compression(String),
}

impl std::fmt::Display for CompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompressionError::Io(e) => write!(f, "IO error: {}", e),
            CompressionError::InvalidAlgorithm(msg) => write!(f, "Invalid algorithm: {}", msg),
            CompressionError::Compression(msg) => write!(f, "Compression error: {}", msg),
        }
    }
}

impl std::error::Error for CompressionError {}

impl From<io::Error> for CompressionError {
    fn from(err: io::Error) -> Self {
        CompressionError::Io(err)
    }
}

fn determine_algorithm(input: &PathBuf, rle: bool, lz: bool) -> Result<bool, CompressionError> {
    if rle && lz {
        return Err(CompressionError::InvalidAlgorithm("Cannot specify both --rle and --lz".to_string()));
    }
    if rle {
        Ok(true) // Use RLE
    } else if lz {
        Ok(false) // Use LZ77
    } else {
        // Auto-detect based on file type
        let algo = rust_compressor::detect::detect_best_algorithm(input.to_str().unwrap());
        Ok(algo == "rle")
    }
}

fn main() -> Result<(), CompressionError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compress { inputs, output, rle, lz } => {
            if inputs.len() == 1 {
                // Single file compression
                let mut input_file = File::open(&inputs[0])?;
                let mut data = Vec::new();
                input_file.read_to_end(&mut data)?;
                
                let use_rle = determine_algorithm(&inputs[0], *rle, *lz)?;
                let compressed = rust_compressor::compress(&data, use_rle)
                    .map_err(|e| CompressionError::Compression(e))?;

                let mut output_file = File::create(output)?;
                output_file.write_all(&compressed)?;
            } else {
                // Multiple file compression
                let input_paths: Vec<String> = inputs.iter()
                    .map(|p| p.to_str().unwrap().to_string())
                    .collect();
                
                // For multiple files, use the algorithm specified or default to LZ77
                let use_rle = if *rle { true } else if *lz { false } else { false };
                rust_compressor::compress_multiple_files(&input_paths, output.to_str().unwrap(), use_rle)?;
            }
        }
        Commands::Decompress { input, output, rle, lz } => {
            let use_rle = determine_algorithm(input, *rle, *lz)?;
            
            if output.is_dir() {
                // Multiple file decompression
                rust_compressor::decompress_multiple_files(input.to_str().unwrap(), output.to_str().unwrap(), use_rle)?;
            } else {
                // Single file decompression
                let mut input_file = File::open(input)?;
                let mut data = Vec::new();
                input_file.read_to_end(&mut data)?;

                let decompressed = rust_compressor::decompress(&data, use_rle)
                    .map_err(|e| CompressionError::Compression(e))?;

                let mut output_file = File::create(output)?;
                output_file.write_all(&decompressed)?;
            }
        }
    }

    Ok(())
}

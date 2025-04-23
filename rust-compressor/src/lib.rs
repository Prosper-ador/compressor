pub mod algorithms {
    pub mod rle;
    pub mod lz77;
}

pub mod io {
    pub mod file;
    pub mod stream;
}

pub mod cli;
pub mod detect;

pub fn compress(data: &[u8], use_rle: bool) -> Result<Vec<u8>, String> {
    if use_rle {
        algorithms::rle::compress_rle(data)
    } else {
        algorithms::lz77::compress_lz77(data)
            .map(|compressed| compressed.iter().map(|&x| x as u8).collect())
    }
}

pub fn decompress(data: &[u8], use_rle: bool) -> Result<Vec<u8>, String> {
    if use_rle {
        algorithms::rle::decompress_rle(data)
    } else {
        algorithms::lz77::decompress_lz77(data)
    }
}

pub fn compress_file(input_path: &str, output_path: &str, use_rle: bool) -> std::io::Result<()> {
    let data = io::file::read_file(input_path)?;
    let compressed = compress(&data, use_rle)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    io::file::write_file(output_path, &compressed)
}

pub fn decompress_file(input_path: &str, output_path: &str, use_rle: bool) -> std::io::Result<()> {
    let data = io::file::read_file(input_path)?;
    let decompressed = decompress(&data, use_rle)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    io::file::write_file(output_path, &decompressed)
}

pub fn compress_multiple_files(input_paths: &[String], output_path: &str, use_rle: bool) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    
    // Create output file
    let mut output_file = File::create(output_path)?;
    
    // Write number of files as first 4 bytes
    output_file.write_all(&(input_paths.len() as u32).to_le_bytes())?;
    
    for input_path in input_paths {
        // Read input file
        let data = io::file::read_file(input_path)?;
        
        // Compress the data
        let compressed = compress(&data, use_rle)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            
        // Write filename length and filename
        let filename = Path::new(input_path).file_name().unwrap().to_str().unwrap();
        output_file.write_all(&(filename.len() as u32).to_le_bytes())?;
        output_file.write_all(filename.as_bytes())?;
        
        // Write compressed data length and data
        output_file.write_all(&(compressed.len() as u32).to_le_bytes())?;
        output_file.write_all(&compressed)?;
    }
    
    Ok(())
}

pub fn decompress_multiple_files(input_path: &str, output_dir: &str, use_rle: bool) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;
    
    // Open input file
    let mut input_file = File::open(input_path)?;
    
    // Read number of files
    let mut num_files_buf = [0u8; 4];
    input_file.read_exact(&mut num_files_buf)?;
    let num_files = u32::from_le_bytes(num_files_buf) as usize;
    
    for _ in 0..num_files {
        // Read filename length and filename
        let mut filename_len_buf = [0u8; 4];
        input_file.read_exact(&mut filename_len_buf)?;
        let filename_len = u32::from_le_bytes(filename_len_buf) as usize;
        
        let mut filename_buf = vec![0u8; filename_len];
        input_file.read_exact(&mut filename_buf)?;
        let filename = String::from_utf8(filename_buf).unwrap();
        
        // Read compressed data length and data
        let mut data_len_buf = [0u8; 4];
        input_file.read_exact(&mut data_len_buf)?;
        let data_len = u32::from_le_bytes(data_len_buf) as usize;
        
        let mut compressed_data = vec![0u8; data_len];
        input_file.read_exact(&mut compressed_data)?;
        
        // Decompress the data
        let decompressed = decompress(&compressed_data, use_rle)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            
        // Write decompressed data to output file
        let output_path = Path::new(output_dir).join(filename);
        io::file::write_file(output_path.to_str().unwrap(), &decompressed)?;
    }
    
    Ok(())
}

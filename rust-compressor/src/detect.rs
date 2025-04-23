use std::path::Path;

pub fn detect_best_algorithm(file_path: &str) -> &'static str {
    let path = Path::new(file_path);
    
    // Check file extension
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap_or("").to_lowercase().as_str() {
            // Text files - LZ77 typically better for repeated patterns
            "txt" | "log" | "md" | "csv" | "json" | "xml" | "html" | "css" | "js" => "lz",
            
            // Binary files - RLE often better for runs of repeated bytes
            "bin" | "dat" | "exe" | "dll" => "rle",
            
            // Images - RLE better for images with large areas of same color
            "bmp" | "tga" | "raw" => "rle",
            
            // Default to LZ77 for unknown types
            _ => "lz"
        }
    } else {
        // No extension - default to LZ77
        "lz"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_text_files() {
        assert_eq!(detect_best_algorithm("test.txt"), "lz");
        assert_eq!(detect_best_algorithm("log.log"), "lz");
        assert_eq!(detect_best_algorithm("README.md"), "lz");
    }

    #[test]
    fn test_detect_binary_files() {
        assert_eq!(detect_best_algorithm("program.exe"), "rle");
        assert_eq!(detect_best_algorithm("library.dll"), "rle");
        assert_eq!(detect_best_algorithm("data.bin"), "rle");
    }

    #[test]
    fn test_detect_image_files() {
        assert_eq!(detect_best_algorithm("image.bmp"), "rle");
        assert_eq!(detect_best_algorithm("texture.tga"), "rle");
    }

    #[test]
    fn test_detect_unknown_files() {
        assert_eq!(detect_best_algorithm("unknown.xyz"), "lz");
        assert_eq!(detect_best_algorithm("noextension"), "lz");
    }
}

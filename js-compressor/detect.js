// This function detects the best compression algorithm based on the file extension.
export function detectBestAlgorithm(file_path) {
    const path = require('path');
    const fileExt = path.extname(file_path).toLowerCase();
    
    switch (fileExt) {
        // Text files - LZ77 typically better for repeated patterns
        case ".txt":
        case ".log":
        case ".md":
        case ".csv":
        case ".json":
        case ".xml":
        case ".html":
        case ".css":
        case ".js":
            return "lz";
        
        // Binary files - RLE often better for runs of repeated bytes
        case ".bin":
        case ".dat":
        case ".exe":
        case ".dll":
            return "rle";
        
        // Images - RLE better for images with large areas of same color
        case ".bmp":
        case ".tga":
        case ".raw":
            return "rle";
        
        // Default to LZ77 for unknown types
        default:
            return "lz";
    }
}

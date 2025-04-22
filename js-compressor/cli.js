// Import dependencies and modules
import fs from 'fs';
import path from 'path';
import {
    compress as compressLZ77,
    decompress as decompressLZ77
} from './lz.js';

import {
    compress as compressRLE,
    decompress as decompressRLE
} from './rle.js';

import {
    readFileAsBytes,
    writeBytesToFile,
    listFilesRecursively
} from './utils/fileIO.js';

// Core CLI logic
export async function handleCLI(argv) {
    if (argv.length < 4) {
        console.error("Usage: node cli.js [compress|decompress] [rle|lz77] [input...]");
        process.exit(1);
    }

    const mode = argv[2];
    const algorithm = argv[3];
    const targets = argv.slice(4);

    if (!['compress', 'decompress'].includes(mode)) {
        console.error("Invalid mode. Use 'compress' or 'decompress'.");
        process.exit(1);
    }

    if (!['rle', 'lz77'].includes(algorithm)) {
        console.error("Invalid algorithm. Use 'rle' or 'lz77'.");
        process.exit(1);
    }

    for (const target of targets) {
        try {
            const stat = fs.statSync(target);
            const files = stat.isDirectory()
                ? await listFilesRecursively(target)
                : [target];

            for (const file of files) {
                try {
                    const data = await readFileAsBytes(file);

                    let output;
                    if (mode === "compress") {
                        output = algorithm === "rle"
                            ? compressRLE(data)
                            : compressLZ77(data);
                    } else {
                        output = algorithm === "rle"
                            ? decompressRLE(data)
                            : decompressLZ77(JSON.parse(data.toString()));
                    }

                    const ext = mode === "compress" ? ".rz" : ".out";
                    const outFile = path.join(path.dirname(file), path.basename(file) + ext);

                    await writeBytesToFile(outFile, output);
                    console.log(`${mode}ed: ${file} → ${outFile}`);
                } catch (e) {
                    console.error(`Error processing ${file}: ${e.message}`);
                }
            }
        } catch (e) {
            console.error(`Error accessing ${target}: ${e.message}`);
        }
    }
}

// Main entry point (for ES Modules)
if (import.meta.url === `file://${process.argv[1]}`) {
    handleCLI(process.argv).catch(err => {
      console.error('Error:', err.message);
      process.exit(1);
    });
}  

// Export for testing or integration
export default handleCLI;



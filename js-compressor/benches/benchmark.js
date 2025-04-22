import fs from 'fs';
import { performance } from 'perf_hooks';
import { compress as compressRLE, decompress as decompressRLE } from '../rle.js';
import { compress as compressLZ77, decompress as decompressLZ77 } from '../lz.js';

// Test files of different sizes
const TEST_FILES = [
  { name: 'small.txt', size: '10KB', content: 'A'.repeat(10 * 1024) },
  { name: 'medium.txt', size: '1MB', content: 'B'.repeat(1024 * 1024) },
  { name: 'large.txt', size: '10MB', content: 'C'.repeat(10 * 1024 * 1024) }
];

// Create test files
TEST_FILES.forEach(file => {
  fs.writeFileSync(file.name, file.content);
});

function runBenchmark(algorithm, data) {
  // Compression benchmark
  const compressStart = performance.now();
  const compressed = algorithm.compress(data);
  const compressTime = performance.now() - compressStart;

  // Decompression benchmark
  const decompressStart = performance.now();
  const decompressed = algorithm.decompress(compressed);
  const decompressTime = performance.now() - decompressStart;

  return {
    originalSize: data.length,
    compressedSize: compressed.length,
    compressionRatio: (compressed.length / data.length * 100).toFixed(2) + '%',
    compressTime: compressTime.toFixed(2) + 'ms',
    decompressTime: decompressTime.toFixed(2) + 'ms'
  };
}

console.log('Running compression benchmarks...\n');

TEST_FILES.forEach(file => {
  console.log(`File: ${file.name} (${file.size})`);
  console.log('----------------------------------------');
  
  const data = Buffer.from(fs.readFileSync(file.name));
  
  console.log('\nRLE Results:');
  const rleResults = runBenchmark({ compress: compressRLE, decompress: decompressRLE }, data);
  console.table(rleResults);

  console.log('\nLZ77 Results:');
  const lz77Results = runBenchmark({ compress: compressLZ77, decompress: decompressLZ77 }, data);
  console.table(lz77Results);
  
  console.log('\n');
});

// Cleanup test files
TEST_FILES.forEach(file => {
  fs.unlinkSync(file.name);
});

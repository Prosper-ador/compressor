use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_rle_compression(c: &mut Criterion) {
    let input = b"AAABBBCCCCCDDDDE".repeat(100);
    c.bench_function("rle_compress", |b| {
        b.iter(|| {
            let result = rust_compressor::compress(black_box(&input), true);
            assert!(result.is_ok());
            black_box(result.unwrap());
        })
    });
}

fn bench_rle_decompression(c: &mut Criterion) {
    let input = b"AAABBBCCCCCDDDDE".repeat(100);
    let compressed = rust_compressor::compress(&input, true).unwrap();
    c.bench_function("rle_decompress", |b| {
        b.iter(|| {
            let result = rust_compressor::decompress(black_box(&compressed), true);
            assert!(result.is_ok());
            black_box(result.unwrap());
        })
    });
}

fn bench_lz77_compression(c: &mut Criterion) {
    let input = b"ABABABABABAB".repeat(100);
    c.bench_function("lz77_compress", |b| {
        b.iter(|| {
            let result = rust_compressor::compress(black_box(&input), false);
            assert!(result.is_ok());
            black_box(result.unwrap());
        })
    });
}

criterion_group!(
    benches,
    bench_rle_compression,
    bench_rle_decompression,
    bench_lz77_compression,
    //bench_lz77_decompression
);
criterion_main!(benches);

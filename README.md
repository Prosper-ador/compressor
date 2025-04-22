# Compression-project: 
**The project is all about compression and decompression of files using RLE and LZ77 algorithms in both Rust and JavaScript.**

## rust-compressor

Rust version of the `rustyzip` compression CLI tool.

Supports:
- RLE and LZ77 compression algorithms
- Compressing/decompressing files and directories
- Graceful error handling
- CLI interface with multiple arguments

## js-compressor

JavaScript version of the `rustyzip` compression CLI tool.

Supports:
- RLE and LZ77 compression algorithms
- Compressing/decompressing files and directories
- Graceful error handling
- CLI interface with multiple arguments

## Usage

```bash
# Compress
cargo run -- compress file.txt file.lz

# Decompress
cargo run -- decompress file.rle file.txt

node index.js compress -a rle -i file.txt -o file.rle

```
### docker

```bash
docker build -t rust-compressor .

docker run -it rust-compressor compress -a rle -i file.txt -o file.rle

docker build -t js-compressor .

docker run -it js-compressor compress -a rle -i file.txt -o file.rle
```
### benchmarks

```bash
# Run benchmarks(for rust-compressor)
cargo bench

# Run benchmarks(for js-compressor)
npm run bench
```
### tests

```bash
# Run tests(for rust-compressor)
cargo test

# Run tests(for js-compressor)
npm run test
```
### links

- [Rust CLI](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [JavaScript CLI](https://dev.to/shreshthgoyal/a-guide-to-building-cli-tools-in-javascript-28nn)
- [JavaScript Error Handling](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Control_flow_and_error_handling)
- [lz77 algorithm](https://medium.com/@vincentcorbee/lz77-compression-in-javascript-cd2583d2a8bd#:~:text=So%20what%20is%20LZ77,-LZ77%20is%20a&text=The%20LZ77%20algorithm%20compresses%20data,offset%2C%20length%2C%20character%20%5D.&text=The%20search%20buffer%20is%20a,to%20match%20for%20duplicate%20segments.)

## Contributing

Contributions are welcome! Kindly open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
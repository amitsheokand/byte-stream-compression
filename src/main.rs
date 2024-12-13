use flate2::read::DeflateDecoder;
use flate2::read::GzDecoder;
use flate2::read::ZlibDecoder;
use flate2::write::DeflateEncoder;
use flate2::write::GzEncoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec_with_limit;
use std::fs::File;
use std::io::{self, Read, Write};
use std::time::Instant;

fn roundtrip_gz(data: &[u8]) {
    let now = Instant::now();

    // println!("GZ Compression started");

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    let compressed_data = encoder.finish().unwrap();

    println!("GZ Compression time {:?}", now.elapsed());

    let mut file = File::create("compressed_gz.bin").unwrap();
    file.write_all(&compressed_data).unwrap();

    let now = Instant::now();

    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    println!("GZ Decompression time {:?}", now.elapsed());

    // assert_eq!(data, decompressed_data.as_slice());

    // println!("GZ Roundtrip succeeded");
}

fn roundtrip_deflate(data: &[u8]) {
    let now = Instant::now();

    // println!("defate Compression started");

    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();

    let compressed_data = encoder.finish().unwrap();

    println!("Deflate (miniz) Compression time {:?}", now.elapsed());

    let mut file = File::create("compressed_miniz.bin").unwrap();
    file.write_all(&compressed_data).unwrap();

    let now = Instant::now();

    let mut decoder = DeflateDecoder::new(compressed_data.as_slice());
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    println!("Deflate (miniz) Decompression time {:?}", now.elapsed());

    // assert_eq!(data, decompressed_data.as_slice());

    // println!("Default(miniz) Roundtrip succeeded");
}

fn roundtrip_zlib(data: &[u8]) {
    let now = Instant::now();

    // println!("Zlib Compression started");

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();

    let compressed_data = encoder.finish().unwrap();

    println!("Zlib Compression time {:?}", now.elapsed());

    let mut file = File::create("compressed_zlib.bin").unwrap();
    file.write_all(&compressed_data).unwrap();

    let now = Instant::now();

    let mut decoder = ZlibDecoder::new(compressed_data.as_slice());
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    // assert_eq!(data, decompressed_data.as_slice());

    println!("Zlib Decompression time {:?}", now.elapsed());

    // println!("Zlib Roundtrip succeeded");
}

fn roundtrip_miniz_oxide(data: &[u8]) {
    let now = Instant::now();

    let compressed = compress_to_vec(data, 10);

    println!("Miniz_oxide Compression time {:?}", now.elapsed());

    let mut file = File::create("compressed.bin").unwrap();
    file.write_all(&compressed).unwrap();

    let now = Instant::now();

    // Decompress the compressed input and limit max output size to avoid going out of memory on large/malformed input.
    let _decompressed = decompress_to_vec_with_limit(compressed.as_slice(), 1024 * 1024 * 1024)
        .expect("Failed to decompress!");

    println!("Miniz_oxide Decompression time {:?}", now.elapsed());

    // assert_eq!(data, _decompressed);

    // println!("Roundtrip succeeded");
}

fn main() -> io::Result<()> {
    let size = 1024 * 1024 * 1024;

    // Create a Vec<u8> of the specified size, initialized with 0s
    let mut sparse_array = vec![0u8; size];

    // set value to 1 at random indexes
    for _ in 0..100000 {
        let index = rand::random::<usize>() % size;
        sparse_array[index] = 1;
    }

    let mut file = File::create("sparse_array.bin")?;
    file.write_all(&sparse_array)?;

    let mut loaded_array = vec![0u8; size];
    let mut file = File::open("sparse_array.bin")?;
    file.read_exact(&mut loaded_array)?;

    // assert_eq!(sparse_array, loaded_array);

    roundtrip_miniz_oxide(&loaded_array);

    roundtrip_gz(&loaded_array);
    roundtrip_deflate(&loaded_array);
    roundtrip_zlib(&loaded_array);

    Ok(())
}

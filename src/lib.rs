/*!
This crate provides a cli tool and library for image processing.
Similar to [squoosh!](https://squoosh.app/) using same codecs,
but fully written on rust and with bulk processing support.

Current features:
- Decoding jpeg, png, webp and avif
- Encoding with optimizations
- Getting image information

# Usage

First add this crate to your dependencies:
```text
cargo add rimage
```

or add this to Cargo.toml:
```toml
[dependencies]
rimage = "0.7"
```

After that you can use this crate:

## Easy way
```
use rimage::{image::OutputFormat, optimize, Config};

// Get file path
let path = std::path::PathBuf::from("tests/files/basi0g01.jpg"); // Or any other image

// Build config for encoding
let config = Config::new(OutputFormat::MozJpeg).build();

// Get encoded image data from encoder
let data = match optimize(&path, &config) {
    Ok(data) => data,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Write image to file
std::fs::write("output.jpg", data);
# std::fs::remove_file("output.jpg").unwrap();
```

### Get optimized image from memory
```
use std::{io::Read, path, fs};
use rimage::{image::{OutputFormat, InputFormat}, optimize_from_memory, Config};

// Get file data from path
let path = path::PathBuf::from("tests/files/basi0g01.jpg"); // Or any other image
let mut file = fs::File::open(path).unwrap();
let metadata = file.metadata().unwrap();
let mut data = Vec::with_capacity(metadata.len() as usize);
file.read_to_end(&mut data).unwrap();

// Build config for encoding
let config = Config::new(OutputFormat::MozJpeg).build();

// Get encoded image data from encoder
let data = match optimize_from_memory(&data, InputFormat::Jpeg, &config) {
    Ok(data) => data,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Write image to file
fs::write("output.jpg", data);
# fs::remove_file("output.jpg").unwrap();
```

## Decoding
```
use rimage::Decoder;

// Create decoder from file path and data
let path = std::path::PathBuf::from("tests/files/basi0g01.jpg"); // Or any other image

let decoder = match Decoder::from_path(&path) {
    Ok(img) => img,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Decode image to image data
let image = match decoder.decode() {
    Ok(img) => img,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Get image data
println!("Size: {:?}", image.size());
println!("Data length: {:?}", image.data().len());

// Do something with image...
```

### Decoding from memory
```
use std::{io::Read, path, fs};
use rimage::{Decoder, image::InputFormat};

// Get file data
let path = path::PathBuf::from("tests/files/basi0g01.jpg"); // Or any other image
let mut file = fs::File::open(path).unwrap();
let metadata = file.metadata().unwrap();
let mut data = Vec::with_capacity(metadata.len() as usize);
file.read_to_end(&mut data).unwrap();

// Create decoder from file data and input format
let decoder = Decoder::from_mem(&data, InputFormat::Jpeg);

// Decode image to image data
let image = match decoder.decode() {
    Ok(img) => img,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Get image data
println!("Size: {:?}", image.size());
println!("Data length: {:?}", image.data().len());

// Do something with image...
```

## Encoding

```
# use rimage::Decoder;
use rimage::{Config, Encoder, image::OutputFormat};
# let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
# let decoder = Decoder::from_path(&path).unwrap();
# let image = decoder.decode().unwrap();

// Build config for encoding
let config = Config::new(OutputFormat::MozJpeg).build();

let encoder = Encoder::new(&config, image); // where image is image::ImageData

// Get encoded image data from encoder
let data = match encoder.encode() {
    Ok(data) => data,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Write image to file
std::fs::write("output.jpg", data);
# std::fs::remove_file("output.jpg").unwrap();
```
*/
#![warn(missing_docs)]

/// Errors that can occur during image processing
pub mod error;

/// Image data structs
pub mod image;

mod config;
mod decoder;
mod encoder;
mod optimize;

pub use config::Config;
pub use decoder::Decoder;
pub use encoder::Encoder;
pub use optimize::optimize;
pub use optimize::optimize_from_memory;

#[cfg(test)]
pub mod test_utils;

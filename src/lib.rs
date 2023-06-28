/*!
This crate provides a cli tool and library for image processing.
Similar to [squoosh!](https://squoosh.app/) using same codecs,
but fully written on rust and with bulk processing support.

Current features:
- Decoding jpeg and png
- Encoding with optimizations
- Get image information

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

## Decoding
```
use rimage::Decoder;

// Create decoder from file path and data
let path = std::path::PathBuf::from("tests/files/basi0g01.jpg"); // Or any other image
let file = std::fs::File::open(&path).unwrap();
let decoder = Decoder::new(&path, file);

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
use rimage::{Config, Encoder, OutputFormat};
# let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
# let file = std::fs::File::open(&path).unwrap();
# let decoder = Decoder::new(&path, file);
# let image = decoder.decode().unwrap();

// Build config for encoding
let config = match Config::build(
    75.0,
    OutputFormat::MozJpeg,
    None,
    None,
    None,
) {
    Ok(config) => config,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

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

pub use image::{ImageData, OutputFormat, ResizeType};

/// Decoders for images
#[deprecated(since = "0.2.0", note = "use the Decoder struct instead")]
pub mod decoders;
/// Encoders for images
#[deprecated(since = "0.2.0", note = "use the Encoder struct instead")]
pub mod encoders;

/// Errors that can occur during image processing
pub mod error;

/// Image data structs
pub mod image;

mod config;
mod decoder;
mod encoder;
mod memory_decoder;
mod optimize;

pub use config::Config;
pub use decoder::Decoder;
pub use encoder::Encoder;
pub use memory_decoder::MemoryDecoder;
pub use optimize::optimize;
pub use optimize::optimize_from_memory;

#[cfg(test)]
pub mod test_utils;

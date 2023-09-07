/*!
# Rimage

Rimage is a powerful Rust image optimization library designed to simplify and enhance your image optimization workflows. With Rimage, you can effortlessly optimize images for various formats, set quality levels, and apply advanced image optimization techniques with ease. Whether you're building a web application, mobile app, or desktop software, Rimage empowers you to deliver visually stunning content with minimal effort.

## Features

1. **Flexible Format Conversion**: Rimage supports all modern image formats, including JPEG, PNG, AVIF, and WebP.
2. **Quality Control**: Fine-tune the quality of your images using a simple and intuitive interface.
3. **Parallel Optimization**: Harness the power of parallel processing to optimize multiple images simultaneously.
4. **Quantization and Dithering**: For advanced users, Rimage offers control over quantization and dithering.
5. **Image Resizing**: Resize images with ease using `resize` crate.

## Decoding

From path:

```
use rimage::Decoder;

let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");

let decoder = Decoder::from_path(&path)?;

let image = decoder.decode()?;

// do something with the image data...
# Ok::<(), rimage::error::DecoderError>(())
```

From memory:
```
# use std::fs::File;
use rimage::{Decoder, config::ImageFormat};

# let f = File::open("tests/files/basi0g01.jpg").unwrap();
let reader = std::io::BufReader::new(f); // you can use any reader

let decoder = Decoder::new(reader).with_format(ImageFormat::Jpeg);

let image = decoder.decode()?;

// do something with the image data...
# Ok::<(), rimage::error::DecoderError>(())
```

## Encoding

```
use std::fs::File;

use rimage::{rgb::RGBA8, Encoder, Image, config::{EncoderConfig, Codec}};

let image_data = vec![RGBA8::new(0, 0, 0, 0); 100 * 50];
let image = Image::new(image_data, 100, 50);

let config = EncoderConfig::new(Codec::MozJpeg).with_quality(80.0).unwrap();
let file = File::create("output.jpg").expect("Failed to create file");

let encoder = Encoder::new(file, image).with_config(config);

encoder.encode()?;

# std::fs::remove_file("output.jpg").unwrap();
# Ok::<(), rimage::error::EncoderError>(())
```
*/

#![warn(missing_docs)]

///  Module for configuring image processing settings.
pub mod config;
mod decoder;
mod encoder;
///  Module for library errors.
pub mod error;
mod image;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use image::Image;

pub use resize;
pub use rgb;

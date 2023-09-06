/*!
# Rimage

Rimage is a powerful Rust image optimization library designed to simplify and enhance your image optimization workflows. With Rimage, you can effortlessly optimize images for various formats, set quality levels, and apply advanced image optimization techniques with ease. Whether you're building a web application, mobile app, or desktop software, Rimage empowers you to deliver visually stunning content with minimal effort.

## Features

1. **Flexible Format Conversion**: Rimage supports all modern image formats, including JPEG, PNG, AVIF, and WebP.
2. **Quality Control**: Fine-tune the quality of your images using a simple and intuitive interface.
3. **Parallel Optimization**: Harness the power of parallel processing to optimize multiple images simultaneously.
4. **Quantization and Dithering**: For advanced users, Rimage offers control over quantization and dithering.
5. **Image Resizing**: Resize images with ease using `resize` crate.

## Encoding

Basic usage:

```
use rimage::config::{EncoderConfig, Codec};

let config = EncoderConfig::new(Codec::MozJpeg).with_quality(80.0).unwrap();
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

pub use encoder::Encoder;
pub use image::Image;

pub use resize;
pub use rgb;

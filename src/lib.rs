/*!
# Rimage

Rimage is a powerful Rust image optimization library extending `zune_image` crate. Designed to enhance your image optimization workflows.

## Features

- Modern codecs:
    - Rimage uses modern codecs optimized to produce tiny images
    - Under the hood uses `zune_image` crate that enhances performance
- Optimization operations:
    - Rimage provides several image optimization operation
    - Resize - uses `fast_image_resize` crate that has incredible performance
    - Quantization - allowing to reduce image palette

## Formats

| Image Format | Decoder | Encoder |
|--------------|---------|---------|
| jpeg         | -       | mozjpeg |
| png          | -       | oxipng  |
| avif         | libavif | ravif   |
| webp         | webp    | webp    |

## Usage

This library is a extension for [`zune_image`] crate. For proper usage you will need to install it along with [`zune_core`].

### Decoders

```
use std::fs::File;

use rimage::codecs::avif::AvifDecoder;
use zune_image::image::Image;
# let path = "tests/files/avif/f1t.avif";

let file_content = File::open(path).unwrap();

let mut decoder = AvifDecoder::try_new(file_content).unwrap();

let img = Image::from_decoder(decoder).unwrap();
```
> `zune_image` currently doesn't support custom decoders for `read` method. So for decoding you will need hacky approach to satisfy the compiler.

### Encoders

With default options

```
use rimage::codecs::mozjpeg::MozJpegEncoder;
use zune_image::traits::EncoderTrait;
# use zune_image::image::Image;
# let img = Image::open("tests/files/jpg/f1t.jpg").unwrap();

let mut encoder = MozJpegEncoder::new();
let mut result: Vec<u8> = vec![];

encoder.encode(&img, &mut result).unwrap();
```

With custom options

```
use rimage::codecs::mozjpeg::{MozJpegOptions, MozJpegEncoder};
use zune_image::traits::EncoderTrait;
# use zune_image::image::Image;
# let img = Image::open("tests/files/jpg/f1t.jpg").unwrap();

let options = MozJpegOptions {
    quality: 80.,
    ..Default::default()
};

let mut encoder = MozJpegEncoder::new_with_options(options);

let mut result: Vec<u8> = vec![];

encoder.encode(&img, &mut result).unwrap();
```
> Note that some codecs have own implementation of options, check their documentation to learn more

### Operations

Resize

```
use rimage::operations::resize::{Resize, ResizeAlg};
use zune_image::traits::OperationsTrait;
# use zune_image::image::Image;
# let mut img = Image::open("tests/files/jpg/f1t.jpg").unwrap();

let resize = Resize::new(100, 100, ResizeAlg::Nearest);

resize.execute(&mut img).unwrap();
```
> Check [`fast_image_resize`] documentation to learn more about resize algorithms

Quantize

```
use rimage::operations::quantize::Quantize;
use zune_image::traits::OperationsTrait;
# use zune_image::image::Image;
# use zune_core::colorspace::ColorSpace;
# let mut img = Image::open("tests/files/jpg/f1t.jpg").unwrap();
# img.convert_color(ColorSpace::RGBA).unwrap();

let quantize = Quantize::new(75, None); // without dithering

quantize.execute(&mut img).unwrap();
```
*/

#![warn(missing_docs)]

/// All additional operations for the zune_image
pub mod operations;

/// All additional codecs for the zune_image
pub mod codecs;

#[cfg(test)]
mod test_utils;

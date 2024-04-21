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
| jpeg         | -       | jpegli |
| png          | -       | oxipng  |
| avif         | libavif | ravif   |
| webp         | webp    | webp    |

## Usage

This library is a extension for [`zune_image`] crate. For proper usage you will need to install it along with [`zune_core`].

### Decoders

```
use std::fs::read;

use rimage::codecs::avif::AvifDecoder;
use zune_image::traits::DecoderTrait;
use zune_core::bytestream::ZByteReader;
# let path = "tests/files/avif/f1t.avif";

let file_content = read(path).unwrap();

let reader = ZByteReader::new(file_content);

let mut decoder = AvifDecoder::try_new(reader).unwrap();

let img =
    <AvifDecoder<ZByteReader<Vec<u8>>> as DecoderTrait<Vec<u8>>>::decode(&mut decoder).unwrap();
```
> `zune_image` currently doesn't support custom decoders for `read` method. So for decoding you will need hacky approach to satisfy the compiler.

### Encoders

With default options

```
use rimage::codecs::jpegli::JpegliDecoder;
use zune_image::traits::EncoderTrait;
# use zune_image::image::Image;
# let img = Image::open("tests/files/jpg/f1t.jpg").unwrap();

let mut encoder = JpegliDecoder::new();

encoder.encode(&img).unwrap();
```

With custom options

```
use rimage::codecs::jpegli::{JpegliOptions, JpegliEncoder};
use zune_image::traits::EncoderTrait;
# use zune_image::image::Image;
# let img = Image::open("tests/files/jpg/f1t.jpg").unwrap();

let options = JpegliOptions {
    quality: 80.,
    ..Default::default()
};

let mut encoder = JpegliEncoder::new_with_options(options);

encoder.encode(&img).unwrap();
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

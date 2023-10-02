# Rimage

[![build status](https://img.shields.io/github/actions/workflow/status/SalOne22/rimage/rimage.yml?label=rimage&style=flat-square)](https://github.com/SalOne22/rimage/actions?query=branch%3Amain+)
[![docs.rs](https://img.shields.io/docsrs/rimage/latest?style=flat-square)](https://docs.rs/rimage)
[![version](https://img.shields.io/crates/v/rimage?style=flat-square)](https://crates.io/crates/rimage)
[![license](https://img.shields.io/crates/l/rimage?style=flat-square)](https://github.com/SalOne22/rimage)

A powerful Rust image optimization CLI tool and library inspired by [squoosh!](https://squoosh.app/).

## Overview

Rimage simplifies and enhances your image optimization workflows. Optimize images effortlessly, set quality levels, and apply advanced techniques with ease. Ideal for web apps, mobile apps, and desktop software.

## Features

- **Flexible Format Conversion**: Supports modern image formats: JPEG, JPEG XL, PNG, AVIF, WebP.
- **Quality Control**: Fine-tune image quality with an intuitive interface.
- **Parallel Optimization**: Optimize multiple images in parallel.
- **Quantization and Dithering**: Advanced control for experts.
- **Image Resizing**: Easy resizing with the `resize` crate.

## Installation

You can download latest release from the [releases](https://github.com/SalOne22/rimage/releases) tab.

Alternatively you can build rimage from source if you have `rust`, `cargo`, `nasm` and `cmake` installed:

```sh
cargo install rimage
```

## Usage

```
Usage: rimage [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Input file(s) to process

Options:
  -h, --help     Print help
  -V, --version  Print version

General:
  -q, --quality <QUALITY>  Optimization quality [default: 75]
  -f, --codec <CODEC>      Image codec to use [default: mozjpeg]
  -o, --output <DIR>       Write output file(s) to <DIR>
  -r, --recursive          Saves output file(s) preserving folder structure
  -s, --suffix [<SUFFIX>]  Appends suffix to output file(s) names
  -b, --backup             Appends '.backup' to input file(s) names

Quantization:
      --quantization [<QUALITY>]  Enables quantization with optional quality [default: 75]
      --dithering [<QUALITY>]     Enables dithering with optional quality [default: 75]

Resizing:
      --width <WIDTH>    Resize image with specified width
      --height <HEIGHT>  Resize image with specified height
      --filter <FILTER>  Filter used for image resizing [default: lanczos3]
```

Note that image formats may wary from features that are used when building `rimage`.

List of supported codecs with all features:

- `mozjpeg`, `jpeg`, `jpg` => mozjpeg codec
- `png` => browser png codec without compression
- `oxipng` => oxipng codec with compression
- `jpegxl`, `jxl` => jpeg xl codec
- `webp` => webp codec
- `avif` => avif codec

List of available resize filters:

- `point` => Point resizing
- `triangle` => Triangle (bilinear) resizing
- `catmull-rom`, `catrom` => Catmull-Rom (bicubic) resizing
- `mitchell` => Resize using Mitchell-Netravali filter
- `lanczos3` => Resize using Sinc-windowed Sinc with radius of 3

## Library Installation

Add Rimage to your project with Cargo:

```sh
cargo add rimage
```

Or add this to your `Cargo.toml`:

```toml
[dependencies]
rimage = "0.9.0"
```

## Library Usage

### Decoding

```rs
use rimage::Decoder;

let path = std::path::PathBuf::from(/* Your path */);

let decoder = Decoder::from_path(&path)?;

let image = decoder.decode()?;

// Handle the image...
```

### Encoding

```rs
use rimage::{rgb::RGBA8, Encoder, Image, config::{EncoderConfig, Codec}};

let image_data = vec![RGBA8::new(0, 0, 0, 0); 100 * 50];
let image = Image::new(image_data, 100, 50);

let config = EncoderConfig::new(Codec::MozJpeg).with_quality(80.0).unwrap();
let file = std::fs::File::create("output.jpg").expect("Failed to create file");

let encoder = Encoder::new(file, image).with_config(config);

encoder.encode()?; // Writes image data to the file.
```

> For full API documentation, visit [docs.rs](https://docs.rs/rimage) page.

## Contributing

Read the [contribution guide](CONTRIBUTING.md) for build instructions and guidelines.

## License

Rimage is dual-licensed under [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) and [MIT License](https://opensource.org/licenses/MIT). You can choose either license for your use.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Changelog

View the [Changelog](CHANGELOG.md) for version-specific changes.

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

Dependencies:  
On x86_64 macos requires libjxl installed.

You can download latest release from the [releases](https://github.com/SalOne22/rimage/releases) tab.

Alternatively you can build rimage from source if you have `rust`, `cargo`, `nasm` and `cmake` installed:

```sh
cargo install rimage
```

## Note

1. The `-q` opinion is disabled when pics are converted to Jxl(JpegXL) format.

2. You'd better not use the process to convert a pic with _ICC tag_ for avoiding color confusion.

3. If you're a user who just want to **use Rimage easily with a friendly GUI**, [Rimage_gui](https://github.com/Mikachu2333/rimage_gui/releases/) may be fit for you, it support both Chinese and English. Just select the version you need and download it to use.

## Usage

```text
Usage: rimage [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Input file(s) to process

Options:
  -h, --help     Print help
  -V, --version  Print version

General:
  -q, --quality <QUALITY>         Optimization image quality, disabled when use Jxl format
                                  [range: 1 - 100] [default: 75]
  -f, --codec <CODEC>             Image codec for output
                                  [possible values: jpg, png, oxipng, jxl, webp, avif] [default: jpg]
  -o, --output <DIR>              Write output file(s) to <DIR>, if "-r" option is not used
  -r, --recursive                 Saves output file(s) preserving folder structure
  -s, --suffix [<SUFFIX>]         Appends suffix to output file(s) names
  -b, --backup                    Appends ".backup" suffix to input file(s) extension
  -t, --threads                   Number of threads to use, more will run faster, but too many may crash
                                  [range: 1 - 16] [integer only] [default: number of cores]

Quantization:
      --quantization [<QUALITY>]  Enables quantization with optional quality
                                  [range: 1 - 100] [default: 75]
      --dithering [<QUALITY>]     Enables dithering with optional quality
                                  [range: 1 - 100] [default: 75]

Resizing:
      --width <WIDTH>             Resize image with specified width
                                  [integer only]
      --height <HEIGHT>           Resize image with specified height
                                  [integer only]
      --filter <FILTER>           Filter used for image resizing
                                  [possible values: point, triangle, catrom, mitchell, lanczos3] [default: lanczos3]
```

Note that image formats may wary from features that are used when building `rimage`.

### List of supprted codec (Format)

| Codec (Format)             | Input | Output | Notes                      |
| -------------------------- | ----- | ------ | -------------------------- |
| `avif`                     | √     | √      |                            |
| **`jpg`/`jpeg`/`mozjpeg`** | √     | √      | **DEFAULT**, Common&Small  |
| `webp`                     | √     | √      |                            |
| **`png`**                  | √     | √      |                            |
| -------------------------- | ----- | ------ | -------------------------- |
| `jxl`/`jpegxl`             | √     | √ \| X | `-q` opinion is disabled   |
| `oxipng`                   | X     | √      |                            |
| -------------------------- | ----- | ------ | -------------------------- |
| `bmp`                      | √     | X      |                            |
| `dds`                      | √     | X      |                            |
| `exr`                      | √     | X      |                            |
| `ff`/`farbfeld`            | √     | X      |                            |
| `gif`                      | √     | X      |                            |
| `hdr`                      | √     | X      |                            |
| `ico`                      | √     | X      |                            |
| `pam`/`pbm`/`pgm`/`ppm`    | √     | X      |                            |
| `qoi`                      | √     | X      |                            |
| `tga`                      | √     | X      |                            |
| `tif`/`tiff`               | √     | X      |                            |

_Full_ List of supported **output** codecs with all features:

- `png` => browser png codec without compression
- `oxipng` => oxipng codec with compression
- `jpegxl`, `jxl` => jpeg xl codec
- `webp` => webp codec
- `avif` => avif codec

_Full_ List of available resize filters:

- `point` => Point resizing
- `triangle` => Triangle (bilinear) resizing
- `catmull-rom`, `catrom` => Catmull-Rom (bicubic) resizing
- `mitchell` => Resize using Mitchell-Netravali filter
- `lanczos3` => Resize using Sinc-windowed Sinc with radius of 3

## Example

### png => jpg & quality => 90 & backup

| Image Path                      | Quality | Out Format | Out Dir                   | Backup |
| ------------------------------- | ------- | ---------- | ------------------------- | ------ |
| "D:\\Desktop\\input [text].png" | 90      | jpg        | "D:\\Desktop\\OutputTest" | True   |

```sh
rimage.exe "D:\\Desktop\\input [text].png" -q 90 -f jpg -o "D:\\Desktop\\OutputTest" -b
```

### suffix & recursive & quantization & dithering

| Image Path                    | Quality | Out Format | Suffix | Recursive | Quantization | Dithering |
| ----------------------------- | ------- | ---------- | ------ | --------- | ------------ | --------- |
| "C:\\中 文\\ソフトウェア.PNG" | 40      | png        | \_문자 | True      | 95           | 85        |

```sh
rimage.exe "C:\\中  文\\ソフトウェア.PNG" -q 40 --codec png -s "_문자" -r --quantization 95 --dithering 85
```

### jpg => webp & threads &resize width and height (both are opinional)

| Image Path                  | Quality | Out Format | Out Dir             | Threads | Width | Height |
| --------------------------- | ------- | ---------- | ------------------- | ------- | ----- | ------ |
| "C:\\Docs\\justfortest.JPG" | 40      | webp       | "C:\\Desktop\\Test" | 4       | 60    | 10     |

```sh
rimage.exe "C:\\Docs\\justfortest.PNG" --quality 40 --codec webp --output "C:\\Desktop\\Test" --threads 4 --width 60 --height 10
```

## Library Installation

Add Rimage to your project with Cargo:

```sh
cargo add rimage
```

Or add this to your `Cargo.toml`:

```toml
[dependencies]
rimage = "0.10.3"
```

## Library Usage

### Decoding

```rs
use rimage::Decoder;

let decoder = Decoder::from_path("image.jpg")?;

let image = decoder.decode()?;

// do something with the image data...
```

### Encoding

```rs
use std::fs::File;

use rimage::{rgb::RGBA8, Encoder, config::{EncoderConfig, Codec}};
use image::{RgbaImage, DynamicImage};

let image_data = vec![0; 100 * 50 * 4];
let image = RgbaImage::from_raw(100, 50, image_data)?;

let config = EncoderConfig::new(Codec::MozJpeg).with_quality(80.0)?;
let file = File::create("output.jpg")?;

let encoder = Encoder::new(file, DynamicImage::ImageRgba8(image)).with_config(config);

encoder.encode()?;
```

> For full API documentation, visit [docs.rs](https://docs.rs/rimage) page.

## Contributing

Read the [contribution guide](CONTRIBUTING.md) for build instructions and guidelines.

## License

Rimage is dual-licensed under [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) and [MIT License](https://opensource.org/licenses/MIT). You can choose either license for your use.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Changelog

View the [Changelog](CHANGELOG.md) for version-specific changes.

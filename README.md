# Rimage

[![build status](https://img.shields.io/github/actions/workflow/status/SalOne22/rimage/rimage.yml?label=rimage&style=flat-square)](https://github.com/SalOne22/rimage/actions?query=branch%3Amain+)
[![docs.rs](https://img.shields.io/docsrs/rimage/latest?style=flat-square)](https://docs.rs/rimage)
[![version](https://img.shields.io/crates/v/rimage?style=flat-square)](https://crates.io/crates/rimage)
[![license](https://img.shields.io/crates/l/rimage?style=flat-square)](https://github.com/SalOne22/rimage)

A powerful Rust image optimization CLI tool inspired by [squoosh!](https://squoosh.app/).

## Features

- Modern codecs:
  - Rimage uses modern codecs optimized to produce tiny images
  - Under the hood uses `zune_image` crate that enhances performance
- Optimization operations:
  - Rimage provides several image optimization operation
  - Resize - uses `fast_image_resize` crate that has incredible performance
  - Quantization - allowing to reduce image palette

## Installation

You can download latest release from the [releases](https://github.com/SalOne22/rimage/releases) tab.

Alternatively you can build rimage from source if you have `cargo` installed:

```sh
cargo install rimage
```

> ### Note
>
> If you're a user who just want to **use Rimage easily with a friendly GUI**, [Rimage_gui](https://github.com/Mikachu2333/rimage_gui/releases/) may be fit for you, it support both Chinese and English. Just select the version you need and download it to use.

## Usage

For library usage check [Docs.rs](https://docs.rs/rimage/latest/rimage/)

### List of supported Codecs

| Image Format | Decoder       | Encoder                 | NOTE                                                 |
| ------------ | ------------- | ----------------------- | ---------------------------------------------------- |
| bmp          | zune-bmp      | -                       | Input only                                           |
| jpeg         | zune-jpeg     | mozjpeg or jpeg-encoder |                                                      |
| png          | zune-png      | oxipng or zune-png      | Static pics only                                     |
| avif         | libavif       | ravif                   | Only common features are supported, Static pics only |
| webp         | webp          | webp                    | Static pics only                                     |
| ppm          | zune-ppm      | zune-ppm                |                                                      |
| qoi          | zune-qoi      | zune-qoi                |                                                      |
| farbfeld     | zune-farbfeld | zune-farbfeld           |                                                      |
| psd          | zune-psd      | -                       | Input only                                           |
| jpeg-xl      | jxl-oxide     | zune-jpegxl             | Lossless Output only                                 |
| hdr          | zune-hdr      | zune-hdr                |                                                      |

### List of supported preprocessing options

- Resize
- Quantization

## Contributing

Read the [contribution guide](CONTRIBUTING.md) for build instructions and guidelines.

## License

Rimage is dual-licensed under [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) and [MIT License](https://opensource.org/licenses/MIT). You can choose either license for your use.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Changelog

View the [Changelog](CHANGELOG.md) for version-specific changes.

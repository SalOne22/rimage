# Rimage

[![Build Status](https://img.shields.io/github/actions/workflow/status/SalOne22/rimage/rimage.yml?label=rimage)](https://github.com/SalOne22/rimage/actions?query=branch%3Amain+)
[![docs.rs](https://img.shields.io/docsrs/rimage/latest)](https://docs.rs/rimage)
[![Version](https://img.shields.io/crates/v/rimage)](https://crates.io/crates/rimage)
[![License](https://img.shields.io/crates/l/rimage)](https://github.com/SalOne22/rimage)

This is CLI tool inspired by [squoosh!](https://squoosh.app/)  
Rimage currently supports two codecs, mozjpeg and oxipng, and aims to add support for AVIF and WebP in the future.

## Installation

If you have cargo you can use this

```
cargo install rimage
```

or from [Releases](https://github.com/SalOne22/rimage/releases) on GitHub

## Usage

```
rimage -q 75 *.jpg
```

- Quality: `-q 0` through `-q 100`, higher is better
- Output format: `-o png`, currently supported only jpg, png and oxipng
- Suffix for output: `-s _updated`, adds suffix in file name ("input.jpg" -> "input_updated.jpg")
- Info: `-i`, flag used to get info about images (size and data length)
- More options will be added later

## To-Do

- Bulk image optimization in parallel
- Support for AVIF and WebP
- Image resize
- Image quantization
- And allot of bugfixes and optimizations

## Contribute

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Any help would be greatly appreciated!

## License

Rimage is licensed under either the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0), or [the MIT license](https://opensource.org/licenses/MIT).

All images are taken from [PNGSuite](http://www.schaik.com/pngsuite/)

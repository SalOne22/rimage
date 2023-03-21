# Rimage

[![Build Status](https://img.shields.io/github/actions/workflow/status/SalOne22/rimage/rust.yml?label=rimage)](https://github.com/SalOne22/rimage/actions?query=branch%3Amain+)
![Version](https://img.shields.io/crates/v/rimage)
![License](https://img.shields.io/crates/l/rimage)

This is CLI tool inspired by [squoosh!](https://squoosh.app/)  
Rimage currently supports two codecs, mozjpeg and oxipng, and aims to add support for AVIF and WebP in the future.

## Usage

`rimage -q 0.75 *.jpg`

- Quality: `-q 0` through `-q 1`, higher is better
- More options will be added later

## To-Do

- Bulk image optimization in parallel
- Support for AVIF and WebP
- Image resize
- Image quantization

## Contribute

If you are interested in contributing to the development of Rimage, you can get started by cloning the repository using Git or GitHub Desktop, followed by running `cargo build`.
Once you have created your branch, you can make a pull request.  
Im new to GitHub so any help would be greatly appreciated! 🤘

## License

Rimage is licensed under either the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0), or [the MIT license](https://opensource.org/licenses/MIT).

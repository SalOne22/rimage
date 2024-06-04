# Rimage

[![build status](https://img.shields.io/github/actions/workflow/status/SalOne22/rimage/rimage.yml?label=rimage&style=flat-square)](https://github.com/SalOne22/rimage/actions?query=branch%3Amain+)
[![docs.rs](https://img.shields.io/docsrs/rimage/latest?style=flat-square)](https://docs.rs/rimage)
[![version](https://img.shields.io/crates/v/rimage?style=flat-square)](https://crates.io/crates/rimage)
[![license](https://img.shields.io/crates/l/rimage?style=flat-square)](https://github.com/SalOne22/rimage)

A powerful Rust image optimization CLI tool inspired by [squoosh!](https://squoosh.app/).

:warning: WARNING: This documentation works only for latest version of rimage! You can get latest version from [releases](https://github.com/SalOne22/rimage/releases) tab or explicitly with cargo: `cargo install rimage@0.11.0-next.1`

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

If you're a Rust programmer, rimage can be installed with `cargo`.

```sh
cargo install rimage
```

Alternatively, one can use [cargo binstall](https://github.com/cargo-bins/cargo-binstall) to install a rimage binary directly from GitHub:

```sh
cargo binstall rimage
```

> ### Note
>
> If you're a user who just want to **use Rimage easily with a friendly GUI**, [Rimage_gui](https://github.com/Mikachu2333/rimage_gui/releases/) may be fit for you, it support both Chinese and English. Just select the version you need and download it to use.

## Usage

```
Optimize images natively with best-in-class codecs

Usage: rimage [COMMAND]

Commands:
  avif      Encode images into AVIF format. (Small and Efficient)
  farbfeld  Encode images into Farbfeld format. (Bitmapped)
  jpeg      Encode images into JPEG format. (Progressive-able)
  jpeg_xl   Encode images into JpegXL format. (Big but Lossless)
  mozjpeg   Encode images into JPEG format using MozJpeg codec. (RECOMMENDED and Small)
  oxipng    Encode images into PNG format using OxiPNG codec. (Progressive-able)
  png       Encode images into PNG format.
  ppm       Encode images into PPM format. (Bitmapped)
  qoi       Encode images into QOI format. (Trendy and Small)
  webp      Encode images into WebP format. (Lossless-able)
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Basic optimization suitable for web

To optimize images with great defaults, you can simply call `rimage <command>`. For example:

```sh
rimage mozjpeg ./image.jpg
```

By default rimage will place output images right in place of precious images, resulting in overwrite if input and output has the same format. To change this behavior you can use this options:

```sh
# will place output images in `./output` directory, images may be overwritten if has the same name
rimage mozjpeg -d ./output ./image.jpg

# will rename all input files before processing with `@backup` suffix
rimage mozjpeg --backup ./image.jpg

# will place output images in ./output directory preserving folder structure
rimage mozjpeg -d ./output -r ./inner/image.jpg ./image.jpg
```

### Preprocessing

Rimage has pipeline preprocessing options. Simple usage:

```sh
# will resize image to specified dimensions
rimage mozjpeg --resize 500x200 ./image.jpg
```

If you want to run preprocessing pipeline in specific order, you can do this:

```sh
# will quantize image with 80% quality, after run resize to 64x64 pixels using the Nearest filter.
rimage mozjpeg --quantization 80 --resize 64x64 --filter nearest ./image.jpg

# will resize image to 64x64 pixels using the Nearest filter, and after run quantization with 80% quality.
rimage mozjpeg --resize 64x64 --filter nearest --quantization 80 ./image.jpg
```

Note that some preprocessing option are order independent. For example filter option, will apply resize filter to all resize invocations. Same for dithering, applies to every quantization invocations.

### Advanced options

If you want customize optimization you can provide additional options to encoders. For mozjpeg this options are valid:

```
Options:
  -q, --quality <NUM>         Quality, values 60-80 are recommended. [default: 75]
      --chroma_quality <NUM>  Separate chrome quality.
      --baseline              Set to use baseline encoding (by default is progressive).
      --no_optimize_coding    Set to make files larger for no reason.
      --smoothing <NUM>       Use MozJPEG's smoothing.
      --colorspace <COLOR>    Set color space of JPEG being written. [default: ycbcr] [possible values: ycbcr, grayscale, rgb]
      --multipass             Specifies whether multiple scans should be considered during trellis quantization.
      --subsample <PIX>       Sets chroma subsampling.
      --qtable <TABLE>        Use a specific quantization table. [default: NRobidoux] [possible values: AhumadaWatsonPeterson, AnnexK, Flat, KleinSilversteinCarney, MSSSIM, NRobidoux, PSNRHVS, PetersonAhumadaWatson, WatsonTaylorBorthwick]
```

For more info use `rimage help <command>`

For library usage check [Docs.rs](https://docs.rs/rimage/latest/rimage/)

### List of supported Codecs

| Image Codecs | Decoder       | Encoder                 | NOTE                                                 |
| ------------ | ------------- | ----------------------- | ---------------------------------------------------- |
| avif         | libavif       | ravif                   | Common features only, Static only                    |
| bmp          | zune-bmp      | X                       | Input only                                           |
| farbfeld     | zune-farbfeld | zune-farbfeld           |                                                      |
| hdr          | zune-hdr      | zune-hdr                |                                                      |
| jpeg         | zune-jpeg     | mozjpeg or jpeg-encoder | Multifunctional when use mozjpeg encoder             |
| jpeg-xl      | jxl-oxide     | zune-jpegxl             | Lossless only                                        |
| png          | zune-png      | oxipng or zune-png      | Static only, Multifunctional when use oxipng encoder |
| ppm          | zune-ppm      | zune-ppm                |                                                      |
| psd          | zune-psd      | X                       | Input only                                           |
| qoi          | zune-qoi      | zune-qoi                |                                                      |
| webp         | webp          | webp                    | Static only                                          |

### List of supported preprocessing options

- Resize
- Quantization
- Alpha premultiply

## Known bugs

- **Dir path end with `\` may cause rimage crashes** due to a cmd bug [#72653](https://github.com/rust-lang/rust/issues/72653).

### Example:

This will crash:

```sh
rimage png "D:\example.jpg" -d "D:\desktop\" -s "suffix"
```

This will work as expected:

```sh
rimage png "D:\example.jpg" -d "D:\desktop" -s "suf test" # without trailing backslash

rimage png "D:\example.jpg" -s "suffix"  -d "D:\desktop\" # backslash at the end
```

## Contributing

Read the [contribution guide](CONTRIBUTING.md) for build instructions and guidelines.

## License

Rimage is dual-licensed under [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) and [MIT License](https://opensource.org/licenses/MIT). You can choose either license for your use.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Changelog

View the [Changelog](CHANGELOG.md) for version-specific changes.

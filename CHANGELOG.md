# Changelog

All notable changes to the Rimage library will be documented in this file.

## v0.11.0

### Breaking changes

- Another complete rewrite of the library, please use `zune-image` with this crate.
- CLI has been rewritten to a new, more flexible interface

Improved performance (alot). To libvips performance, like to the moon, but still.
Benchmark data was taken from [sharp](https://github.com/lovell/sharp) repository.

Options `--directory /tmp --resize 500x200 --filter lanczos3 mozjpeg --quality 80 -- ./2569067123_aca715a2ee_o.jpg`

One image [from here](https://github.com/lovell/sharp/blob/main/test/fixtures/2569067123_aca715a2ee_o.jpg):
```
Benchmark 1: rimage-0.11.0-next
  Time (mean ± σ):      48.6 ms ±   1.3 ms    [User: 43.9 ms, System: 8.4 ms]
  Range (min … max):    46.7 ms …  53.3 ms    59 runs

Benchmark 2: rimage-0.10.3
  Time (mean ± σ):     691.6 ms ±   5.2 ms    [User: 709.3 ms, System: 10.1 ms]
  Range (min … max):   687.1 ms … 705.0 ms    10 runs

Benchmark 3: squoosh-cli
  Time (mean ± σ):     760.6 ms ±  29.7 ms    [User: 2155.7 ms, System: 291.2 ms]
  Range (min … max):   738.0 ms … 829.5 ms    10 runs

Summary
  rimage-0.11.0-next ran
   14.23 ± 0.39 times faster than rimage-0.10.3
   15.65 ± 0.74 times faster than squoosh-cli
```

Options `--directory /tmp --resize 500x200 --filter lanczos3 mozjpeg --quality 80 -- ./*.jpg`

Multiple images in parallel [from here](https://github.com/lovell/sharp/tree/main/test/fixtures) (only jpg):
```
Benchmark 1: rimage-0.11.0-next
  Time (mean ± σ):     177.2 ms ±   4.6 ms    [User: 999.5 ms, System: 38.7 ms]
  Range (min … max):   169.3 ms … 184.7 ms    17 runs

Benchmark 2: rimage-0.10.3
  Time (mean ± σ):     26.399 s ±  0.336 s    [User: 29.653 s, System: 0.089 s]
  Range (min … max):   26.045 s … 27.297 s    10 runs

Summary
  rimage-0.11.0-next ran
  148.99 ± 4.31 times faster than rimage-0.10.3
```

## [v0.10.3](https://github.com/SalOne22/rimage/releases/tag/v0.10.3)

### Changes

- Added support for all image formats that supported by `image` crate

## [v0.10.2](https://github.com/SalOne22/rimage/releases/tag/v0.10.2)

### Bug Fixes

- fixed bug when resize option doesn't do anything

## [v0.10.1](https://github.com/SalOne22/rimage/releases/tag/v0.10.1)

### Changes

- Removed unused dependencies
- Updated readme and help section
- Reduced size of the binary

## [v0.10.0](https://github.com/SalOne22/rimage/releases/tag/v0.10.0)

### Breaking Changes

- Replaced `image` module: Now re-exporting the [`image`](https://crates.io/crates/image) crate, which may affect existing functionalities.
- Refactored Decoder and Encoder: Changes in method signatures and behavior could potentially break existing code.
- Removed support for windows-gnu targets due to full msvc support.

### New features

- Added support for musl targets
- Added support for wasm targets (emscripten and wasi)
- Added full support for all features on windows i686 (x86) and x86_64 targets

### Changes

- Changed codec for jxl compression. Shifted to support only lossless compression for improved efficiency and portability.
- Integration with image crate: Refactored Decoder and Encoder modules to utilize the image crate, enhancing functionality.

### Additional Notes

- Dependencies Update: Adjusted dependencies to align with new implementations and removed outdated ones.

## [v0.9.1](https://github.com/SalOne22/rimage/releases/tag/v0.9.1)

### New features

- Added previously removed `--threads` option when parallel feature is enabled.

### Changes

- Temporarily removed from plans stdin input due to complexity.

## [v0.9.0](https://github.com/SalOne22/rimage/releases/tag/v0.9.0)

### Breaking Changes

- **Library Structure Rewrite**: The library structure has been entirely rewritten, resulting in no backward compatibility.

### Changes

- Output directory now works differently, folder structure only preserves with `--recursive` flag.
- `--format` flag renamed to `--codec`, `-f` shortcut is not affected.
- Removed stdin input, no more freezes when input files is not provided.
  > Stdin input will be returned in latest releases.
- Removed progress bar because of issues with `indicatif` crate.
  > Progress bar will be returned in latest releases.

### Refactor

- Divided crate to separate features to improve modularity.
- Divided `Config` into several parts for improved modularity.
- Moved `Codec` and `ImageFormat` into the `config` module.
- Updated the `Decoder` and `Encoder` structs with more concise interfaces.
- Updated error messages and handling to align with the new library structure.
- Enhanced `from_path` in `Decoder` to handle image format and orientation.

### New Features

- Introduced `EncoderConfig` for configuring encoding.
- Introduced `ResizeConfig` for configuring image resizing (optional).
- Introduced `QuantizationConfig` for configuring image quantization (optional).
- Added `fixed_orientation` to `Decoder` to save image orientation according to EXIF tag.
- Added `-b --backup` flag to backup input files in case of replacing.

### Bug Fixes

- When glob cannot find any files, files provided by user will be processed.

## [v0.8.2](https://github.com/SalOne22/rimage/releases/tag/v0.8.2)

### Bug Fixes

- Fixed an issue where extensions were written in uppercase.

## [v0.8.1](https://github.com/SalOne22/rimage/releases/tag/v0.8.1)

### Enhancements

- Updated progress bar. ![progress_bar](https://github.com/SalOne22/rimage/assets/111443297/847d30df-54e4-40c8-9d02-f67a66f140a8)
- Rimage now uses the rayon crate for parallel optimizations.
- Added `--quiet` flag that disables the progress bar.

## [v0.8.0](https://github.com/SalOne22/rimage/releases/tag/v0.8.0)

### Breaking Changes

- `Decoder` now acts as a builder for `GenericDecoder` capable of decoding byte slices and files.
- `Config` now uses a builder pattern.
- Renamed `OutputFormat` to `Codec`.
- Errors are now more declarative.
- Removed `decoders` and `encoders`.
- Introduced `optimize` and `optimize_from_memory` functions.
- Added `ImageFormat` for memory decoding.
- Fixed jpeg decoding.

### Enhancements

- Decoder now accepts an opened file as input.
- `ImageData` now stores bytes as `Box<[u8]>`.
- Global allocator is now Jemalloc on Unix and MiMalloc on Windows.
- Reduced peak heap usage by half.
- Output format option is now named simply `format`.
- Added AVIF decoding and encoding.
- Added output directory option for saving in different locations.

## [v0.7.1](https://github.com/SalOne22/rimage/releases/tag/v0.7.1)

### Bug Fixes

- Fixed an issue where extensions were written in uppercase; now they are all normalized to lowercase.

## [v0.7.0](https://github.com/SalOne22/rimage/releases/tag/v0.7.0)

### Breaking Changes

- Decoder now accepts an opened file as input.

### Enhancements

- Added AVIF decoding and encoding.
- Added an output directory option for saving in different locations.

## [v0.6.0](https://github.com/SalOne22/rimage/releases/tag/v0.6.0)

### New Features

- Added WebP decoding and encoding.

### Bug Fixes

- Fixed a typo in logs.

## [v0.5.1](https://github.com/SalOne22/rimage/releases/tag/v0.5.1)

### New Features

- Added logging of errors and info.

### Changes

- Replaced `eprintln!` with `error!`.

## [v0.5.0](https://github.com/SalOne22/rimage/releases/tag/v0.5.0)

### New Features

- Added image resize functionality.
- Introduced a resize error to `EncodingError`.
- Added width and height arguments to CLI.
- Added a resize filter type argument to CLI.

### Changes

- `Config::build` now requires 5 arguments.

## [v0.4.0](https://github.com/SalOne22/rimage/releases/tag/v0.4.0)

### New Features

- Added image quantization.
- Introduced quantization error to `EncodingError`.
- Added a `data_mut` function to `ImageData`.
- Added an `encode_quantized` function.
- Added a quantization argument to CLI.
- Added a dithering argument to CLI.

## [v0.3.0](https://github.com/SalOne22/rimage/releases/tag/v0.3.0)

### New Features

- Added parallelism.
- Added a thread number option to use (Default: number of CPUs).

### Changes

- Replaced strings in errors with `SimpleError`.

## [v0.2.1](https://github.com/SalOne22/rimage/releases/tag/v0.2.1)

### Changes

- Updated the Readme.

## [v0.2.0](https://github.com/SalOne22/rimage/releases/tag/v0.2.0)

### New Features

- Introduced `ImageData` for storing image data.
- Introduced `Decoder` to decode images.
- Introduced `Encoder` to encode images.
- Introduced error struct's in `rimage::errors`.
- Added image ## [v0.1.processing from stdio.
- Added an info option.
- Added a suffix option.

### Changes

- Deprecated `decoders::decode_image` and `encoders::encode_image`; use `Decoder` and `Encoder` struct's instead.
- Improved documentation for almost all functions and struct's with examples.
- Added support for PNG as output (not oxipng codec).

## [v0.1.3](https://github.com/SalOne22/rimage/releases/tag/v0.1.3)

### Bug Fixes

- Fixed long processing of PNG images.

## [v0.1.2](https://github.com/SalOne22/rimage/releases/tag/v0.1.2)

### New Features

- Added a pretty progress bar.

## [v0.1.1](https://github.com/SalOne22/rimage/releases/tag/v0.1.1)

### Bug Fixes

- Fixed a hardcoded format output.
- Added support for RGBA images.

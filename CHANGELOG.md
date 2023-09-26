# Changelog

All notable changes to the Rimage library will be documented in this file.

## **[Unreleased]** v0.9.0

### Breaking Changes

- **Library Structure Rewrite**: The library structure has been entirely rewritten, resulting in no backward compatibility.

### Changes

- Output directory now works differently, folder structure only preserves with `--recursive` flag.
- Input from stdin now works with `-` as input, no more freezes when input files is not provided.
- `--format` flag renamed to `--codec`, `-f` shortcut is not affected.
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
- Added `-b --backup` flag to backup input files in case of replacing.

## [v0.8.2](https://github.com/SalOne22/rimage/releases/tag/v0.8.2)

### Bug Fixes

- Fixed an issue where extensions were written in uppercase.

## [v0.8.1](https://github.com/SalOne22/rimage/releases/tag/v0.8.1)

### Enhancements

- Updated progress bar. ![Updated version](./assets/progress_bar.gif)
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

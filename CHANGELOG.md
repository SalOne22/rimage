## v0.8.1

- [Changed] Updated progress bar. ![Updated version](./assets/progress_bar.gif)
- [Changed] Now rimage uses rayon crate for parallel optimizations
- [Added] `--quiet` flag that disables progress bar

## v0.8.0

- [Breaking Changes] `Decoder` now is builder for `GenericDecoder` that capable to decode byte slices and files
- [Breaking Changes] `Config` now uses builder pattern
- [Changed] `OutputFormat` is renamed to `Codec`
- [Changed] Errors now is more declarative
- [Removed] `decoders` and `encoders`
- [Added] `optimize` and `optimize_from_memory` functions
- [Added] `ImageFormat` for memory decoding
- [Bugfix] Fixed jpeg decoding

## v0.7.1

- [Bugfix] Fixed extensions written in uppercase, now they are all normalized to lowercase

## v0.7.0

- [Breaking Changes] Decoder now accept opened file as input
- [Changed] ImageData now stores bytes as Box<[u8]>
- [Changed] Global allocator now is Jemalloc on unix and MiMalloc on Windows
- [Changed] Reduced peak heap usage twice!
- [Changed] Now output format option is named just format
- [Added] AVIF decoding and encoding
- [Added] Output dir option for saving in different locations

## v0.6.0

- [Added] WebP decoding and encoding
- [Bugfix] Fixed typo in logs

## v0.5.1

- [Added] Logging of errors and info
- [Changed] `eprintln!` now `error!`

## v0.5.0

- [Added] Image Resize
- [Added] Resize error to EncodingError
- [Added] Width and Height arguments to CLI
- [Added] Resize filter type argument to CLI
- [Changed] `Config::build` now require 5 arguments

## v0.4.0

- [Added] Image quantization
- [Added] Quantization error to EncodingError
- [Added] data_mut function to ImageData
- [Added] Encode quantized function
- [Added] Quantization argument to CLI
- [Added] Dithering argument to CLI

## v0.3.0

- [Added] Parallelism
- [Added] Thread number to use option (Default: number of cpus)
- [Changed] Strings in errors replaced with SimpleError

## v0.2.1

- [Changed] Readme updated
- [Changed] Updated regex to 1.7.3

## v0.2.0

- [Added] struct `ImageData` for storing images data
- [Added] struct `Decoder` to decode images
- [Added] struct `Encoder` to encode images
- [Added] structs for errors in `rimage::errors`

- [Added] image processing from stdio
- [Added] info option
- [Added] suffix option

- [Changed] `decoders::decode_image` and `encoders::encode_image` now deprecated, use `Decoder` and `Encoder` structs instead
- [Improvement] Added documentation to almost all functions and structs with examples
- [Improvement] Added support for png as output (not oxipng)

## v0.1.3

- [Bugfix] Fixed long processing of png images

## v0.1.2

- [Added] Added pretty progress bar

## v0.1.1

- [Bugfix] Fixed hardcoded format output
- [Improvement] Added support for RGBA images

## v0.4.0

- [Added] Image quantization
- [Added] Quantization error to EncodingError
- [Added] data_mut function to ImageData
- [Added] Encode quantized function
- [Added] Quantization argument to CLI

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

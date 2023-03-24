## v0.2.0

- [Added] struct `ImageData` for storing images
- [Added] struct `DecodingError` to process errors
- [Added] struct `Decoder` to decode images

- [Added] image processing from stdio
- [Added] info option

- [Changed] `decoders::decode_image` and `encoders::encode_image` now deprecated, use `Decoder` and `Encoder` structs instead

## v0.1.3

- [Bugfix] Fixed long processing of png images

## v0.1.2

- [Added] Added pretty progress bar

## v0.1.1

- [Bugfix] Fixed hardcoded format output
- [Improvement] Added support for RGBA images

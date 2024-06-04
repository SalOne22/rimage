# Changelog

All notable changes to the Rimage library will be documented in this file.

## v0.11.0-next.1

### Bug Fixes

- **cli:** fixed encoder function ([1a6bf92](https://github.com/SalOne22/rimage/commit/1a6bf92f649787508732a1c769b677997286ac1e))
- **codecs:** fixed clippy errors ([7f3c261](https://github.com/SalOne22/rimage/commit/7f3c261b55a519f26e615e0094edfa5970005d98))
- **codecs:** updated avif decoder to new api ([1c8ea8e](https://github.com/SalOne22/rimage/commit/1c8ea8e998b890299c52ece172011f2692c3d0c1))
- **codecs:** updated avif encoder to new api ([d07193d](https://github.com/SalOne22/rimage/commit/d07193d1881a8f2392e91b64656fd0b624c98c3b))
- **codecs:** updated mozjpeg encoder to new api ([b408b31](https://github.com/SalOne22/rimage/commit/b408b3116a90db19d51581c511ed7168c981209c))
- **codecs:** updated oxipng encoder to new api ([fdfcbd9](https://github.com/SalOne22/rimage/commit/fdfcbd9710a275a15d6d756a7c086b9da491e888))
- **codecs:** updated webp decoder to new api ([98728e0](https://github.com/SalOne22/rimage/commit/98728e0123fcdc9000dd0bd4bee8d9cef0b45800))
- **codecs:** updated webp encoder to new api ([ede4784](https://github.com/SalOne22/rimage/commit/ede47843c384d345d46f9172e9f0f6240768d329))
- **workflow:** :construction_worker: removed check for unique branch ([0505c62](https://github.com/SalOne22/rimage/commit/0505c62e884a5ccec9f58c7f8f5185d91f62501a))
- **workflow:** :green_heart: fixed macos version ([17eeba1](https://github.com/SalOne22/rimage/commit/17eeba1540e6aac0d4121ddbe1676fe6bf4759a2))
- fixed clippy issues ([0045b5b](https://github.com/SalOne22/rimage/commit/0045b5bc06c99d4c0d4a4bda8042d1214b8690b7))

### Features

- **cli:** :children_crossing: added warn when input is not a file ([dd2fe3e](https://github.com/SalOne22/rimage/commit/dd2fe3ec7b46beae3c9d619cac303cc15d74167d))
- **operations:** added "apply icc profile" operation ([e12b7c4](https://github.com/SalOne22/rimage/commit/e12b7c4373b025f00041a2ba92d86781a30feda7))

## [v0.11.0-next.0](https://github.com/SalOne22/rimage/releases/tag/v0.11.0-next.0)

### Breaking changes

- Another complete rewrite of the library, please use `zune-image` with this crate.
- CLI has been rewritten to a new, more flexible interface

Improved performance (alot). To libvips performance, like to the moon, but still.
Benchmark data was taken from [sharp](https://github.com/lovell/sharp) repository.

Options `--directory /tmp --resize 500x200 --filter lanczos3 mozjpeg --quality 80 -- ./2569067123_aca715a2ee_o.jpg`

One image [from here](https://github.com/lovell/sharp/blob/main/test/fixtures/2569067123_aca715a2ee_o.jpg):

```
Benchmark 1: rimage-0.11.0-next.0
  Time (mean ± σ):      48.6 ms ±   1.3 ms    [User: 43.9 ms, System: 8.4 ms]
  Range (min … max):    46.7 ms …  53.3 ms    59 runs

Benchmark 2: rimage-0.10.3
  Time (mean ± σ):     691.6 ms ±   5.2 ms    [User: 709.3 ms, System: 10.1 ms]
  Range (min … max):   687.1 ms … 705.0 ms    10 runs

Benchmark 3: squoosh-cli
  Time (mean ± σ):     760.6 ms ±  29.7 ms    [User: 2155.7 ms, System: 291.2 ms]
  Range (min … max):   738.0 ms … 829.5 ms    10 runs

Summary
  rimage-0.11.0-next.0 ran
   14.23 ± 0.39 times faster than rimage-0.10.3
   15.65 ± 0.74 times faster than squoosh-cli
```

Options `--directory /tmp --resize 500x200 --filter lanczos3 mozjpeg --quality 80 -- ./*.jpg`

Multiple images in parallel [from here](https://github.com/lovell/sharp/tree/main/test/fixtures) (only jpg):

```
Benchmark 1: rimage-0.11.0-next.0
  Time (mean ± σ):     177.2 ms ±   4.6 ms    [User: 999.5 ms, System: 38.7 ms]
  Range (min … max):   169.3 ms … 184.7 ms    17 runs

Benchmark 2: rimage-0.10.3
  Time (mean ± σ):     26.399 s ±  0.336 s    [User: 29.653 s, System: 0.089 s]
  Range (min … max):   26.045 s … 27.297 s    10 runs

Summary
  rimage-0.11.0-next.0 ran
  148.99 ± 4.31 times faster than rimage-0.10.3
```

### Features

- added avif decoder ([b64b931](https://github.com/SalOne22/rimage/commit/b64b931540c122ee3ba17ee97aa6e017d0ee9e0b))
- added avif encoder ([18ee7e3](https://github.com/SalOne22/rimage/commit/18ee7e3163f591257e6f965cd90a8fbb58f4bd39))
- added fast resize operation ([4317475](https://github.com/SalOne22/rimage/commit/4317475c63868e2e11039b36d012c7931f9ba0d8))
- added features to reduce lib size ([e23823f](https://github.com/SalOne22/rimage/commit/e23823f23bc3925ca272028f4ce3f0677736b169))
- added long_about to jxl codec ([8a19d4b](https://github.com/SalOne22/rimage/commit/8a19d4b71ed7e586b1b7a119ad83ff7369cd4bc3))
- added mozjpeg encoder ([7f32068](https://github.com/SalOne22/rimage/commit/7f3206835505049203577aa67c0f12791aa650d2))
- added options to jpeg codec ([ccd249c](https://github.com/SalOne22/rimage/commit/ccd249c60d898944ca03eb38158acc4be78427a1))
- added oxipng encoder ([4a824dc](https://github.com/SalOne22/rimage/commit/4a824dc17a7eb4f7994cff9d5b5695216dd775fd))
- added quantization operation ([25d0d78](https://github.com/SalOne22/rimage/commit/25d0d784202b730c8c1a5de072b189263e3bd1ab))
- added threading to resize operation ([1e89b34](https://github.com/SalOne22/rimage/commit/1e89b345330d4a95d134af1203044f1f2be54e39))
- added webp decoder ([3916d28](https://github.com/SalOne22/rimage/commit/3916d28f84b2dbb42814a8a15b3eee7075c9a3f7))
- added webp encoder ([d542749](https://github.com/SalOne22/rimage/commit/d542749ff61c883ee01fd2a63b7f45c7c1df1c95))
- **cli/help:** added codecs support section ([8578367](https://github.com/SalOne22/rimage/commit/8578367fdf32f17eb7fe67e891c65a0ff1c470d6))
- **cli:** added alpha premultiply preprocessor ([33abc88](https://github.com/SalOne22/rimage/commit/33abc8816223cb4d338037d17ace2b26aa7d8321))
- **cli:** added avif codec ([aa89db5](https://github.com/SalOne22/rimage/commit/aa89db55b2aa4fa47ee72fce2ea5083d63069ff6))
- **cli:** added base cli options ([a22a0aa](https://github.com/SalOne22/rimage/commit/a22a0aa1872a3c295b4812e9e7bc70ef3d4ded5f))
- **cli:** added base codecs ([60d9aa5](https://github.com/SalOne22/rimage/commit/60d9aa58e3d606c99896421340cc7405fe02c315))
- **cli:** added base preprocessors ([99d6a33](https://github.com/SalOne22/rimage/commit/99d6a3331ef43d3cbfbf6d9116b26651c57b742a))
- **cli:** added mozjpeg codec ([37359e2](https://github.com/SalOne22/rimage/commit/37359e2d87372001b79af45ddb20439d4d75426d))
- **cli:** added oxipng codec ([9658fce](https://github.com/SalOne22/rimage/commit/9658fce2475be5de31ec92d2d70da3ae89fe5f31))
- **cli:** added webp codec ([57db180](https://github.com/SalOne22/rimage/commit/57db180cdcf3f0189c9ba83f97527a9bedc53242))
- **cli:** changed general options placement ([5cf1e96](https://github.com/SalOne22/rimage/commit/5cf1e9678b6cdbe198da1f796ede25ef4edf00ee))
- **cli:** implemented main cli ([a666dbf](https://github.com/SalOne22/rimage/commit/a666dbf294d14ddb8dbb73dcd240e4dfb2125834))
- **encoders:** added exif write support for jpeg and png ([fcd4f5e](https://github.com/SalOne22/rimage/commit/fcd4f5e47fa6f5b8bf212329fa65019d5fed642c))
- implemented base cli pipeline ([18bcbe0](https://github.com/SalOne22/rimage/commit/18bcbe0c2405d36e140c360547d700b8060abc96))
- **preprocessors:** implemented quantization operation ([e527d69](https://github.com/SalOne22/rimage/commit/e527d698342e686b6bab1930a64aeefe52122fd1))
- **preprocessors:** implemented resize operation ([7aa016e](https://github.com/SalOne22/rimage/commit/7aa016ea0d2d7e691bfb34754df2a6153225dbeb))

### Bug Fixes

- added binary feature ([ee148f1](https://github.com/SalOne22/rimage/commit/ee148f1e4a0d6d0d106290c057fe29117b890258))
- added more image formats ([15aebe6](https://github.com/SalOne22/rimage/commit/15aebe609f744421429a0985847c7f4795c1e1b1))
- **bin:** moved binary to root folder ([f637d14](https://github.com/SalOne22/rimage/commit/f637d1428adcc622c08631c1a858f25a579757bc))
- **cargo:** updated cargo.toml ([4fba401](https://github.com/SalOne22/rimage/commit/4fba401d1d0ea61001459945e845acd305bb6581))
- **cli/windows:** fixed trailing slash cmd issue ([88e6b92](https://github.com/SalOne22/rimage/commit/88e6b92a88eb0a8ba86db5153e90582d5942a819))
- **cli:** fixed clippy issue ([8b5d230](https://github.com/SalOne22/rimage/commit/8b5d230c0913ca0b2f123ef65dc0be25aaec8b75))
- **cli:** updated features compilation ([0a70264](https://github.com/SalOne22/rimage/commit/0a7026492ddbbcc11b57f8d64c861d70600365e9))
- **cli:** updated preprocessors using traits ([b8be3ff](https://github.com/SalOne22/rimage/commit/b8be3ff5445ee99cda5b9d5831a0c9e82bd257b3))
- **codecs:** updated qtable type ([5a2c2f9](https://github.com/SalOne22/rimage/commit/5a2c2f98f0afc1935b174f5967642babd2632f8e))
- **deps:** changed dav1d dependence to lib aom ([032b050](https://github.com/SalOne22/rimage/commit/032b050c05aa64195e66a54181677aa1555b19c7))
- **deps:** optimized dependencies ([370f5d3](https://github.com/SalOne22/rimage/commit/370f5d3dc2696f7e02b59c54ef777bae930bbf32))
- fixed clippy errors ([2ca79be](https://github.com/SalOne22/rimage/commit/2ca79beea7f6928aa5512074b2f69fb25ff2ae2e))
- fixed clippy errors ([b2b7f79](https://github.com/SalOne22/rimage/commit/b2b7f79636c7febe58f59ecf5c0e549d4d5d35e8))
- fixed clippy warnings ([d98091a](https://github.com/SalOne22/rimage/commit/d98091ab83158a4424df90a10011d744b689d97e))
- fixed tests for image_format ([454f5c8](https://github.com/SalOne22/rimage/commit/454f5c800f303d01cafaac1b236df2ec465690e9))
- **lib:** cleared lib ([5d7b2c6](https://github.com/SalOne22/rimage/commit/5d7b2c6d840c83a23e0392f9b91937a9d2c7d11f))
- made options values public ([7d3fa3b](https://github.com/SalOne22/rimage/commit/7d3fa3bd105342a0eb03251546d30d59da842583))
- moved cli modules to separate directory ([81593d0](https://github.com/SalOne22/rimage/commit/81593d0ca971e10a8ce917e84930ffd7afc8e615))
- moved encoders to separate modules ([7c6816f](https://github.com/SalOne22/rimage/commit/7c6816f1bee2aff3f08fb765349f541ea182fa87))

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

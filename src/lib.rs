/*!
This crate provides a cli tool and library for image processing.
Similar to [squoosh!](https://squoosh.app/) using same codecs,
but fully written on rust and with bulk processing support.

Current features:
- Decoding jpeg and png
- Encoding with optimizations
- Get image information

# Usage

First add this crate to your dependencies:
```text
cargo add rimage
```

or add this to Cargo.toml:
```toml
[dependencies]
rimage = "0.2"
```

After that you can use this crate:

## Decoding
```
use rimage::Decoder;

// Create decoder from file path and data
let path = std::path::PathBuf::from("tests/files/basi0g01.jpg"); // Or any other image
let data = std::fs::read(&path).unwrap();
let decoder = Decoder::new(&path, &data);

// Decode image to image data
let image = match decoder.decode() {
    Ok(img) => img,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Get image data
println!("Size: {:?}", image.size());
println!("Data length: {:?}", image.data().len());

// Do something with image...
```

## Encoding

```
# use rimage::Decoder;
use rimage::{Config, Encoder, OutputFormat};
# let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
# let data = std::fs::read(&path).unwrap();
# let decoder = Decoder::new(&path, &data);
# let image = decoder.decode().unwrap();

// Build config for encoding
let config = match Config::build(
    75.0,
    OutputFormat::MozJpeg,
) {
    Ok(config) => config,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

let encoder = Encoder::new(&config, image); // where image is image::ImageData

// Get encoded image data from encoder
let data = match encoder.encode() {
    Ok(data) => data,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Write image to file
std::fs::write("output.jpg", data);
# std::fs::remove_file("output.jpg").unwrap();
```
*/
#![warn(missing_docs)]

use error::{ConfigError, DecodingError, EncodingError};
use rgb::{
    alt::{GRAY8, GRAYA8},
    AsPixels, ComponentBytes, FromSlice, RGB8, RGBA8,
};
use simple_error::SimpleError;
use std::{panic, path};

pub use image::{ImageData, OutputFormat};

/// Decoders for images
#[deprecated(since = "0.2.0", note = "use the Decoder struct instead")]
pub mod decoders;
/// Encoders for images
#[deprecated(since = "0.2.0", note = "use the Encoder struct instead")]
pub mod encoders;

/// Errors that can occur during image processing
pub mod error;

/// Image data structs
pub mod image;

/// Config for image encoding
///
/// # Example
/// ```
/// use rimage::{Config, OutputFormat};
///
/// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
/// ```
///
/// # Default
/// ```
/// use rimage::{Config, OutputFormat};
///
/// let config = Config::default();
/// assert_eq!(config.quality(), 75.0);
/// assert_eq!(config.output_format(), &OutputFormat::MozJpeg);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    quality: f32,
    output_format: OutputFormat,
}

impl Config {
    /// Create new config
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If quality is not in range 0.0..=100.0
    ///
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(200.0, OutputFormat::MozJpeg);
    /// assert!(config.is_err());
    /// ```
    pub fn build(quality: f32, output_format: OutputFormat) -> Result<Self, ConfigError> {
        if !(0.0..=100.0).contains(&quality) {
            return Err(ConfigError::QualityOutOfBounds);
        }

        Ok(Config {
            quality,
            output_format,
        })
    }
    /// Get quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert_eq!(config.quality(), 75.0);
    /// ```
    #[inline]
    pub fn quality(&self) -> f32 {
        self.quality
    }
    /// Get output format
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert_eq!(config.output_format(), &OutputFormat::MozJpeg);
    /// ```
    #[inline]
    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            quality: 75.0,
            output_format: OutputFormat::MozJpeg,
        }
    }
}

/// Decoder for images
///
/// # Example
/// ```
/// # use rimage::Decoder;
/// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
/// let data = std::fs::read(&path).unwrap();
///
/// let decoder = Decoder::new(&path, &data);
///
/// // Decode image to image data
/// let image = match decoder.decode() {
///     Ok(img) => img,
///     Err(e) => {
///         eprintln!("Oh no there is error! {e}");
///         std::process::exit(1);
///     }
/// };
/// ```
pub struct Decoder<'a> {
    path: &'a path::Path,
    raw_data: &'a [u8],
}

impl<'a> Decoder<'a> {
    /// Create new decoder
    ///
    /// # Example
    /// ```
    /// # use rimage::Decoder;
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// let data = std::fs::read(&path).unwrap();
    ///
    /// let decoder = Decoder::new(&path, &data);
    /// ```
    #[inline]
    pub fn new(path: &'a path::Path, raw_data: &'a [u8]) -> Self {
        Decoder { path, raw_data }
    }

    /// Decode image
    ///
    /// # Example
    /// ```
    /// # use rimage::Decoder;
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let data = std::fs::read(&path).unwrap();
    /// # let decoder = Decoder::new(&path, &data);
    /// // Decode image to image data
    /// let image = match decoder.decode() {
    ///     Ok(img) => img,
    ///     Err(e) => {
    ///         eprintln!("Oh no there is error! {e}");
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// // Do something with image data...
    /// ```
    ///
    /// # Errors
    ///
    /// If image format is not supported
    ///
    /// ```
    /// # use rimage::Decoder;
    /// let path = std::path::PathBuf::from("tests/files/test.bmp");
    /// let data = std::fs::read(&path).unwrap();
    /// let decoder = Decoder::new(&path, &data);
    ///
    /// let result = decoder.decode();
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().to_string(), "Format Error: bmp not supported");
    /// ```
    ///
    /// If image format is supported but there is error during decoding
    ///
    /// ```
    /// # use rimage::Decoder;
    /// let path = std::path::PathBuf::from("tests/files/test_corrupted.jpg");
    /// let data = std::fs::read(&path).unwrap();
    /// let decoder = Decoder::new(&path, &data);
    ///
    /// let result = decoder.decode();
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().to_string(), "Parsing Error: Failed to decode jpeg");
    /// ```
    pub fn decode(&self) -> Result<ImageData, DecodingError> {
        let extension = match self.path.extension() {
            Some(ext) => ext,
            None => {
                return Err(DecodingError::Format(Box::new(SimpleError::new(
                    "No extension",
                ))))
            }
        };

        match extension.to_str() {
            Some("jpg") | Some("jpeg") => self.decode_jpeg(),
            Some("png") => self.decode_png(),
            Some(ext) => Err(DecodingError::Format(Box::new(SimpleError::new(format!(
                "{} not supported",
                ext
            ))))),
            None => Err(DecodingError::Parsing(Box::new(SimpleError::new(
                "Failed to get extension",
            )))),
        }
    }

    fn decode_jpeg(&self) -> Result<ImageData, DecodingError> {
        panic::catch_unwind(|| -> Result<ImageData, DecodingError> {
            let d = mozjpeg::Decompress::new_mem(self.raw_data)?;
            let mut image = d.rgba()?;

            let data: Vec<RGBA8> =
                image
                    .read_scanlines()
                    .ok_or(DecodingError::Parsing(Box::new(SimpleError::new(
                        "Failed to read scanlines",
                    ))))?;

            Ok(ImageData::new(
                image.width(),
                image.height(),
                data.as_bytes().to_owned(),
            ))
        })
        .unwrap_or(Err(DecodingError::Parsing(Box::new(SimpleError::new(
            "Failed to decode jpeg",
        )))))
    }

    fn expand_pixels<T: Copy>(buf: &mut [u8], to_rgba: impl Fn(T) -> RGBA8)
    where
        [u8]: AsPixels<T> + FromSlice<u8>,
    {
        assert!(std::mem::size_of::<T>() <= std::mem::size_of::<RGBA8>());
        for i in (0..buf.len() / 4).rev() {
            let src_pix = buf.as_pixels()[i];
            buf.as_rgba_mut()[i] = to_rgba(src_pix);
        }
    }

    fn decode_png(&self) -> Result<ImageData, DecodingError> {
        let mut d = png::Decoder::new(self.raw_data);
        d.set_transformations(png::Transformations::normalize_to_color8());

        let mut reader = d.read_info()?;
        let width = reader.info().width;
        let height = reader.info().height;

        let buf_size = width as usize * height as usize * 4;

        let mut buf = vec![0; buf_size];

        let info = reader.next_frame(&mut buf)?;

        match info.color_type {
            png::ColorType::Grayscale => Self::expand_pixels(&mut buf, |gray: GRAY8| gray.into()),
            png::ColorType::GrayscaleAlpha => Self::expand_pixels(&mut buf, GRAYA8::into),
            png::ColorType::Rgb => Self::expand_pixels(&mut buf, RGB8::into),
            png::ColorType::Rgba => {}
            png::ColorType::Indexed => {
                return Err(DecodingError::Parsing(Box::new(SimpleError::new(
                    "Indexed color type is not supported",
                ))))
            }
        }

        Ok(ImageData::new(width as usize, height as usize, buf))
    }
}

/// Encoder for images
///
/// # Example
/// ```
/// # use rimage::{Encoder, Config, ImageData, OutputFormat};
/// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
/// # let data = std::fs::read(&path).unwrap();
/// # let decoder = rimage::Decoder::new(&path, &data);
/// # let image = decoder.decode().unwrap();
/// let config = Config::default();
///
/// // image is ImageData
/// let encoder = Encoder::new(&config, image);
/// let result = encoder.encode();
/// assert!(result.is_ok());
/// ```
pub struct Encoder<'a> {
    image_data: ImageData,
    config: &'a Config,
}

impl<'a> Encoder<'a> {
    /// Create new encoder
    ///
    /// # Example
    /// ```
    /// # use rimage::{Encoder, Config, ImageData, OutputFormat};
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let data = std::fs::read(&path).unwrap();
    /// # let decoder = rimage::Decoder::new(&path, &data);
    /// # let image = decoder.decode().unwrap();
    /// let config = Config::default();
    /// let encoder = Encoder::new(&config, image); // where image is ImageData
    /// ```
    pub fn new(conf: &'a Config, image_data: ImageData) -> Self {
        Encoder {
            image_data,
            config: conf,
        }
    }
    /// Encode image
    ///
    /// # Example
    /// ```
    /// # use rimage::{Encoder, Config, ImageData, OutputFormat};
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let data = std::fs::read(&path).unwrap();
    /// # let decoder = rimage::Decoder::new(&path, &data);
    /// # let image = decoder.decode().unwrap();
    /// let config = Config::default();
    /// let encoder = Encoder::new(&config, image); // where image is ImageData
    /// let result = encoder.encode();
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`EncodingError`] if encoding failed
    pub fn encode(self) -> Result<Vec<u8>, EncodingError> {
        match self.config.output_format {
            OutputFormat::Png => self.encode_png(),
            OutputFormat::Oxipng => self.encode_oxipng(),
            OutputFormat::MozJpeg => self.encode_mozjpeg(),
        }
    }

    fn encode_mozjpeg(self) -> Result<Vec<u8>, EncodingError> {
        panic::catch_unwind(|| -> Result<Vec<u8>, EncodingError> {
            let mut encoder = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGBA);

            encoder.set_size(self.image_data.size().0, self.image_data.size().1);
            encoder.set_quality(self.config.quality);
            encoder.set_progressive_mode();
            encoder.set_mem_dest();
            encoder.start_compress();
            encoder.write_scanlines(self.image_data.data());
            encoder.finish_compress();

            encoder.data_to_vec().map_err(|_| {
                EncodingError::Encoding(Box::new(SimpleError::new("Failed to convert data to vec")))
            })
        })
        .unwrap_or_else(|e| {
            Err(EncodingError::Encoding(Box::new(SimpleError::new(
                format!("Failed to encode image: {:?}", e),
            ))))
        })
    }

    fn encode_png(&self) -> Result<Vec<u8>, EncodingError> {
        let mut buf = Vec::new();

        {
            let mut encoder = png::Encoder::new(
                &mut buf,
                self.image_data.size().0 as u32,
                self.image_data.size().1 as u32,
            );

            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);

            let mut writer = encoder.write_header()?;
            writer.write_image_data(self.image_data.data())?;
            writer.finish()?;
        }

        Ok(buf)
    }

    fn encode_oxipng(&self) -> Result<Vec<u8>, EncodingError> {
        let mut buf = Vec::new();

        {
            let mut encoder = png::Encoder::new(
                &mut buf,
                self.image_data.size().0 as u32,
                self.image_data.size().1 as u32,
            );

            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);

            let mut writer = encoder.write_header()?;
            writer.write_image_data(self.image_data.data())?;
            writer.finish()?;
        }

        oxipng::optimize_from_memory(&buf, &oxipng::Options::default())
            .map_err(|e| EncodingError::Encoding(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use regex::Regex;

    use super::*;

    #[test]
    fn config_edge_cases() {
        let config = Config::default();
        assert_eq!(config.output_format, OutputFormat::MozJpeg);
        assert_eq!(config.quality, 75.0);
        let config = Config::build(100.0, OutputFormat::Png).unwrap();
        assert_eq!(config.output_format, OutputFormat::Png);
        assert_eq!(config.quality, 100.0);
        let config = Config::build(0.0, OutputFormat::Oxipng).unwrap();
        assert_eq!(config.output_format, OutputFormat::Oxipng);
        assert_eq!(config.quality, 0.0);
        let config_result = Config::build(101.0, OutputFormat::MozJpeg);
        assert!(config_result.is_err());
        let config_result = Config::build(-1.0, OutputFormat::MozJpeg);
        assert!(config_result.is_err());
    }

    #[test]
    fn decode_unsupported() {
        let path = path::Path::new("tests/files/test.bmp");

        fs::read(path)
            .map(|data| {
                let decoder = Decoder::new(path, &data);
                let result = decoder.decode();
                assert!(result.is_err());
            })
            .unwrap();
    }

    #[test]
    fn decode_grayscale() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x]&[^t].+0g\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_grayscale_alpha() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+4a\d\d\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_rgb() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^^tests/files/[^x]&[^t].+2c\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_rgb_transparent() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^^tests/files/[^x]&[t].+2c\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_rgba() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+6a\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_indexed() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x]&[^t].+3p\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_indexed_transparent() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x]&[t].+3p\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_corrupted() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/x.+\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let d = Decoder::new(path, &data);

            let img = d.decode();
            assert!(img.is_err());
        })
    }

    #[test]
    fn encode_jpeg() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            let conf = Config::build(75.0, OutputFormat::MozJpeg).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();

            assert!(result.is_ok());
            let result = result.unwrap();
            assert!(!result.is_empty());
        })
    }

    #[test]
    fn encode_png() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            let conf = Config::build(75.0, OutputFormat::Png).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();

            assert!(result.is_ok());
            let result = result.unwrap();
            assert!(!result.is_empty());
        })
    }

    #[test]
    fn encode_oxipng() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let data = fs::read(path).unwrap();
            let image = Decoder::new(path, &data).decode().unwrap();

            let conf = Config::build(75.0, OutputFormat::Oxipng).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();

            assert!(result.is_ok());
            let result = result.unwrap();
            assert!(!result.is_empty());
        })
    }
}

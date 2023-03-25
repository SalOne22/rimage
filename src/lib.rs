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

Next import Decoder:
```text
use rimage::Decoder;
```

After that you can use this crate:

## Decoding

```
# use rimage::Decoder;
# let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
// Build decoder from file path
let decoder = match Decoder::build(&path) {
    Ok(d) => d,
    Err(e) => {
        eprintln!("Oh no there is error! {e}");
        std::process::exit(1);
    }
};

// Decode image to image data
let image = match decoder.decode() {
    Ok(img) => img,
    Err(e) => {
        eprintln!("Oh no there is another error! {e}");
        std::process::exit(1);
    }
};

// Get image data
println!("Color Space: {:?}", image.color_space());
println!("Bit Depth: {:?}", image.bit_depth());
println!("Size: {:?}", image.size());
println!("Data length: {:?}", image.data().len());

// Do something with image...
```
*/
#![warn(missing_docs)]

use error::{ConfigError, DecodingError};
use std::{fs, panic, path};

pub use image::ImageData;
use image::{BitDepth, ColorSpace, OutputFormat};

/// Decoders for images
#[deprecated(since = "0.2.0", note = "use the Decoder struct instead")]
pub mod decoders;
/// Encoders for images
#[deprecated(since = "0.2.0", note = "use the Encoder struct instead")]
pub mod encoders;

/// All errors that can occur
pub mod error;

/// Image data
pub mod image;

/// Config for encoder
#[derive(Debug)]
pub struct Config<'a> {
    input: &'a [path::PathBuf],
    quality: f32,
    output_format: OutputFormat,
}

impl<'a> Config<'a> {
    /// Builds config from parameters
    ///
    /// # Examples
    ///
    /// ```
    /// # use rimage::{Config, OutputFormat};
    /// # use std::path;
    /// let input = &[path::PathBuf::from("tests/files/basi0g01.jpg")];
    /// let quality = 100.0;
    /// let output_format = OutputFormat::MozJpeg;
    /// let config = Config::build(input, quality, output_format).unwrap();
    /// ```
    ///
    /// # Errors
    /// - [`ConfigError::QualityOutOfBounds`] if quality is not in range 0.0 - 100.0
    /// - [`ConfigError::InputIsEmpty`] if input is empty
    pub fn build(
        input: &'a [path::PathBuf],
        quality: f32,
        output_format: OutputFormat,
    ) -> Result<Self, ConfigError> {
        if quality < 0.0 || quality > 100.0 {
            return Err(ConfigError::QualityOutOfBounds);
        }

        if input.is_empty() {
            return Err(ConfigError::InputIsEmpty);
        }

        Ok(Config {
            input,
            quality,
            output_format,
        })
    }
    #[inline]
    /// Gets input array of paths from config
    pub fn input(&self) -> &[path::PathBuf] {
        &self.input
    }
    #[inline]
    /// Gets quality of output images from config
    pub fn quality(&self) -> f32 {
        self.quality
    }
    #[inline]
    /// Gets format of output images from config
    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
}

impl<'a> Default for Config<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            input: &[],
            quality: 75.0,
            output_format: OutputFormat::MozJpeg,
        }
    }
}

/// Decoder used to get image data from file
pub struct Decoder<'a> {
    path: &'a path::PathBuf,
    raw_data: Vec<u8>,
}

impl<'a> Decoder<'a> {
    /// Builds decoder from path
    ///
    /// # Examples
    /// ```
    /// # use rimage::Decoder;
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// let d = Decoder::build(&path);
    /// ```
    ///
    /// # Errors
    ///
    /// - [`io::Error`] if file failed to read, find, etc.
    ///
    /// [`Decoder`]: struct.Decoder.html
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    #[inline]
    pub fn build(path: &'a path::PathBuf) -> Result<Self, DecodingError> {
        let raw_data = fs::read(path)?;

        Ok(Decoder { path, raw_data })
    }

    /// Decodes file to ImageData
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
    /// let image = d.decode();
    /// match image {
    ///   Ok(_) => println!("Image decoded"),
    ///  Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`DecodingError::Format`] if format is not supported
    /// - [`DecodingError::Parsing`] if file is not a image, unsupported color space, etc.
    ///
    /// [`DecodingError::Format`]: enum.DecodingError.html#variant.Format
    /// [`DecodingError::Parsing`]: enum.DecodingError.html#variant.Parsing
    pub fn decode(&self) -> Result<ImageData, DecodingError> {
        let extension = match self.path.extension() {
            Some(ext) => ext,
            None => return Err(DecodingError::Format("None".to_string())),
        };

        match extension.to_str() {
            Some("jpg") | Some("jpeg") => self.decode_jpeg(),
            Some("png") => self.decode_png(),
            Some(ext) => Err(DecodingError::Format(ext.to_string())),
            None => Err(DecodingError::Parsing(
                "Extension is not valid unicode".to_string(),
            )),
        }
    }

    fn decode_jpeg(&self) -> Result<ImageData, DecodingError> {
        panic::catch_unwind(|| -> Result<ImageData, DecodingError> {
            let d = mozjpeg::Decompress::new_mem(&self.raw_data)?;
            let color_space = match d.color_space() {
                mozjpeg::ColorSpace::JCS_GRAYSCALE => ColorSpace::Gray,
                mozjpeg::ColorSpace::JCS_CMYK => ColorSpace::Cmyk,
                mozjpeg::ColorSpace::JCS_RGB => ColorSpace::Rgb,
                _ => ColorSpace::Rgb,
            };
            let mut image = match d.image()? {
                mozjpeg::Format::RGB(img) => img,
                mozjpeg::Format::Gray(img) => img,
                mozjpeg::Format::CMYK(img) => img,
            };

            let data = image.read_scanlines_flat().ok_or(DecodingError::Parsing(
                "Cannot read jpeg scanlines".to_string(),
            ))?;

            Ok(ImageData::new(
                color_space,
                BitDepth::Eight,
                image.width() as usize,
                image.height() as usize,
                data,
            ))
        })
        .unwrap_or(Err(DecodingError::Parsing(
            "Failed to decode jpeg".to_string(),
        )))
    }

    fn decode_png(&self) -> Result<ImageData, DecodingError> {
        let d = png::Decoder::new(fs::File::open(self.path)?);
        let mut reader = d.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let data = buf[..info.buffer_size()].into();
        Ok(ImageData::new(
            info.color_type.into(),
            info.bit_depth.into(),
            info.width as usize,
            info.height as usize,
            data,
        ))
    }
}

/// Encoder used to encode image data to file
pub struct Encoder<'a> {
    path: &'a path::PathBuf,
    image_data: ImageData,
    quality: f32,
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn decode_grayscale() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+0g\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            println!("{path:?}");

            assert_eq!(image.color_space(), &ColorSpace::Gray);
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
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorSpace::GrayAlpha);
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
                let re = Regex::new(r"^^tests/files/[^x].+2c\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorSpace::Rgb);
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
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorSpace::Rgba);
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
                let re = Regex::new(r"^tests/files/[^x].+3p\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorSpace::Indexed);
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
            let d = Decoder::build(path);
            assert!(d.is_ok());

            let img = d.unwrap().decode();
            assert!(img.is_err());
        })
    }
}

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

use std::{fmt, fs, io, panic, path};

use clap::Parser;

/// Decoders for images
#[deprecated(since = "0.2.0", note = "use the Decoder struct instead")]
pub mod decoders;
/// Encoders for images
#[deprecated(since = "0.2.0", note = "use the Encoder struct instead")]
pub mod encoders;

/// Config from command line input
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Input files to be compressed
    pub input: Vec<std::path::PathBuf>,

    /// Quality of output images from 0 to 1
    #[arg(short, long, default_value_t = 0.75)]
    pub quality: f32,

    /// Format of output images
    #[arg(short, long, default_value_t = String::from("jpg"))]
    pub output_format: String,

    /// Outputs info about images
    #[arg(short, long, default_value_t = false)]
    pub info: bool,
}

/// Image data from decoder
///
/// # Examples
///
/// ```
/// # use rimage::{Decoder, DecodingError};
/// # use std::path;
/// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
/// let d = Decoder::build(&path)?;
///
/// let image = d.decode()?;
///
/// // Get something from image data
/// println!("Color Space: {:?}", image.color_space());
/// println!("Bit Depth: {:?}", image.bit_depth());
/// println!("Size: {:?}", image.size());
/// println!("Data length: {:?}", image.data().len());
/// # Ok::<(), DecodingError>(())
/// ```
pub struct ImageData {
    color_space: ColorSpace,
    bit_depth: BitDepth,
    width: usize,
    height: usize,
    data: Vec<u8>,
}

/// Decoder used to get image data from file
pub struct Decoder<'a> {
    path: &'a path::PathBuf,
    raw_data: Vec<u8>,
}

/// An error that occurred during decoding a image
#[derive(Debug)]
pub enum DecodingError {
    /// A [`io::Error`] if file failed to read, find, etc.
    IoError(io::Error),
    /// The format of file is not supported
    Format(String),
    /// A configuration error, see [`ConfigError`]
    Config(ConfigError),
    /// A decoding error, file is not a image, unsupported color space, etc.
    Parsing(String),
}

/// An error that occurred if configuration is invalid
#[derive(Debug)]
#[non_exhaustive]
pub enum ConfigError {
    /// Quality is less than 0 or greater than 1
    QualityOutOfBounds,
    /// Width is 0
    WidthIsZero,
    /// Height is 0
    HeightIsZero,
    /// Size is 0
    SizeIsZero,
    /// Output format is not supported
    FormatNotSupported(String),
}

/// Color space of image
#[derive(Debug, PartialEq, Eq)]
pub enum ColorSpace {
    /// **R**ed/**G**reen/**B**lue
    Rgb,
    /// **R**ed/**G**reen/**B**lue/**A**lpha
    Rgba,
    /// **C**yan/**M**agenta/**Y**ellow/Blac**K**
    Cmyk,
    /// Indexed color palette
    Indexed,
    /// Grayscale
    Grayscale,
    /// Grayscale/Alpha
    GrayscaleAlpha,
}

/// Bit depth of image per pixel
#[derive(Debug)]
#[repr(u8)]
pub enum BitDepth {
    /// One bit per pixel
    One = 1,
    /// Two bits per pixel
    Two = 2,
    /// Four bits per pixel
    Four = 4,
    /// Eight bits per pixel
    Eight = 8,
    /// Sixteen bits per pixel
    Sixteen = 16,
}

impl<'a> Decoder<'a> {
    /// Builds decoder from path
    ///
    /// # Result
    ///
    /// - [`Decoder`] if Ok
    /// - [`DecodingError`] if
    ///   - File not found or other `io::Error`
    ///   - Config is invalid
    pub fn build(path: &'a path::PathBuf) -> Result<Self, DecodingError> {
        let raw_data = fs::read(path)?;

        Ok(Decoder { path, raw_data })
    }

    /// Decodes file to ImageData
    ///
    /// # Result
    ///
    /// - [`ImageData`] if Ok
    /// - [`DecodingError`] if
    ///   - File is not a image
    ///   - File extension not supported
    ///   - File corrupted or in unknown color space
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, DecodingError};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// let d = Decoder::build(&path)?;
    /// let img = d.decode()?;
    /// # Ok::<(), DecodingError>(())
    /// ```
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
                mozjpeg::ColorSpace::JCS_GRAYSCALE => ColorSpace::Grayscale,
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

            Ok(ImageData {
                color_space,
                bit_depth: BitDepth::Eight,
                width: image.width(),
                height: image.height(),
                data,
            })
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
        let data = buf[..info.buffer_size()].to_vec();
        Ok(ImageData {
            color_space: info.color_type.into(),
            bit_depth: info.bit_depth.into(),
            width: info.width as usize,
            height: info.height as usize,
            data,
        })
    }
}

impl ImageData {
    /// Returns size of image (Width, Height)
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    /// Returns a ref to color space of image [`ColorSpace`]
    pub fn color_space(&self) -> &ColorSpace {
        &self.color_space
    }
    /// Returns a ref to array of bytes in image
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    /// Returns a ref to bit depth of image [`BitDepth`]
    pub fn bit_depth(&self) -> &BitDepth {
        &self.bit_depth
    }
}

impl From<io::Error> for DecodingError {
    fn from(err: io::Error) -> Self {
        DecodingError::IoError(err)
    }
}

impl From<png::DecodingError> for DecodingError {
    fn from(err: png::DecodingError) -> Self {
        match err {
            png::DecodingError::IoError(io_err) => DecodingError::IoError(io_err),
            png::DecodingError::Format(f_err) => DecodingError::Format(f_err.to_string()),
            png::DecodingError::Parameter(p_err) => DecodingError::Parsing(p_err.to_string()),
            png::DecodingError::LimitsExceeded => {
                DecodingError::Parsing("Png limits exceeded".to_string())
            }
        }
    }
}

impl From<png::ColorType> for ColorSpace {
    fn from(color_type: png::ColorType) -> Self {
        match color_type {
            png::ColorType::Grayscale => ColorSpace::Grayscale,
            png::ColorType::Rgb => ColorSpace::Rgb,
            png::ColorType::Indexed => ColorSpace::Indexed,
            png::ColorType::GrayscaleAlpha => ColorSpace::GrayscaleAlpha,
            png::ColorType::Rgba => ColorSpace::Rgba,
        }
    }
}

impl From<png::BitDepth> for BitDepth {
    fn from(bit_depth: png::BitDepth) -> Self {
        match bit_depth {
            png::BitDepth::One => BitDepth::One,
            png::BitDepth::Two => BitDepth::Two,
            png::BitDepth::Four => BitDepth::Four,
            png::BitDepth::Eight => BitDepth::Eight,
            png::BitDepth::Sixteen => BitDepth::Sixteen,
        }
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodingError::IoError(io_err) => f.write_fmt(format_args!("{}", io_err)),
            DecodingError::Format(fmt_err) => f.write_str(fmt_err),
            DecodingError::Config(cfg_err) => f.write_fmt(format_args!("{}", cfg_err)),
            DecodingError::Parsing(prs_err) => f.write_str(prs_err),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::QualityOutOfBounds => f.write_str("Quality is out of bounds"),
            ConfigError::WidthIsZero => f.write_str("Width is cannot be zero"),
            ConfigError::HeightIsZero => f.write_str("Height is cannot be zero"),
            ConfigError::SizeIsZero => f.write_str("Size is cannot be zero"),
            ConfigError::FormatNotSupported(ext) => {
                f.write_fmt(format_args!("{} is not supported", ext))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn display_decoder_error() {
        assert_eq!(
            DecodingError::IoError(io::Error::new(io::ErrorKind::NotFound, "path not found"))
                .to_string(),
            "path not found"
        );
        assert_eq!(
            DecodingError::Format("WebP not supported".to_string()).to_string(),
            "WebP not supported"
        );
        assert_eq!(
            DecodingError::Config(ConfigError::QualityOutOfBounds).to_string(),
            "Quality is out of bounds"
        )
    }

    #[test]
    fn display_config_error() {
        assert_eq!(
            ConfigError::QualityOutOfBounds.to_string(),
            "Quality is out of bounds"
        );
        assert_eq!(
            ConfigError::WidthIsZero.to_string(),
            "Width is cannot be zero"
        );
        assert_eq!(
            ConfigError::HeightIsZero.to_string(),
            "Height is cannot be zero"
        );
        assert_eq!(
            ConfigError::SizeIsZero.to_string(),
            "Size is cannot be zero"
        );
        assert_eq!(
            ConfigError::FormatNotSupported("webp".to_string()).to_string(),
            "webp is not supported"
        )
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
                let re = Regex::new(r"^tests/files/[^x].+0g\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            println!("{path:?}");

            assert_eq!(image.color_space(), &ColorSpace::Grayscale);
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

            assert_eq!(image.color_space(), &ColorSpace::GrayscaleAlpha);
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

//! # Rimage
//! `rimage` is CLI tool that compress multiple images at once.
//! Also it provides lib crate with functions to decode and encode images

use std::{
    fmt, fs, io, panic,
    path::{self, PathBuf},
};

use clap::Parser;

/// Decoders for images
pub mod decoders;
/// Encoders for images
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
}

pub struct ImageData {
    color_space: ColorScheme,
    width: usize,
    height: usize,
    data: Vec<u8>,
}

pub struct Decoder<'a> {
    path: &'a path::PathBuf,
    raw_data: Vec<u8>,
}

#[derive(Debug)]
pub enum DecodingError {
    IoError(io::Error),
    Format(String),
    Config(ConfigError),
    Parsing(String),
}

#[derive(Debug)]
pub enum ConfigError {
    QualityOutOfBounds,
    WidthIsZero,
    HeightIsZero,
    SizeIsZero,
    FormatNotSupported(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ColorScheme {
    RGB,
    RGBA,
    Indexed,
    Grayscale,
    GrayscaleAlpha,
}

impl<'a> Decoder<'a> {
    pub fn build(path: &'a PathBuf) -> Result<Self, DecodingError> {
        let raw_data = fs::read(path)?;
        Ok(Decoder { path, raw_data })
    }

    pub fn decode(&self) -> Result<ImageData, DecodingError> {
        let extension = match self.path.extension() {
            Some(ext) => ext,
            None => return Err(DecodingError::Format("None".to_string())),
        };

        match extension.to_str() {
            Some("jpg") | Some("jpeg") => self.decode_jpeg(),
            Some("png") => self.decode_png(),
            Some(ext) => return Err(DecodingError::Format(ext.to_string())),
            None => {
                return Err(DecodingError::Parsing(
                    "Extension is not valid unicode".to_string(),
                ))
            }
        }
    }

    fn decode_jpeg(&self) -> Result<ImageData, DecodingError> {
        panic::catch_unwind(|| -> Result<ImageData, DecodingError> {
            let d = mozjpeg::Decompress::new_mem(&self.raw_data)?;
            let color_space = match d.color_space() {
                mozjpeg::ColorSpace::JCS_GRAYSCALE => ColorScheme::Grayscale,
                mozjpeg::ColorSpace::JCS_EXT_RGBA => ColorScheme::RGBA,
                mozjpeg::ColorSpace::JCS_EXT_BGRA => ColorScheme::RGBA,
                mozjpeg::ColorSpace::JCS_EXT_ABGR => ColorScheme::RGBA,
                mozjpeg::ColorSpace::JCS_EXT_ARGB => ColorScheme::RGBA,
                _ => ColorScheme::RGB,
            };
            let mut image = match d.color_space() {
                mozjpeg::ColorSpace::JCS_UNKNOWN => {
                    return Err(DecodingError::Parsing("Unknown color space".to_string()))
                }
                mozjpeg::ColorSpace::JCS_GRAYSCALE => d.grayscale(),
                mozjpeg::ColorSpace::JCS_EXT_RGBA => d.rgba(),
                mozjpeg::ColorSpace::JCS_EXT_BGRA => d.rgba(),
                mozjpeg::ColorSpace::JCS_EXT_ABGR => d.rgba(),
                mozjpeg::ColorSpace::JCS_EXT_ARGB => d.rgba(),
                _ => d.rgb(),
            }?;

            let width = image.width();
            let height = image.height();

            Ok(ImageData {
                color_space,
                width,
                height,
                data: image.read_scanlines_flat().unwrap(),
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
        let color_space = match info.color_type {
            png::ColorType::Grayscale => ColorScheme::Grayscale,
            png::ColorType::Rgb => ColorScheme::RGB,
            png::ColorType::Indexed => ColorScheme::Indexed,
            png::ColorType::GrayscaleAlpha => ColorScheme::GrayscaleAlpha,
            png::ColorType::Rgba => ColorScheme::RGBA,
        };
        Ok(ImageData {
            color_space,
            width: info.width as usize,
            height: info.height as usize,
            data,
        })
    }
}

impl ImageData {
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    pub fn color_space(&self) -> &ColorScheme {
        &self.color_space
    }
    pub fn data_len(&self) -> usize {
        self.data.len()
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
                let re = Regex::new(r"^tests/files/[^x].+0g\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorScheme::Grayscale);
            assert_ne!(image.data_len(), 0);
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
                let re = Regex::new(r"^tests/files/[^x].+4a\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorScheme::GrayscaleAlpha);
            assert_ne!(image.data_len(), 0);
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
                let re = Regex::new(r"^tests/files/[^x].+2c\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorScheme::RGB);
            assert_ne!(image.data_len(), 0);
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

            assert_eq!(image.color_space(), &ColorScheme::RGBA);
            assert_ne!(image.data_len(), 0);
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

            assert_eq!(image.color_space(), &ColorScheme::Indexed);
            assert_ne!(image.data_len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    #[should_panic]
    fn decode_corrupted() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/x.+0g\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorScheme::Grayscale);
            assert_ne!(image.data_len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }
}

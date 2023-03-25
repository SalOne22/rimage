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

use std::{borrow::Cow, fmt, fs, io, panic, path};

/// Decoders for images
#[deprecated(since = "0.2.0", note = "use the Decoder struct instead")]
pub mod decoders;
/// Encoders for images
#[deprecated(since = "0.2.0", note = "use the Encoder struct instead")]
pub mod encoders;

/// Config from command line input
#[derive(Debug)]
pub struct Config<'a> {
    input: &'a [path::PathBuf],
    quality: f32,
    output_format: OutputFormat,
}

impl<'a> Config<'a> {
    /// Builds config from parameters
    ///
    /// # Result
    ///
    /// - [`Config`] if Ok
    /// - [`ConfigError`] if
    ///   - Quality under 0 or greater than 1
    ///   - input is empty
    pub fn build(
        input: &'a [path::PathBuf],
        quality: f32,
        output_format: OutputFormat,
    ) -> Result<Self, ConfigError> {
        if quality < 0.0 || quality > 1.0 {
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
    /// Gets input array of paths from config
    pub fn input(&self) -> &[path::PathBuf] {
        &self.input
    }
    /// Gets quality of output images from config
    pub fn quality(&self) -> f32 {
        self.quality
    }
    /// Gets format of output images from config
    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            input: &[],
            quality: 0.75,
            output_format: OutputFormat::MozJpeg,
        }
    }
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
pub struct ImageData<'a> {
    color_space: ColorSpace,
    bit_depth: BitDepth,
    width: usize,
    height: usize,
    data: Cow<'a, [u8]>,
}

/// Decoder used to get image data from file
pub struct Decoder<'a> {
    path: &'a path::PathBuf,
    raw_data: Vec<u8>,
}

// Write encoder struct
/// Encoder used to encode image data to file
pub struct Encoder<'a> {
    path: &'a path::PathBuf,
    image_data: ImageData<'a>,
    quality: f32,
}

/// Supported output format
///
/// # Examples
/// ```
/// # use rimage::OutputFormat;
/// # use std::str::FromStr;
/// let format = OutputFormat::from_str("mozjpeg").unwrap();
/// println!("Format: {}", format);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// MozJpeg image
    MozJpeg,
    /// Browser Png image
    Png,
    /// OxiPng image
    Oxipng,
}

impl std::str::FromStr for OutputFormat {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mozjpeg" | "jpg" | "jpeg" => Ok(OutputFormat::MozJpeg),
            "png" => Ok(OutputFormat::Png),
            "oxipng" => Ok(OutputFormat::Oxipng),
            _ => Err(format!("{} is not a valid output format", s).into()),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::MozJpeg => write!(f, "jpeg"),
            OutputFormat::Png => write!(f, "png"),
            OutputFormat::Oxipng => write!(f, "oxipng"),
        }
    }
}

/// An error that occurred during decoding a image
///
/// # Examples
/// ```
/// # use rimage::{Decoder, DecodingError};
/// # use std::path;
/// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
/// let d = Decoder::build(&path)?;
/// let image = d.decode();
/// match image {
///     Ok(_) => println!("Image decoded"),
///     Err(e) => println!("Error: {}", e),
/// }
/// # Ok::<(), DecodingError>(())
/// ```
#[derive(Debug)]
pub enum DecodingError {
    /// A [`io::Error`] if file failed to read, find, etc.
    IoError(io::Error),
    /// The format of file is not supported
    Format(String),
    /// A decoding error, file is not a image, unsupported color space, etc.
    Parsing(String),
}

/// An error that occurred if configuration is invalid
///
/// # Examples
/// ```
/// # use rimage::{Config, ConfigError};
/// let config = Config::build(&[], 1.1, rimage::OutputFormat::MozJpeg);
/// match config {
///    Ok(_) => println!("Config is valid"),
///  Err(e) => println!("Error: {}", e),
/// }
/// ```
///
/// # Errors
///
/// - [`ConfigError::QualityOutOfBounds`] if quality is less than 0 or greater than 1
/// - [`ConfigError::InputIsEmpty`] if input is empty
/// - [`ConfigError::WidthIsZero`] if width is 0
/// - [`ConfigError::HeightIsZero`] if height is 0
/// - [`ConfigError::SizeIsZero`] if size is 0
///
/// [`ConfigError::QualityOutOfBounds`]: enum.ConfigError.html#variant.QualityOutOfBounds
/// [`ConfigError::InputIsEmpty`]: enum.ConfigError.html#variant.InputIsEmpty
/// [`ConfigError::WidthIsZero`]: enum.ConfigError.html#variant.WidthIsZero
/// [`ConfigError::HeightIsZero`]: enum.ConfigError.html#variant.HeightIsZero
/// [`ConfigError::SizeIsZero`]: enum.ConfigError.html#variant.SizeIsZero
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
    /// Input is empty
    InputIsEmpty,
}

/// Color space of image
///
/// # Examples
/// ```
/// # use rimage::ColorSpace;
/// # use std::str::FromStr;
/// let color_space = ColorSpace::from_str("rgb").unwrap();
/// println!("Color Space: {}", color_space);
/// ```
///
/// # Errors
///
/// - [`ColorSpace::from_str`] if color space is not supported
///
/// [`ColorSpace::from_str`]: enum.ColorSpace.html#method.from_str
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

impl std::str::FromStr for ColorSpace {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rgb" => Ok(ColorSpace::Rgb),
            "rgba" => Ok(ColorSpace::Rgba),
            "cmyk" => Ok(ColorSpace::Cmyk),
            "indexed" => Ok(ColorSpace::Indexed),
            "grayscale" => Ok(ColorSpace::Grayscale),
            "grayscale_alpha" => Ok(ColorSpace::GrayscaleAlpha),
            _ => Err(format!("{} is not a valid color space", s).into()),
        }
    }
}

impl fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorSpace::Rgb => write!(f, "rgb"),
            ColorSpace::Rgba => write!(f, "rgba"),
            ColorSpace::Cmyk => write!(f, "cmyk"),
            ColorSpace::Indexed => write!(f, "indexed"),
            ColorSpace::Grayscale => write!(f, "grayscale"),
            ColorSpace::GrayscaleAlpha => write!(f, "grayscale_alpha"),
        }
    }
}

/// Bit depth of image per pixel
///
/// # Examples
/// ```
/// # use rimage::BitDepth;
/// # use std::str::FromStr;
/// let bit_depth = BitDepth::from_str("8").unwrap();
/// println!("Bit Depth: {}", bit_depth);
/// ```
///
/// # Errors
///
/// - [`BitDepth::from_str`] if bit depth is not supported
///
/// [`BitDepth::from_str`]: enum.BitDepth.html#method.from_str
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

impl std::str::FromStr for BitDepth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(BitDepth::One),
            "2" => Ok(BitDepth::Two),
            "4" => Ok(BitDepth::Four),
            "8" => Ok(BitDepth::Eight),
            "16" => Ok(BitDepth::Sixteen),
            _ => Err(format!("{} is not a valid bit depth", s)),
        }
    }
}

impl fmt::Display for BitDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitDepth::One => write!(f, "1"),
            BitDepth::Two => write!(f, "2"),
            BitDepth::Four => write!(f, "4"),
            BitDepth::Eight => write!(f, "8"),
            BitDepth::Sixteen => write!(f, "16"),
        }
    }
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

            let data = Cow::Borrowed(image
                .read_scanlines_flat()
                .ok_or(DecodingError::Parsing(
                    "Cannot read jpeg scanlines".to_string(),
                ))?
                .as_slice());

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
        let data = buf[..info.buffer_size()].into();
        Ok(ImageData {
            color_space: info.color_type.into(),
            bit_depth: info.bit_depth.into(),
            width: info.width as usize,
            height: info.height as usize,
            data,
        })
    }
}

impl<'a> ImageData<'a> {
    /// Returns size of image (Width, Height)
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
    /// # let image = d.decode().unwrap();
    /// let (width, height) = image.size();
    /// ```
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    /// Returns a ref to color space of image [`ColorSpace`]
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData, ColorSpace};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
    /// # let image = d.decode().unwrap();
    /// let color_space = image.color_space();
    /// match color_space {
    ///     ColorSpace::Grayscale => println!("Grayscale"),
    ///     ColorSpace::Rgb => println!("RGB"),
    ///     ColorSpace::Cmyk => println!("CMYK"),
    ///     ColorSpace::Rgba => println!("RGBA"),
    ///     ColorSpace::Indexed => println!("Indexed"),
    ///     ColorSpace::GrayscaleAlpha => println!("Grayscale Alpha"),
    /// }
    /// ```
    /// [`ColorSpace`]: enum.ColorSpace.html
    pub fn color_space(&self) -> &ColorSpace {
        &self.color_space
    }
    /// Returns a ref to array of bytes in image
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
    /// # let image = d.decode().unwrap();
    /// let data = image.data();
    /// ```
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    /// Returns a ref to bit depth of image [`BitDepth`]
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData, BitDepth};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
    /// # let image = d.decode().unwrap();
    /// let bit_depth = image.bit_depth();
    /// match bit_depth {
    ///     BitDepth::One => println!("1 bit"),
    ///     BitDepth::Two => println!("2 bits"),
    ///     BitDepth::Four => println!("4 bits"),
    ///     BitDepth::Eight => println!("8 bits"),
    ///     BitDepth::Sixteen => println!("16 bits"),
    /// }
    /// ```
    ///
    /// [`BitDepth`]: enum.BitDepth.html
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
            DecodingError::IoError(io_err) => write!(f, "IO Error: {}", io_err),
            DecodingError::Format(fmt_err) => write!(f, "Format Error: {}", fmt_err),
            DecodingError::Parsing(prs_err) => write!(f, "Parsing Error: {}", prs_err),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::QualityOutOfBounds => write!(f, "Quality is out of bounds"),
            ConfigError::WidthIsZero => write!(f, "Width cannot be zero"),
            ConfigError::HeightIsZero => write!(f, "Height cannot be zero"),
            ConfigError::SizeIsZero => write!(f, "Size cannot be zero"),
            ConfigError::InputIsEmpty => write!(f, "Input cannot be zero"),
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
            "IO Error: path not found"
        );
        assert_eq!(
            DecodingError::Format("WebP not supported".to_string()).to_string(),
            "Format Error: WebP not supported"
        );
    }

    #[test]
    fn display_config_error() {
        assert_eq!(
            ConfigError::QualityOutOfBounds.to_string(),
            "Quality is out of bounds"
        );
        assert_eq!(ConfigError::WidthIsZero.to_string(), "Width cannot be zero");
        assert_eq!(
            ConfigError::HeightIsZero.to_string(),
            "Height cannot be zero"
        );
        assert_eq!(ConfigError::SizeIsZero.to_string(), "Size cannot be zero");
        assert_eq!(
            ConfigError::InputIsEmpty.to_string(),
            "Input cannot be zero"
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

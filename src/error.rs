use std::{error::Error, fmt, io};

/// An error that occurred if configuration is invalid
///
/// # Examples
/// ```
/// # use rimage::{Config, error::ConfigError};
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

impl Error for ConfigError {}

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

/// An error that occurred during decoding a image
///
/// # Examples
/// ```
/// # use rimage::{Decoder, error::DecodingError};
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

impl Error for DecodingError {}

impl From<io::Error> for DecodingError {
    #[inline]
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
            DecodingError::IoError(io_err) => write!(f, "IO Error: {}", io_err),
            DecodingError::Format(fmt_err) => write!(f, "Format Error: {}", fmt_err),
            DecodingError::Parsing(prs_err) => write!(f, "Parsing Error: {}", prs_err),
        }
    }
}

/// An error that occurred during encoding a image
#[derive(Debug)]
pub enum EncodingError {
    /// A [`io::Error`] if file failed to write, find, etc.
    IoError(io::Error),
    /// The format of file is not supported
    Format(String),
    /// A encoding error, data is invalid, unsupported color space, etc.
    Encoding(String),
}

impl Error for EncodingError {}

impl From<io::Error> for EncodingError {
    #[inline]
    fn from(err: io::Error) -> Self {
        EncodingError::IoError(err)
    }
}

impl From<png::EncodingError> for EncodingError {
    fn from(err: png::EncodingError) -> Self {
        match err {
            png::EncodingError::IoError(io_err) => EncodingError::IoError(io_err),
            png::EncodingError::Format(f_err) => EncodingError::Format(f_err.to_string()),
            png::EncodingError::Parameter(p_err) => EncodingError::Encoding(p_err.to_string()),
            png::EncodingError::LimitsExceeded => {
                EncodingError::Encoding("Png limits exceeded".to_string())
            }
        }
    }
}

impl From<oxipng::PngError> for EncodingError {
    fn from(err: oxipng::PngError) -> Self {
        match err {
            oxipng::PngError::DeflatedDataTooLong(_) => {
                EncodingError::Encoding("Deflated data too long".to_string())
            }
            oxipng::PngError::TimedOut => EncodingError::Encoding("Timed out".to_string()),
            oxipng::PngError::NotPNG => EncodingError::Encoding("Not a PNG".to_string()),
            oxipng::PngError::APNGNotSupported => {
                EncodingError::Encoding("APNG not supported".to_string())
            }
            oxipng::PngError::InvalidData => EncodingError::Encoding("Invalid data".to_string()),
            oxipng::PngError::TruncatedData => {
                EncodingError::Encoding("Truncated data".to_string())
            }
            oxipng::PngError::ChunkMissing(_) => {
                EncodingError::Encoding("Chunk missing".to_string())
            }
            oxipng::PngError::Other(err) => EncodingError::Encoding(err.into_string()),
            _ => EncodingError::Encoding("Unknown error".to_string()),
        }
    }
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::IoError(io_err) => write!(f, "IO Error: {}", io_err),
            EncodingError::Format(fmt_err) => write!(f, "Format Error: {}", fmt_err),
            EncodingError::Encoding(enc_err) => write!(f, "Encoding Error: {}", enc_err),
        }
    }
}

#[cfg(test)]
mod tests {
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
}

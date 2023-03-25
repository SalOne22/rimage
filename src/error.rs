use std::{fmt, io};

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

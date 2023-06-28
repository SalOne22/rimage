use std::{error::Error, fmt, io};

/// An error that occurred if configuration is invalid
#[derive(Debug)]
pub enum ConfigError {
    /// Quality is less than 0 or greater than 100
    QualityOutOfBounds,
    /// Width is 0
    WidthIsZero,
    /// Height is 0
    HeightIsZero,
    /// Quantization quality is less than 0 or greater than 100
    QuantizationQualityOutOfBounds,
    /// Dithering level is less than 0 or greater than 1.0
    DitheringLevelOutOfBounds,
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::QualityOutOfBounds => write!(f, "Quality is out of bounds"),
            ConfigError::WidthIsZero => write!(f, "Width cannot be zero"),
            ConfigError::HeightIsZero => write!(f, "Height cannot be zero"),
            ConfigError::QuantizationQualityOutOfBounds => {
                write!(f, "Quantization quality is out of bounds")
            }
            ConfigError::DitheringLevelOutOfBounds => {
                write!(f, "Dithering level is out of bounds")
            }
        }
    }
}

/// An error that occurred during decoding a image
#[derive(Debug)]
pub enum DecodingError {
    /// A [`io::Error`] if file failed to read, find, etc.
    IoError(io::Error),
    /// The format of file is not supported
    Format(Box<dyn Error>),
    /// A decoding error, file is not a image, unsupported color space, etc.
    Parsing(Box<dyn Error>),
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
            _ => DecodingError::Parsing(Box::new(err)),
        }
    }
}

impl From<libwebp::error::WebPSimpleError> for DecodingError {
    #[inline]
    fn from(err: libwebp::error::WebPSimpleError) -> Self {
        DecodingError::Parsing(Box::new(err))
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
    Format(Box<dyn Error>),
    /// A encoding error, data is invalid, unsupported color space, etc.
    Encoding(Box<dyn Error>),
    /// A quantization error, if some error occurred during quantization
    Quantization(Box<dyn Error>),
    /// A resize error, if some error occurred during resizing
    Resize(Box<dyn Error>),
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
            _ => EncodingError::Encoding(Box::new(err)),
        }
    }
}

impl From<oxipng::PngError> for EncodingError {
    #[inline]
    fn from(err: oxipng::PngError) -> Self {
        EncodingError::Encoding(Box::new(err))
    }
}

impl From<imagequant::Error> for EncodingError {
    #[inline]
    fn from(err: imagequant::Error) -> Self {
        EncodingError::Encoding(Box::new(err))
    }
}

impl From<resize::Error> for EncodingError {
    #[inline]
    fn from(err: resize::Error) -> Self {
        EncodingError::Encoding(Box::new(err))
    }
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::IoError(io_err) => write!(f, "IO Error: {}", io_err),
            EncodingError::Format(fmt_err) => write!(f, "Format Error: {}", fmt_err),
            EncodingError::Encoding(enc_err) => write!(f, "Encoding Error: {}", enc_err),
            EncodingError::Quantization(qnt_err) => write!(f, "Quantization Error: {}", qnt_err),
            EncodingError::Resize(rsz_err) => write!(f, "Resize Error: {}", rsz_err),
        }
    }
}

#[cfg(test)]
mod tests;

use libwebp::error::WebPSimpleError;
use std::io;
use thiserror::Error;

/// An error that occurred if configuration is invalid
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Quality is less than 0 or greater than 100
    #[error("{0} is out of range from 0 to 100")]
    QualityOutOfBounds(f32),
    /// Width is 0
    #[error("Width cannot be zero")]
    WidthIsZero,
    /// Height is 0
    #[error("Height cannot be zero")]
    HeightIsZero,
    /// Quantization quality is less than 0 or greater than 100
    #[error("{0} is out of range from 0 to 100")]
    QuantizationQualityOutOfBounds(u8),
    /// Dithering level is less than 0 or greater than 1.0
    #[error("{0} is out of range from 0 to 1.0")]
    DitheringLevelOutOfBounds(f32),
}

/// An error that occurred during decoding a image
#[derive(Error, Debug)]
pub enum DecodingError {
    /// A [`io::Error`] if file failed to read, find, etc.
    #[error(transparent)]
    IO(#[from] io::Error),
    /// The format of file is not supported
    #[error("{0} is not supported")]
    Format(String),
    /// A parsing error, color type is not supported, failed to read extension etc.
    #[error("{0}")]
    Parsing(String),
    /// A Jpeg decoding error
    #[error("{0}")]
    Jpeg(String),
    /// A Png decoding error
    #[error(transparent)]
    Png(#[from] png::DecodingError),
    /// A Webp decoding error
    #[error(transparent)]
    Webp(#[from] WebPSimpleError),
    /// A Avif decoding error
    #[error("{0}")]
    Avif(String),
}

/// An error that occurred during encoding a image
#[derive(Error, Debug)]
pub enum EncodingError {
    /// A [`io::Error`] if file failed to write, find, etc.
    #[error(transparent)]
    IO(#[from] io::Error),
    /// A error that occurred during image resize
    #[error(transparent)]
    Resize(#[from] resize::Error),
    /// A error that occurred during image quantization
    #[error(transparent)]
    Quantization(#[from] imagequant::Error),
    /// A Jpeg encoding error
    #[error("{0}")]
    Jpeg(String),
    /// A Png encoding error
    #[error(transparent)]
    Png(#[from] png::EncodingError),
    /// A OxiPNG encoding error
    #[error(transparent)]
    OxiPng(#[from] oxipng::PngError),
    /// A Webp encoding error
    #[error(transparent)]
    Webp(#[from] WebPSimpleError),
    /// A Avif encoding error
    #[error(transparent)]
    Avif(#[from] ravif::Error),
}

#[cfg(test)]
mod tests;

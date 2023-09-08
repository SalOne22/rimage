use std::io;

use thiserror::Error;

/// Error type for invalid quantization configuration.
///
/// This error is returned when the input values for [`QuantizationConfig`] are out of the valid range.
#[derive(Error, Debug)]
pub enum InvalidQuantizationConfig {
    /// Error indicating that the quality value is out of bounds.
    #[error("Quality value {0} is out of bounds (0-100).")]
    QualityOutOfBounds(u8),

    /// Error indicating that the dithering level is out of bounds.
    #[error("Dithering level {0} is out of bounds (0.0-1.0).")]
    DitheringOutOfBounds(f32),
}

/// Error type for invalid encoder configuration.
///
/// This error is returned when the input values for [`EncoderConfig`] are out of the valid range.
#[derive(Error, Debug)]
pub enum InvalidEncoderConfig {
    /// Error indicating that the quality value is out of bounds.
    #[error("Quality value {0} is out of bounds (0.0-100.0).")]
    QualityOutOfBounds(f32),
}

/// Enum representing errors related to parsing image formats.
#[derive(Error, Debug)]
pub enum ImageFormatError {
    /// Error indicating an unknown or unsupported image format.
    #[error("Unknown image format: {0}")]
    Unknown(String),

    /// Error indicating that the image format is missing or could not be determined.
    #[error("Can't find image format")]
    Missing,
}

/// Enum representing various error types that can occur during image encoding.
#[derive(Error, Debug)]
pub enum EncoderError {
    /// Error indicating an I/O (input/output) operation failure.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Error indicating a resizing operation failure.
    #[error(transparent)]
    #[cfg(feature = "resizing")]
    Resize(#[from] resize::Error),

    /// Error indicating a quantization operation failure.
    #[error(transparent)]
    #[cfg(feature = "quantization")]
    Quantization(#[from] imagequant::Error),

    /// Error indicating an overflow or conversion error.
    #[error(transparent)]
    Overflow(#[from] std::num::TryFromIntError),

    /// Error indicating an encoding failure for the JPEG XL format.
    #[error(transparent)]
    JpegXl(#[from] jpegxl_rs::EncodeError),

    /// Error indicating an encoding failure for the PNG format.
    #[error(transparent)]
    Png(#[from] png::EncodingError),

    /// Error indicating an error during the encoding of PNG images with oxipng.
    #[error(transparent)]
    #[cfg(feature = "oxipng")]
    OxiPng(#[from] oxipng::PngError),

    /// Error indicating an error during the encoding of AVIF images with ravif.
    #[error(transparent)]
    Ravif(#[from] ravif::Error),

    /// General error indicating that something went wrong during image encoding.
    #[error("Something went wrong")]
    General,
}

/// Enum representing errors that can occur during image decoding.
#[derive(Error, Debug)]
pub enum DecoderError {
    /// An I/O error occurred during image decoding.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// An error occurred while parsing or detecting the image format.
    #[error(transparent)]
    Format(#[from] ImageFormatError),

    /// An error occurred when decoding AVIF image format with a specific exit code.
    #[error("Failed to decode AVIF with exit code: {0}")]
    Avif(u32),

    /// An error occurred when decoding JPEG image format with MozJPEG.
    #[error("Failed to decode JPEG with MozJPEG")]
    MozJpeg,

    /// An error occurred during JPEG XL image decoding.
    #[error(transparent)]
    JpegXl(#[from] jpegxl_rs::DecodeError),

    /// An error occurred during PNG image decoding.
    #[error(transparent)]
    Png(#[from] png::DecodingError),

    /// An error occurred when decoding WebP image format with libwebp.
    #[error("Failed to decode WebP with libwebp")]
    WebP,
}

#[cfg(test)]
mod tests {
    // Import the necessary dependencies from the code
    use super::*;

    #[test]
    fn invalid_quantization_config_errors() {
        // Test QualityOutOfBounds error
        let quality_error = InvalidQuantizationConfig::QualityOutOfBounds(120);
        assert_eq!(
            format!("{}", quality_error),
            "Quality value 120 is out of bounds (0-100)."
        );

        // Test DitheringOutOfBounds error
        let dithering_error = InvalidQuantizationConfig::DitheringOutOfBounds(1.5);
        assert_eq!(
            format!("{}", dithering_error),
            "Dithering level 1.5 is out of bounds (0.0-1.0)."
        );
    }

    #[test]
    fn invalid_encoder_config_errors() {
        // Test QualityOutOfBounds error
        let quality_error = InvalidEncoderConfig::QualityOutOfBounds(120.0);
        assert_eq!(
            format!("{}", quality_error),
            "Quality value 120 is out of bounds (0.0-100.0)."
        );
    }

    #[test]
    fn encoder_error_messages() {
        // Test Io error message
        let io_error = EncoderError::Io(std::io::Error::new(std::io::ErrorKind::Other, "IO error"));
        assert_eq!(format!("{}", io_error), "IO error");

        // Test Resize error message
        #[cfg(feature = "resizing")]
        let resize_error = EncoderError::Resize(resize::Error::OutOfMemory);
        #[cfg(feature = "resizing")]
        assert_eq!(
            format!("{}", resize_error),
            format!("{}", resize::Error::OutOfMemory)
        );

        // Test Quantization error message
        #[cfg(feature = "quantization")]
        let quantization_error = EncoderError::Quantization(imagequant::Error::OutOfMemory);
        #[cfg(feature = "quantization")]
        assert_eq!(
            format!("{}", quantization_error),
            format!("{}", imagequant::Error::OutOfMemory)
        );

        // no need to test others 🤷‍♀️
    }
}

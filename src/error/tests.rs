use simple_error::SimpleError;

use super::*;

#[test]
fn display_decoder_error() {
    assert_eq!(
        DecodingError::IoError(io::Error::new(io::ErrorKind::NotFound, "path not found"))
            .to_string(),
        "IO Error: path not found"
    );
    assert_eq!(
        DecodingError::Format(Box::new(SimpleError::new("webp not supported"))).to_string(),
        "Format Error: webp not supported"
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
    assert_eq!(
        ConfigError::QuantizationQualityOutOfBounds.to_string(),
        "Quantization quality is out of bounds"
    );
    assert_eq!(
        ConfigError::DitheringLevelOutOfBounds.to_string(),
        "Dithering level is out of bounds"
    );
}

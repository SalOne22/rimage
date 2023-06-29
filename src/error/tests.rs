use super::*;

#[test]
fn display_decoder_error() {
    assert_eq!(
        DecodingError::IO(io::Error::new(io::ErrorKind::NotFound, "path not found")).to_string(),
        "path not found"
    );
    assert_eq!(
        DecodingError::Format("webp".to_string()).to_string(),
        "webp is not supported"
    );
}

#[test]
fn display_config_error() {
    assert_eq!(
        ConfigError::QualityOutOfBounds(200.0).to_string(),
        "200 is out of range from 0 to 100"
    );
    assert_eq!(ConfigError::WidthIsZero.to_string(), "Width cannot be zero");
    assert_eq!(
        ConfigError::HeightIsZero.to_string(),
        "Height cannot be zero"
    );
    assert_eq!(
        ConfigError::QuantizationQualityOutOfBounds(200).to_string(),
        "200 is out of range from 0 to 100"
    );
    assert_eq!(
        ConfigError::DitheringLevelOutOfBounds(2.0).to_string(),
        "2 is out of range from 0 to 1.0"
    );
}

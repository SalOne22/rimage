use super::*;

#[test]
fn config_edge_cases() {
    let config = Config::default();
    assert_eq!(config.output_format, OutputFormat::MozJpeg);
    assert_eq!(config.quality, 75.0);
    let config = Config::build(100.0, OutputFormat::Png, None, None, None).unwrap();
    assert_eq!(config.output_format, OutputFormat::Png);
    assert_eq!(config.quality, 100.0);
    let config = Config::build(0.0, OutputFormat::Oxipng, None, None, None).unwrap();
    assert_eq!(config.output_format, OutputFormat::Oxipng);
    assert_eq!(config.quality, 0.0);
    let config_result = Config::build(101.0, OutputFormat::MozJpeg, None, None, None);
    assert!(config_result.is_err());
    let config_result = Config::build(-1.0, OutputFormat::MozJpeg, None, None, None);
    assert!(config_result.is_err());
}

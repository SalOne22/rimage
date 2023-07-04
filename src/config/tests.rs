use super::*;

#[test]
fn quality_edge_cases() {
    let mut conf = Config::new(OutputFormat::MozJpeg);

    assert!(conf.quality(0.0).is_ok());
    assert!(conf.quality(50.0).is_ok());
    assert!(conf.quality(100.0).is_ok());
    assert!(conf.quality(-10.0).is_err());
    assert!(conf.quality(110.0).is_err());
}

#[test]
fn target_width_edge_cases() {
    let mut conf = Config::new(OutputFormat::MozJpeg);

    assert!(conf.target_width(Some(0)).is_err());
    assert!(conf.target_width(Some(50)).is_ok());
    assert!(conf.target_width(Some(usize::MAX)).is_ok());
}

#[test]
fn target_height_edge_cases() {
    let mut conf = Config::new(OutputFormat::MozJpeg);

    assert!(conf.target_height(Some(0)).is_err());
    assert!(conf.target_height(Some(50)).is_ok());
    assert!(conf.target_height(Some(usize::MAX)).is_ok());
}

#[test]
fn quantization_quality_edge_cases() {
    let mut conf = Config::new(OutputFormat::MozJpeg);

    assert!(conf.quantization_quality(Some(0)).is_ok());
    assert!(conf.quantization_quality(Some(50)).is_ok());
    assert!(conf.quantization_quality(Some(100)).is_ok());
    assert!(conf.quantization_quality(Some(110)).is_err());
}

#[test]
fn dithering_level_edge_cases() {
    let mut conf = Config::new(OutputFormat::MozJpeg);

    assert!(conf.dithering_level(Some(0.0)).is_ok());
    assert!(conf.dithering_level(Some(0.5)).is_ok());
    assert!(conf.dithering_level(Some(1.0)).is_ok());
    assert!(conf.dithering_level(Some(-1.0)).is_err());
    assert!(conf.dithering_level(Some(1.1)).is_err());
}

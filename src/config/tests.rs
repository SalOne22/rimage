use super::*;

#[test]
fn build_edge_cases() {
    let conf = Config::build(100.0, OutputFormat::MozJpeg);
    assert!(conf.is_ok());

    let conf = Config::build(50.0, OutputFormat::MozJpeg);
    assert!(conf.is_ok());

    let conf = Config::build(0.0, OutputFormat::MozJpeg);
    assert!(conf.is_ok());

    let conf = Config::build(-10.0, OutputFormat::MozJpeg);
    assert!(conf.is_err());

    let conf = Config::build(110.0, OutputFormat::MozJpeg);
    assert!(conf.is_err());
}

#[test]
fn quality_edge_cases() {
    let mut conf = Config::build(100.0, OutputFormat::MozJpeg).unwrap();

    assert!(conf.set_quality(0.0).is_ok());
    assert!(conf.set_quality(50.0).is_ok());
    assert!(conf.set_quality(100.0).is_ok());
    assert!(conf.set_quality(-10.0).is_err());
    assert!(conf.set_quality(110.0).is_err());
}

#[test]
fn target_width_edge_cases() {
    let mut conf = Config::build(100.0, OutputFormat::MozJpeg).unwrap();

    assert!(conf.set_target_width(Some(0)).is_err());
    assert!(conf.set_target_width(Some(50)).is_ok());
    assert!(conf.set_target_width(Some(usize::MAX)).is_ok());
}

#[test]
fn target_height_edge_cases() {
    let mut conf = Config::build(100.0, OutputFormat::MozJpeg).unwrap();

    assert!(conf.set_target_height(Some(0)).is_err());
    assert!(conf.set_target_height(Some(50)).is_ok());
    assert!(conf.set_target_height(Some(usize::MAX)).is_ok());
}

#[test]
fn quantization_quality_edge_cases() {
    let mut conf = Config::build(100.0, OutputFormat::MozJpeg).unwrap();

    assert!(conf.set_quantization_quality(Some(0)).is_ok());
    assert!(conf.set_quantization_quality(Some(50)).is_ok());
    assert!(conf.set_quantization_quality(Some(100)).is_ok());
    assert!(conf.set_quantization_quality(Some(110)).is_err());
}

#[test]
fn dithering_level_edge_cases() {
    let mut conf = Config::build(100.0, OutputFormat::MozJpeg).unwrap();

    assert!(conf.set_dithering_level(Some(0.0)).is_ok());
    assert!(conf.set_dithering_level(Some(0.5)).is_ok());
    assert!(conf.set_dithering_level(Some(1.0)).is_ok());
    assert!(conf.set_dithering_level(Some(-1.0)).is_err());
    assert!(conf.set_dithering_level(Some(1.1)).is_err());
}

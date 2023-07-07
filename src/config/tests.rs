use super::*;

#[test]
fn quality_edge_cases() {
    let mut conf = Config::builder(Codec::MozJpeg);

    assert!(conf.quality(0.0).build().is_ok());
    assert!(conf.quality(50.0).build().is_ok());
    assert!(conf.quality(100.0).build().is_ok());
    assert!(conf.quality(-10.0).build().is_err());
    assert!(conf.quality(110.0).build().is_err());
}

#[test]
fn target_width_edge_cases() {
    let mut conf = Config::builder(Codec::MozJpeg);

    assert!(conf.target_width(0).build().is_err());
    assert!(conf.target_width(50).build().is_ok());
    assert!(conf.target_width(usize::MAX).build().is_ok());
}

#[test]
fn target_height_edge_cases() {
    let mut conf = Config::builder(Codec::MozJpeg);

    assert!(conf.target_height(0).build().is_err());
    assert!(conf.target_height(50).build().is_ok());
    assert!(conf.target_height(usize::MAX).build().is_ok());
}

#[test]
fn quantization_quality_edge_cases() {
    let mut conf = Config::builder(Codec::MozJpeg);

    assert!(conf.quantization_quality(0).build().is_ok());
    assert!(conf.quantization_quality(50).build().is_ok());
    assert!(conf.quantization_quality(100).build().is_ok());
    assert!(conf.quantization_quality(110).build().is_err());
}

#[test]
fn dithering_level_edge_cases() {
    let mut conf = Config::builder(Codec::MozJpeg);

    assert!(conf.dithering_level(0.0).build().is_ok());
    assert!(conf.dithering_level(0.5).build().is_ok());
    assert!(conf.dithering_level(1.0).build().is_ok());
    assert!(conf.dithering_level(-1.0).build().is_err());
    assert!(conf.dithering_level(1.1).build().is_err());
}

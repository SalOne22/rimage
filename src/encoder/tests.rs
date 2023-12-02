use image::RgbaImage;
use std::io::Cursor;

// Import the necessary dependencies from the code
use super::*;
use crate::config::Codec;

#[test]
fn encoder_new() {
    // Create a mock image and writer
    let image_data = vec![0; 100 * 50 * 4];
    let image = RgbaImage::from_raw(100, 50, image_data).unwrap();
    let writer = Cursor::new(Vec::new());

    // Create an Encoder with default config
    let encoder = Encoder::new(writer.clone(), DynamicImage::ImageRgba8(image));

    // Verify that the Encoder was created with the correct properties
    assert_eq!(encoder.w, writer);
    assert_eq!(encoder.data.as_bytes(), &[0; 100 * 50 * 4]);
    assert_eq!(encoder.conf.codec(), &Codec::MozJpeg);
}

#[test]
fn encoder_with_config() {
    // Create a mock image and writer
    let image_data = vec![0; 100 * 50 * 4];
    let image = RgbaImage::from_raw(100, 50, image_data).unwrap();
    let writer = Cursor::new(Vec::new());

    // Create an Encoder with a custom config
    let config = EncoderConfig::new(Codec::MozJpeg)
        .with_quality(90.0)
        .unwrap();
    let encoder = Encoder::new(writer.clone(), DynamicImage::ImageRgba8(image)).with_config(config);

    // Verify that the Encoder was created with the correct properties
    assert_eq!(encoder.w, writer);
    assert_eq!(encoder.data.as_bytes(), &[0; 100 * 50 * 4]);
    assert_eq!(encoder.conf.codec(), &Codec::MozJpeg);
    assert_eq!(encoder.conf.quality(), 90.0);
}

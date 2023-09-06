use std::io::Cursor;

use rgb::RGBA8;

// Import the necessary dependencies from the code
use super::*;
use crate::config::Codec;

#[test]
fn encoder_new() {
    // Create a mock image and writer
    let image_data = vec![RGBA8::new(0, 0, 0, 0); 100 * 50];
    let image = Image::new(image_data.clone(), 100, 50);
    let writer = Cursor::new(Vec::new());

    // Create an Encoder with default config
    let encoder = Encoder::new(writer.clone(), image);

    // Verify that the Encoder was created with the correct properties
    assert_eq!(encoder.w, writer);
    assert_eq!(encoder.data.data(), &[RGBA8::new(0, 0, 0, 0); 100 * 50]);
    assert_eq!(encoder.conf.codec(), &Codec::MozJpeg);
}

#[test]
fn encoder_with_config() {
    // Create a mock image and writer
    let image_data = vec![RGBA8::new(0, 0, 0, 0); 100 * 50];
    let image = Image::new(image_data.clone(), 100, 50);
    let writer = Cursor::new(Vec::new());

    // Create an Encoder with a custom config
    let config = EncoderConfig::new(Codec::WebP).with_quality(90.0).unwrap();
    let encoder = Encoder::new(writer.clone(), image).with_config(config);

    // Verify that the Encoder was created with the correct properties
    assert_eq!(encoder.w, writer);
    assert_eq!(encoder.data.data(), &[RGBA8::new(0, 0, 0, 0); 100 * 50]);
    assert_eq!(encoder.conf.codec(), &Codec::WebP);
    assert_eq!(encoder.conf.quality(), 90.0);
}

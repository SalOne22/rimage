use std::path;

use once_cell::sync::Lazy;

use crate::{test_utils::get_files_by_regex, Decoder};

use super::*;

static FILES: Lazy<Vec<path::PathBuf>> =
    Lazy::new(|| get_files_by_regex(r"^tests/files/[^x].+\.png"));

fn encode_files<F>(files: &[path::PathBuf], conf: &Config, callback: F)
where
    F: Fn(Result<Vec<u8>, EncodingError>),
{
    files.iter().for_each(|path| {
        println!("{path:?}");
        let image = Decoder::from_path(path).unwrap().decode().unwrap();

        let encoder = Encoder::new(conf, image);
        let result = encoder.encode();

        callback(result);
    })
}

#[test]
fn encode_jpeg() {
    let conf = Config::builder(Codec::MozJpeg).build().unwrap();

    encode_files(&FILES, &conf, |result| {
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    });
}

#[test]
fn encode_png() {
    let conf = Config::builder(Codec::Png).build().unwrap();

    encode_files(&FILES, &conf, |result| {
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    });
}

#[test]
fn encode_oxipng() {
    let conf = Config::builder(Codec::Oxipng).build().unwrap();

    encode_files(&FILES, &conf, |result| {
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    });
}

#[test]
fn encode_webp() {
    let conf = Config::builder(Codec::WebP).build().unwrap();

    encode_files(&FILES, &conf, |result| {
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    });
}

#[test]
fn encode_avif() {
    let conf = Config::builder(Codec::Avif).build().unwrap();

    encode_files(&FILES, &conf, |result| {
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    });
}

#[test]
fn encode_quantized() {
    let path = path::PathBuf::from("tests/files/basi2c08.png");

    let image = Decoder::from_path(&path).unwrap().decode().unwrap();

    let conf = Config::builder(Codec::Oxipng).build().unwrap();

    let encoder = Encoder::new(&conf, image);
    let result = encoder.encode_quantized(50, 1.0);

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.is_empty());
}

#[test]
fn encode_quantized_out_of_bounds() {
    let path = path::PathBuf::from("tests/files/basi2c08.png");

    let image = Decoder::from_path(&path).unwrap().decode().unwrap();

    let conf = Config::builder(Codec::Oxipng).build().unwrap();

    let encoder = Encoder::new(&conf, image);
    let result = encoder.encode_quantized(120, 1.0);
    assert!(result.is_err());
}

#[test]
fn resize_image() {
    let data = [255; 100 * 100 * 4];
    let image = ImageData::new(100, 100, &data);

    let conf = Config::builder(Codec::Oxipng)
        .target_height(50)
        .target_width(50)
        .build()
        .unwrap();

    let mut encoder = Encoder::new(&conf, image);

    let result = encoder.resize();

    assert!(result.is_ok());
    assert_eq!(encoder.image_data.size(), (50, 50));
    assert!(encoder.image_data.data().len() < 100 * 100 * 4);
}

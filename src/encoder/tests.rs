use std::{fs, path};

use regex::Regex;

use crate::Decoder;

use super::*;

#[test]
fn encode_jpeg() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        let conf = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();

        let encoder = Encoder::new(&conf, image);
        let result = encoder.encode();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    })
}

#[test]
fn encode_png() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        let conf = Config::build(75.0, OutputFormat::Png, None, None, None).unwrap();

        let encoder = Encoder::new(&conf, image);
        let result = encoder.encode();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    })
}

#[test]
fn encode_oxipng() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        let conf = Config::build(75.0, OutputFormat::Oxipng, None, None, None).unwrap();

        let encoder = Encoder::new(&conf, image);
        let result = encoder.encode();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    })
}

#[test]
fn encode_webp() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        let conf = Config::build(75.0, OutputFormat::WebP, None, None, None).unwrap();

        let encoder = Encoder::new(&conf, image);
        let result = encoder.encode();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    })
}

#[test]
fn encode_avif() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        let conf = Config::build(75.0, OutputFormat::Avif, None, None, None).unwrap();

        let encoder = Encoder::new(&conf, image);
        let result = encoder.encode();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    })
}

#[test]
fn encode_quantized() {
    let path = path::PathBuf::from("tests/files/basi2c08.png");
    let file = fs::File::open(&path).unwrap();

    let image = Decoder::new(&path, file).decode().unwrap();

    let conf = Config::build(75.0, OutputFormat::Oxipng, None, None, None).unwrap();

    let encoder = Encoder::new(&conf, image);
    let result = encoder.encode_quantized(50, 1.0);

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.is_empty());
}

#[test]
fn encode_quantized_out_of_bounds() {
    let path = path::PathBuf::from("tests/files/basi2c08.png");
    let file = fs::File::open(&path).unwrap();

    let image = Decoder::new(&path, file).decode().unwrap();

    let conf = Config::build(75.0, OutputFormat::Oxipng, None, None, None).unwrap();

    let encoder = Encoder::new(&conf, image);
    let result = encoder.encode_quantized(120, 1.0);
    assert!(result.is_err());
}

#[test]
fn resize_image() {
    let data = [255; 100 * 100 * 4];
    let image = ImageData::new(100, 100, &data);

    let conf = Config::build(75.0, OutputFormat::Oxipng, Some(50), Some(50), None).unwrap();

    let mut encoder = Encoder::new(&conf, image);

    let result = encoder.resize();

    assert!(result.is_ok());
    assert_eq!(encoder.image_data.size(), (50, 50));
    assert!(encoder.image_data.data().len() < 100 * 100 * 4);
}

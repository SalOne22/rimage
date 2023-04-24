use regex::Regex;
use rimage::{Config, Decoder, Encoder, OutputFormat};
use std::path::PathBuf;
use std::{fs, path};

#[test]
fn test_image_processing_jpeg() {
    let path = PathBuf::from("tests/files/basi0g01.jpg");
    let file = fs::File::open(&path).unwrap();
    let decoder = Decoder::new(&path, file);
    let image = decoder.decode().unwrap();

    let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
    let encoder = Encoder::new(&config, image);
    let encoded_data = encoder.encode().unwrap();

    let output_path = "tests/files/out.jpg";
    fs::write(output_path, encoded_data).unwrap();

    let output_data = fs::read(output_path).unwrap();
    assert!(!output_data.is_empty());

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_image_processing_png() {
    let path = PathBuf::from("tests/files/basi0g01.png");
    let file = fs::File::open(&path).unwrap();
    let decoder = Decoder::new(&path, file);
    let image = decoder.decode().unwrap();

    let config = Config::build(75.0, OutputFormat::Png, None, None, None).unwrap();
    let encoder = Encoder::new(&config, image);
    let encoded_data = encoder.encode().unwrap();

    let output_path = "tests/files/out.png";
    fs::write(output_path, encoded_data).unwrap();

    let output_data = fs::read(output_path).unwrap();
    assert!(!output_data.is_empty());

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_image_processing_oxipng() {
    let path = PathBuf::from("tests/files/basi0g01.png");
    let file = fs::File::open(&path).unwrap();
    let decoder = Decoder::new(&path, file);
    let image = decoder.decode().unwrap();

    let config = Config::build(75.0, OutputFormat::Oxipng, None, None, None).unwrap();
    let encoder = Encoder::new(&config, image);
    let encoded_data = encoder.encode().unwrap();

    let output_path = "tests/files/oxiout.png";
    fs::write(output_path, encoded_data).unwrap();

    let output_data = fs::read(output_path).unwrap();
    assert!(!output_data.is_empty());

    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_bulk_image_processing_jpeg() {
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

    for path in files {
        let file = fs::File::open(&path).unwrap();
        let decoder = Decoder::new(&path, file);
        let image = decoder.decode().unwrap();

        let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
        let encoder = Encoder::new(&config, image);
        let encoded_data = encoder.encode().unwrap();

        let mut output_path = path::PathBuf::from("tests/files/");
        output_path.push(path.file_name().unwrap());
        output_path.set_extension("out.jpg");
        fs::write(&output_path, encoded_data).unwrap();

        let output_data = fs::read(&output_path).unwrap();
        assert!(!output_data.is_empty());

        fs::remove_file(&output_path).unwrap();
    }
}

#[test]
fn test_bulk_image_processing_png() {
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

    for path in files {
        let file = fs::File::open(&path).unwrap();
        let decoder = Decoder::new(&path, file);
        let image = decoder.decode().unwrap();

        let config = Config::build(75.0, OutputFormat::Png, None, None, None).unwrap();
        let encoder = Encoder::new(&config, image);
        let encoded_data = encoder.encode().unwrap();

        let mut output_path = path::PathBuf::from("tests/files/");
        output_path.push(path.file_name().unwrap());
        output_path.set_extension("out.png");
        fs::write(&output_path, encoded_data).unwrap();

        let output_data = fs::read(&output_path).unwrap();
        assert!(!output_data.is_empty());

        fs::remove_file(&output_path).unwrap();
    }
}

#[test]
fn test_bulk_image_processing_oxipng() {
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

    for path in files {
        let file = fs::File::open(&path).unwrap();
        let decoder = Decoder::new(&path, file);
        let image = decoder.decode().unwrap();

        let config = Config::build(75.0, OutputFormat::Oxipng, None, None, None).unwrap();
        let encoder = Encoder::new(&config, image);
        let encoded_data = encoder.encode().unwrap();

        let mut output_path = path::PathBuf::from("tests/files/");
        output_path.push(path.file_name().unwrap());
        output_path.set_extension("oxiout.png");
        fs::write(&output_path, encoded_data).unwrap();

        let output_data = fs::read(&output_path).unwrap();
        assert!(!output_data.is_empty());

        fs::remove_file(&output_path).unwrap();
    }
}

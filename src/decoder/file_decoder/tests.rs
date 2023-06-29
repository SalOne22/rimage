use std::{fs, path, str::FromStr};

use crate::{image::ImageData, test_utils::get_files_by_regex};

use super::*;

fn decode_files<F>(files: &[path::PathBuf], callback: F)
where
    F: Fn(Result<ImageData, DecodingError>),
{
    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let format = InputFormat::from_str(extension).unwrap();

        let image = FileDecoder::new(file, format).decode();

        callback(image)
    });
}

#[test]
fn decode_unsupported() {
    let path = path::Path::new("tests/files/test.bmp");

    let file = fs::File::open(path).unwrap();
    let extension = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let format = InputFormat::from_str(extension).unwrap();

    let decoder = FileDecoder::new(file, format);
    let result = decoder.decode();
    assert!(result.is_err());
}

#[test]
fn decode_grayscale() {
    let files = get_files_by_regex(r"^tests/files/[^x]&[^t].+0g\d\d((\.png)|(\.jpg))");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

#[test]
fn decode_grayscale_alpha() {
    let files = get_files_by_regex(r"^tests/files/[^x].+4a\d\d\.png");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

#[test]
fn decode_rgb() {
    let files = get_files_by_regex(r"^tests/files/[^x]&[^t].+2c\d\d((\.png)|(\.jpg))");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

#[test]
fn decode_rgb_transparent() {
    let files = get_files_by_regex(r"^tests/files/[^x]&[t].+2c\d\d((\.png)|(\.jpg))");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

#[test]
fn decode_rgba() {
    let files = get_files_by_regex(r"^tests/files/[^x].+6a\d\d\.png$");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

#[test]
fn decode_indexed() {
    let files = get_files_by_regex(r"^tests/files/[^x]&[^t].+3p\d\d\.png$");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

#[test]
fn decode_indexed_transparent() {
    let files = get_files_by_regex(r"^tests/files/[^x]&[t].+3p\d\d\.png$");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

#[test]
fn decode_corrupted() {
    let files = get_files_by_regex(r"^tests/files/x.+\d\d\.png$");

    decode_files(&files, |image| assert!(image.is_err()));
}

#[test]
fn decode_webp() {
    let files = get_files_by_regex(r"^tests/files/.+.webp$");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

#[test]
fn decode_avif() {
    let files = get_files_by_regex(r"^tests/files/.+.avif$");

    decode_files(&files, |image| {
        let image = image.unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    });
}

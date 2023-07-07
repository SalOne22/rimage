use std::{fs, io::Read, path, str::FromStr};

use crate::{image::ImageData, test_utils::get_files_by_regex};

use super::*;

fn decode_files<F>(files: &[path::PathBuf], callback: F)
where
    F: Fn(Result<ImageData, DecodingError>),
{
    files.iter().for_each(|path| {
        println!("{path:?}");
        let mut file = fs::File::open(path).unwrap();
        let metadata = file.metadata().unwrap();
        let mut buf = Vec::with_capacity(metadata.len() as usize);
        file.read_to_end(&mut buf).unwrap();

        let format = ImageFormat::from_str(path.extension().unwrap().to_str().unwrap()).unwrap();

        let image = MemoryDecoder::new(&buf, format).decode();

        callback(image)
    });
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

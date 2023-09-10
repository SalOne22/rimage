use std::{error::Error, fs, io::Cursor};

use super::*;

#[test]
fn new_decoder() {
    let cursor = Cursor::new(Vec::new());
    let decoder = Decoder::new(cursor.clone());

    assert_eq!(decoder.r, cursor);
    assert_eq!(decoder.format, None);
    #[cfg(feature = "transform")]
    assert_eq!(decoder.fix_orientation, None);
}

#[test]
fn new_decoder_with_format() {
    let cursor = Cursor::new(Vec::new());
    let decoder = Decoder::new(cursor.clone()).with_format(ImageFormat::Jpeg);

    assert_eq!(decoder.r, cursor);
    assert_eq!(decoder.format, Some(ImageFormat::Jpeg));
    #[cfg(feature = "transform")]
    assert_eq!(decoder.fix_orientation, None);
}

#[test]
fn decode_without_format() {
    let cursor = Cursor::new(Vec::new());
    let decoder = Decoder::new(cursor.clone());

    assert_eq!(decoder.r, cursor);
    assert_eq!(decoder.format, None);
    #[cfg(feature = "transform")]
    assert_eq!(decoder.fix_orientation, None);

    let result = decoder.decode();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Can't find image format");
}

#[test]
#[cfg(feature = "transform")]
fn new_decoder_with_fixed_orientation() {
    let cursor = Cursor::new(Vec::new());
    let decoder = Decoder::new(cursor.clone()).with_fixed_orientation(4);

    assert_eq!(decoder.r, cursor);
    assert_eq!(decoder.format, None);
    #[cfg(feature = "transform")]
    assert_eq!(decoder.fix_orientation, Some(4));
}

#[test]
#[cfg(feature = "exif")]
fn fix_orientation() -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir("tests/files/exif")?;

    for entry in files {
        let entry = entry?;
        println!("path: {:?}", entry.path());

        assert!(entry.file_type()?.is_file());

        let decoder = Decoder::from_path(&entry.path())?;
        let image = decoder.decode()?;

        assert_eq!(image.width(), 48);
        assert_eq!(image.height(), 80);
    }

    Ok(())
}

#[test]
#[cfg(feature = "avif")]
fn decode_avif() -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir("tests/files/avif")?;

    for entry in files {
        let entry = entry?;
        println!("path: {:?}", entry.path());

        assert!(entry.file_type()?.is_file());

        let decoder = Decoder::from_path(&entry.path())?;
        let image = decoder.decode()?;

        assert_eq!(image.width(), 48);
        assert_eq!(image.height(), 80);
    }

    Ok(())
}

#[test]
fn decode_jpeg() -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir("tests/files/jpg")?;

    for entry in files {
        let entry = entry?;
        println!("path: {:?}", entry.path());

        assert!(entry.file_type()?.is_file());

        let decoder = Decoder::from_path(&entry.path())?;
        let image = decoder.decode()?;

        assert!(matches!(image.width(), 48 | 80));
        assert!(matches!(image.height(), 48 | 80));
    }

    Ok(())
}

#[test]
#[cfg(feature = "jxl")]
fn decode_jpegxl() -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir("tests/files/jxl")?;

    for entry in files {
        let entry = entry?;
        println!("path: {:?}", entry.path());

        assert!(entry.file_type()?.is_file());

        let decoder = Decoder::from_path(&entry.path())?;
        let image = decoder.decode()?;

        assert_eq!(image.width(), 48);
        assert_eq!(image.height(), 80);
    }

    Ok(())
}

#[test]
fn decode_png() -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir("tests/files/png")?;

    for entry in files {
        let entry = entry?;
        println!("path: {:?}", entry.path());

        assert!(entry.file_type()?.is_file());

        let decoder = Decoder::from_path(&entry.path())?;
        let image = decoder.decode()?;

        assert_eq!(image.width(), 48);
        assert_eq!(image.height(), 80);
    }

    Ok(())
}

#[test]
#[cfg(feature = "webp")]
fn decode_webp() -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir("tests/files/webp")?;

    for entry in files {
        let entry = entry?;
        println!("path: {:?}", entry.path());

        assert!(entry.file_type()?.is_file());

        let decoder = Decoder::from_path(&entry.path())?;
        let image = decoder.decode()?;

        assert_eq!(image.width(), 48);
        assert_eq!(image.height(), 80);
    }

    Ok(())
}

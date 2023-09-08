use std::{error::Error, fs};

use super::*;

#[test]
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

        assert_eq!(image.width(), 48);
        assert_eq!(image.height(), 80);
    }

    Ok(())
}

#[test]
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

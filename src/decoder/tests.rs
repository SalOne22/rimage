use std::{error::Error, fs};

use super::*;

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

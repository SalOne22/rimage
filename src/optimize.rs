use std::{error::Error, fs, path};

use log::info;

use crate::{decoder::Decoder, image::ImageFormat, Config, Encoder};

/// Optimizes one image with provided config
///
/// # Example
/// ```
/// # use rimage::{Config, optimize, image::Codec};
/// // Get file path
/// let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
///
/// // Build config for encoding
/// let config = Config::builder(Codec::MozJpeg).build().unwrap();
///
/// // Get encoded image data from encoder
/// let data = optimize(&path, &config).unwrap();
///
/// // Do something with image data...
/// ```
///
/// # Errors
///
/// This function can return any error that happens during decoding and encoding process
pub fn optimize(image_path: &path::Path, config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = fs::File::open(image_path)?;

    info!("read {} bytes", file.metadata().unwrap().len());

    let d = Decoder::from_path(image_path)?;
    let e = Encoder::new(config, d.decode()?);

    Ok(e.encode()?)
}

/// Optimizes one image from memory with provided config
///
/// # Example
/// ```
/// use std::io::Read;
/// # use rimage::{optimize_from_memory, image::{ImageFormat, Codec}, Config};
///
/// // Get file data
/// let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
/// let mut file = std::fs::File::open(path).unwrap();
/// let metadata = file.metadata().unwrap();
/// let mut data = Vec::with_capacity(metadata.len() as usize);
/// file.read_to_end(&mut data).unwrap();
///
/// // Build config for encoding
/// let config = Config::builder(Codec::MozJpeg).build().unwrap();
///
/// // Get encoded image data from encoder
/// let data = optimize_from_memory(&data, ImageFormat::Jpeg, &config).unwrap();
///
/// // Do something with image data...
/// ```
///
/// # Errors
///
/// This function can return any error that happens during decoding and encoding process
pub fn optimize_from_memory(
    data: &[u8],
    input_format: ImageFormat,
    config: &Config,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let d = Decoder::from_mem(data, input_format);
    let e = Encoder::new(config, d.decode()?);

    Ok(e.encode()?)
}

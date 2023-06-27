use std::{error::Error, fs, path};

use log::info;

use crate::{image::InputFormat, Config, Decoder, Encoder, MemoryDecoder};

/// Optimizes one image with provided config
pub fn optimize(image_path: &path::Path, config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = fs::File::open(image_path)?;

    info!("read {} bytes", file.metadata().unwrap().len());

    let d = Decoder::new(image_path, file);
    let e = Encoder::new(config, d.decode()?);

    Ok(e.encode()?)
}

/// Optimizes one image from memory with provided config
pub fn optimize_from_memory(
    data: &[u8],
    input_format: InputFormat,
    config: &Config,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let d = MemoryDecoder::new(data, input_format);
    let e = Encoder::new(config, d.decode()?);

    Ok(e.encode()?)
}

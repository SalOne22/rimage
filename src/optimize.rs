use std::{error::Error, fs, path};

use log::info;

use crate::{Config, Decoder, Encoder};

/// Optimizes one image with provided config
pub fn optimize(image_path: &path::Path, config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = fs::File::open(image_path)?;

    info!("read {} bytes", file.metadata().unwrap().len());

    let d = Decoder::new(image_path, file);
    let e = Encoder::new(config, d.decode()?);

    Ok(e.encode()?)
}

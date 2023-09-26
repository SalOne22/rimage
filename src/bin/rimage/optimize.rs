use std::{
    error::Error,
    fs::{self, File},
    path::{Path, PathBuf},
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use rimage::{config::EncoderConfig, Decoder, Encoder};

#[cfg(not(feature = "parallel"))]
pub fn optimize_files(
    paths: impl IntoIterator<Item = (PathBuf, PathBuf)>,
    conf: EncoderConfig,
    backup: bool,
) {
    paths
        .into_iter()
        .for_each(move |(input, output): (PathBuf, PathBuf)| {
            optimize(&input, &output, conf.clone(), backup).unwrap_or_else(|e| {
                dbg!(&e);
                eprintln!("{input:?}: {e}");
            });
        });
}

#[cfg(feature = "parallel")]
pub fn optimize_files(
    paths: impl IntoParallelIterator<Item = (PathBuf, PathBuf)>,
    conf: EncoderConfig,
    backup: bool,
) {
    paths
        .into_par_iter()
        .for_each(move |(input, output): (PathBuf, PathBuf)| {
            optimize(&input, &output, conf.clone(), backup).unwrap_or_else(|e| {
                eprintln!("{input:?}: {e}");
            });
        });
}

fn optimize(
    in_path: &Path,
    out_path: &Path,
    conf: EncoderConfig,
    backup: bool,
) -> Result<(), Box<dyn Error>> {
    let decoder = Decoder::from_path(in_path)?;

    if backup {
        fs::rename(
            in_path,
            format!("{}.backup", in_path.as_os_str().to_str().unwrap()),
        )?;
    }

    let image = decoder.decode()?;

    fs::create_dir_all(out_path.parent().unwrap())?;
    let out_file = File::create(out_path)?;

    let encoder = Encoder::new(out_file, image).with_config(conf);
    encoder.encode()?;

    Ok(())
}

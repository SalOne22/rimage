use console::{Emoji, Style};
use indicatif::{DecimalBytes, MultiProgress};
use std::{
    error::Error,
    fs::{self, File},
    path::{Path, PathBuf},
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use rimage::{config::EncoderConfig, Decoder, Encoder};

use crate::progress_bar::{create_spinner, set_error};

#[cfg(not(feature = "parallel"))]
pub fn optimize_files(
    paths: impl IntoIterator<Item = (PathBuf, PathBuf)>,
    conf: EncoderConfig,
    backup: bool,
    m: &MultiProgress,
) {
    paths
        .into_iter()
        .for_each(move |paths| optimize_file(paths, conf.clone(), backup, m));
}

#[cfg(feature = "parallel")]
pub fn optimize_files(
    paths: impl IntoParallelIterator<Item = (PathBuf, PathBuf)>,
    conf: EncoderConfig,
    backup: bool,
    m: &MultiProgress,
) {
    paths
        .into_par_iter()
        .for_each(move |paths| optimize_file(paths, conf.clone(), backup, m));
}

fn optimize_file(
    (input, output): (PathBuf, PathBuf),
    conf: EncoderConfig,
    backup: bool,
    m: &MultiProgress,
) {
    let file_name = input
        .file_name()
        .expect("Filename should be present")
        .to_str()
        .unwrap()
        .to_owned();

    let spinner = create_spinner(file_name.clone(), m);

    let file_size_before = match fs::metadata(&input) {
        Ok(meta) => meta.len(),
        Err(e) => {
            set_error(&spinner, &file_name, &e.to_string());
            return;
        }
    };

    match optimize(&input, &output, conf.clone(), backup) {
        Ok(()) => {}
        Err(e) => {
            set_error(&spinner, &file_name, &e.to_string());
            return;
        }
    };

    let file_size_after = match fs::metadata(&output) {
        Ok(meta) => meta.len(),
        Err(e) => {
            set_error(&spinner, &file_name, &e.to_string());
            return;
        }
    };

    let diff = file_size_after as f64 / file_size_before as f64;
    let abs_percent = diff.abs() * 100.0;
    let percent = if diff > 1.0 {
        abs_percent - 100.0
    } else {
        100.0 - abs_percent
    };

    let cyan = Style::new().cyan();

    spinner.set_prefix(format!("{}", Emoji("✅", "Done")));
    spinner.finish_with_message(format!(
        "{file_name} completed {} -> {} {}",
        cyan.apply_to(DecimalBytes(file_size_before)),
        cyan.apply_to(DecimalBytes(file_size_after)),
        if file_size_after > file_size_before {
            Style::new()
                .red()
                .apply_to(format!("{} {:.1}%", Emoji("🔺", "^"), percent))
        } else {
            Style::new()
                .green()
                .apply_to(format!("{} {:.1}%", Emoji("🔻", "v"), percent))
        }
    ));
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

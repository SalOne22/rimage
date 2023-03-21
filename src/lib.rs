//! # Rimage
//! `rimage` is CLI tool that compress multiple images at once.
//! Also it provides lib crate with functions to decode and encode images

use clap::Parser;

/// Config from command line input
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Input files to be compressed
    pub input: Vec<std::path::PathBuf>,

    /// Quality of output images from 0 to 1
    #[arg(short, long, default_value_t = 0.75)]
    pub quality: f32,

    /// Format of output images
    #[arg(short, long, default_value_t = String::from("jpg"))]
    pub output_format: String,
}

/// Decoders for images
pub mod decoders;
/// Encoders for images
pub mod encoders;

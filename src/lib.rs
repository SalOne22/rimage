use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Input files to be compressed
    pub input: Vec<std::path::PathBuf>,

    /// Quality of output images from 0 to 1
    #[arg(short, long, default_value_t = 0.75)]
    pub quality: f32,
}

pub mod decoders;
pub mod encoders;

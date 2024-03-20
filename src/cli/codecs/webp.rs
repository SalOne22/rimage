use clap::{arg, value_parser, Command};

use crate::cli::common::CommonArgs;

pub fn webp() -> Command {
    Command::new("webp")
        .about("Encode images into WebP format. (Lossless-able)")
        .args([
            arg!(--lossless "Encode image without quality loss.").conflicts_with_all(["quality"]),
            arg!(-q --quality <NUM> "Quality, values 60-80 are recommended.")
                .value_parser(value_parser!(u8).range(1..=100))
                .default_value("75"),
            arg!(--slight_loss <NUM> "Slight loss in quality for lossless encoding.")
                .value_parser(value_parser!(u8).range(0..=100))
                .default_value("0")
                .requires("lossless"),
            arg!(--discrete "Discrete tone image.").requires("lossless"),
            arg!(--exact "Preserve transparent data."),
        ])
        .common_args()
}

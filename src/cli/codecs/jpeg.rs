use clap::{arg, value_parser, Command};

use crate::cli::common::CommonArgs;

pub fn jpeg() -> Command {
    Command::new("jpeg")
        .alias("jpg")
        .about("Encode images into JPEG format. (Progressive-able)")
        .args([
            arg!(-q --quality <NUM> "Quality which the image will be encoded with.")
                .value_parser(value_parser!(u8).range(1..=100)),
            arg!(--progressive "Set to use progressive encoding."),
        ])
        .common_args()
}

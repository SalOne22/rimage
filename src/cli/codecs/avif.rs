use clap::{arg, value_parser, Command};
use indoc::indoc;

use crate::cli::common::CommonArgs;

pub fn avif() -> Command {
    Command::new("avif")
        .about("Encode images into AVIF format. (Small and Efficient)")
        .args([
            arg!(-q --quality <NUM> "Quality which the image will be encoded with.")
                .value_parser(value_parser!(u8).range(1..=100))
                .default_value("50"),
            arg!(--alpha_quality <NUM> "Separate alpha quality which the image will be encoded with.")
                .value_parser(value_parser!(u8).range(1..=100)),
            arg!(--speed <NUM> "Compression speed (effort).")
                .long_help(indoc! {r#"Compression speed (effort).

                1 = very very slow, but max compression (smallest)
                10 = quick, but larger file sizes and lower quality."#})
                .value_parser(value_parser!(u8).range(1..=10))
                .default_value("6"),
            arg!(--colorspace <COLOR> "Set color space of AVIF being written.")
                .value_parser(["ycbcr", "rgb"])
                .default_value("ycbcr"),
            arg!(--alpha_mode <MODE> "Configure handling of color channels in transparent images.")
                .value_parser(["UnassociatedDirty", "UnassociatedClean", "Premultiplied"])
                .default_value("UnassociatedClean")
        ]).common_args()
}

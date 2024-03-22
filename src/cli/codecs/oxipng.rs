use clap::{arg, value_parser, Command};
use indoc::indoc;

use crate::cli::common::CommonArgs;

pub fn oxipng() -> Command {
    Command::new("oxipng")
        .alias("oxi")
        .about("Encode images into PNG format using OxiPNG codec. (Progressive-able)")
        .args([
            arg!(--interlace "Set interlace mode (progressive)."),
            arg!(--effort <NUM> "Optimization level (0-6)").long_help(indoc! {r#"Set the optimization level preset.
            The default level 2 is quite fast and provides good compression.
            
            Lower levels are faster, higher levels provide better compression, though with increasingly diminishing returns.

            0   => (1 trial, determined heuristically)
            1   => (1 trial, determined heuristically)
            2   => (4 fast trials, 1 main trial)
            3   => (4 trials)
            4   => (4 trials)
            5   => (8 trials)
            6   => (10 trials)"#})
            .value_parser(value_parser!(u8).range(0..=6))
            .default_value("2"),
        ]).common_args()
}

use clap::{arg, value_parser, Command};

pub fn jpeg() -> Command {
    Command::new("jpeg")
        .alias("jpg")
        .about("Encode images into JPEG format.")
        .args([
            arg!(-q --quality <NUM> "Quality which the image will be encoded with.")
                .value_parser(value_parser!(u8).range(1..=100)),
            arg!(--progressive "Set to use progressive encoding."),
        ])
}

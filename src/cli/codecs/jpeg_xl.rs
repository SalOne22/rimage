use clap::Command;
use indoc::indoc;

pub fn jpeg_xl() -> Command {
    Command::new("jpeg_xl")
        .alias("jxl")
        .about("Encode images into jpeg xl format")
        .long_about(indoc! {"Encode images into jpeg xl format

        Only supports lossless encoding"})
}

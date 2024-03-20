use clap::Command;
use indoc::indoc;

use crate::cli::common::CommonArgs;

pub fn jpeg_xl() -> Command {
    Command::new("jpeg_xl")
        .alias("jxl")
        .about("Encode images into JpegXL format. (Big but Lossless)")
        .long_about(indoc! {r#"Encode images into jpeg xl format.

        Only supports lossless encoding"#})
        .common_args()
}

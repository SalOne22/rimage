use clap::Command;
use indoc::indoc;

pub fn jpeg_xl() -> Command {
    Command::new("jpeg_xl")
        .alias("jxl")
        .about("Encode images into JpegXL format. (Big but Lossless)")
        .long_about(indoc! {r#"Encode images into jpeg xl format.

        Only supports lossless encoding"#})
}

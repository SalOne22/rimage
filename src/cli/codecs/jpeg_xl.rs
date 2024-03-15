use clap::Command;

pub fn jpeg_xl() -> Command {
    Command::new("jpeg_xl")
        .about("Encode images into jpeg xl format")
        .alias("jxl")
}

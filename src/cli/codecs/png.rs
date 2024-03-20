use clap::Command;

use crate::cli::common::CommonArgs;

pub fn png() -> Command {
    Command::new("png")
        .about("Encode images into PNG format.")
        .common_args()
}

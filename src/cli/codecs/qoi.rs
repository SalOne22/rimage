use clap::Command;

use crate::cli::common::CommonArgs;

pub fn qoi() -> Command {
    Command::new("qoi")
        .about("Encode images into QOI format. (Trendy and Small)")
        .common_args()
}

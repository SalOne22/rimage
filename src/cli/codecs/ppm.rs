use clap::Command;

use crate::cli::common::CommonArgs;

pub fn ppm() -> Command {
    Command::new("ppm")
        .about("Encode images into PPM format. (Bitmapped)")
        .common_args()
}

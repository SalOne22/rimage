use clap::Command;

use crate::cli::common::CommonArgs;

pub fn farbfeld() -> Command {
    Command::new("farbfeld")
        .about("Encode images into Farbfeld format. (Bitmapped)")
        .common_args()
}

use clap::Command;

pub fn png() -> Command {
    Command::new("png").about("Encode images into PNG format. (Common)")
}

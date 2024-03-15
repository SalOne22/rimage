use clap::Command;

pub fn farbfeld() -> Command {
    Command::new("farbfeld").about("Encode images into Farbfeld format")
}

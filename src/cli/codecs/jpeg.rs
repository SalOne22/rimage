use clap::Command;

pub fn jpeg() -> Command {
    Command::new("jpeg").about("Encode images into JPEG format")
}

use clap::Command;

pub fn ppm() -> Command {
    Command::new("ppm").about("Encode images into PPM format. (Uncommon)")
}

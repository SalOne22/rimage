use clap::Command;

pub fn qoi() -> Command {
    Command::new("qoi").about("Encode images into QOI format. (Trendy and Small)")
}

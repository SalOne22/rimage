use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input files to be compressed
    input: Vec<std::path::PathBuf>,

    /// Quality of output images from 0 to 1
    #[arg(short, long, default_value_t = 0.75)]
    quality: f32,
}

fn main() {
    let args = Args::parse_from(wild::args_os());

    println!("Args: {args:#?}");
}

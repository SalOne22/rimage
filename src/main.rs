use std::{io, path, process};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rimage::{decoders, encoders, image::OutputFormat, Config, Decoder};

#[derive(Parser)]
#[command(author, about, version, long_about = None)]
struct Args {
    /// Input file(s)
    input: Vec<path::PathBuf>,
    /// Quality of the output image (0-100)
    #[arg(short, long, default_value = "75")]
    quality: f32,
    /// Output format of the output image
    #[arg(short, long, default_value = "jpg")]
    output_format: OutputFormat,
    /// Print image info
    #[arg(short, long)]
    info: bool,
}

fn main() {
    let mut args = Args::parse_from(wild::args_os());
    let conf = if let Ok(conf) = Config::build(&args.input, args.quality, args.output_format) {
        conf
    } else {
        eprintln!("Error: Invalid configuration.");
        process::exit(1);
    };
    let pb = ProgressBar::new(args.input.len() as u64);

    if args.input.is_empty() {
        args.input = io::stdin()
            .lines()
            .map(|res| {
                let input_file = res.unwrap();
                path::PathBuf::from(input_file.trim())
            })
            .collect();
    }

    pb.set_style(
        ProgressStyle::with_template("{bar:40.green/blue}  {pos}/{len} {wide_msg}")
            .unwrap()
            .progress_chars("##*"),
    );
    pb.set_position(0);

    if args.info {
        for path in &args.input {
            let d = match Decoder::build(path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{} Error: {e}", path.file_name().unwrap().to_str().unwrap());
                    continue;
                }
            };

            let img = match d.decode() {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("{} Error: {e}", path.file_name().unwrap().to_str().unwrap());
                    continue;
                }
            };

            println!("{:?}", path.file_name().unwrap());
            println!("Color Space: {:?}", img.color_space());
            println!("Bit Depth: {:?}", img.bit_depth());
            println!("Size: {:?}", img.size());
            println!("Data length: {:?}", img.data().len());
            println!();
        }
        process::exit(0);
    }

    for path in &args.input {
        pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());
        pb.inc(1);

        let (pixels, width, height) = match decoders::decode_image(path) {
            Ok(data) => data,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match encoders::encode_image(
            path,
            &pixels,
            &args.output_format.to_string(),
            width,
            height,
            args.quality,
        ) {
            Ok(()) => (),
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
    }
    pb.finish();
}

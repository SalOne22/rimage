use std::{io, path, process};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rimage::{decoders, encoders, Config, Decoder};

fn main() {
    let mut conf = Config::parse_from(wild::args_os());
    let pb = ProgressBar::new(conf.input.len() as u64);

    if conf.input.is_empty() {
        conf.input = io::stdin()
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

    if conf.info {
        for path in &conf.input {
            let d = Decoder::build(path).unwrap();
            let img = d.decode().unwrap();

            println!("{:?}", path.file_name().unwrap());
            println!("Color Space: {:?}", img.color_space());
            println!("Bit Depth: {:?}", img.bit_depth());
            println!("Size: {:?}", img.size());
            println!("Data length: {:?}", img.data().len());
            println!();
        }
        process::exit(0);
    }

    for path in &conf.input {
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
            &conf.output_format,
            width,
            height,
            conf.quality,
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

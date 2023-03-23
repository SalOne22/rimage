use std::process;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rimage::{decoders, encoders, Commands, Config, Decoder};

fn main() {
    let conf = Config::parse_from(wild::args_os());
    let pb = ProgressBar::new(conf.input.len() as u64);

    pb.set_style(
        ProgressStyle::with_template("{bar:40.green/blue}  {pos}/{len} {wide_msg}")
            .unwrap()
            .progress_chars("##*"),
    );
    pb.set_position(0);

    match conf.command {
        Commands::Info { input } => {
            for path in input {
                let d = match Decoder::build(&path) {
                    Ok(dec) => dec,
                    Err(e) => {
                        println!("Error: {e}");
                        continue;
                    }
                };
                let img_data = match d.decode() {
                    Ok(img_d) => img_d,
                    Err(e) => {
                        println!("Error: {e}");
                        continue;
                    }
                };

                println!("Path: {:?}", path);
                println!("WxH: {:?}", img_data.size());
                println!("Color space: {:?}", img_data.color_space());
                println!("Bytes: {}", img_data.data_len());
                println!("");
            }
            process::exit(0);
        }
    }

    for path in conf.input {
        pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());
        pb.inc(1);

        let (pixels, width, height) = decoders::decode_image(&path).unwrap();

        encoders::encode_image(
            &path,
            &pixels,
            &conf.output_format,
            width,
            height,
            conf.quality,
        )
        .unwrap();
    }
    pb.finish();
}

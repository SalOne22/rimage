use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rimage::{decoders, encoders, Config};

fn main() {
    let conf = Config::parse_from(wild::args_os());
    let pb = ProgressBar::new(conf.input.len() as u64);

    pb.set_style(
        ProgressStyle::with_template("{bar:40.green/blue}  {pos}/{len} {wide_msg}")
            .unwrap()
            .progress_chars("##*"),
    );
    pb.set_position(0);

    for path in conf.input {
        pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());
        pb.inc(1);

        let (pixels, width, height) = match decoders::decode_image(&path) {
            Ok(data) => data,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match encoders::encode_image(
            &path,
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

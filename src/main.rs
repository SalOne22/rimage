use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use rayon::{prelude::*, ThreadPoolBuilder};
use rimage::{decoders, encoders, Config};

fn main() {
    pretty_env_logger::init();
    let conf = Config::parse_from(wild::args_os());
    let pb = ProgressBar::new(conf.input.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("{bar:40.green/blue}  {pos}/{len}")
            .unwrap()
            .progress_chars("##*"),
    );
    pb.set_position(0);

    let pool = ThreadPoolBuilder::new().build().unwrap();

    pool.install(|| {
        conf.input.par_iter().for_each(|path| {
            info!("Processing {path:?}");
            let (pixels, width, height) = decoders::decode_image(path).unwrap();

            encoders::encode_image(
                path,
                &pixels,
                &conf.output_format,
                width,
                height,
                conf.quality,
            )
            .unwrap();

            pb.inc(1);
        })
    });

    pb.finish();
}

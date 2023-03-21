use clap::Parser;
use rimage::{decoders, encoders, Config};

fn main() {
    let conf = Config::parse_from(wild::args_os());

    for path in conf.input {
        let (pixels, width, height) = decoders::decode_image(&path).unwrap();

        encoders::encode_image(&path, &pixels, "jpg", width, height, conf.quality).unwrap();
    }
}

use clap::Command;

use self::{
    farbfeld::farbfeld, jpeg::jpeg, jpeg_xl::jpeg_xl, mozjpeg::mozjpeg, png::png, ppm::ppm,
    qoi::qoi,
};

mod farbfeld;
mod jpeg;
mod jpeg_xl;
mod mozjpeg;
mod png;
mod ppm;
mod qoi;

impl Codecs for Command {
    fn codecs(self) -> Self {
        self.subcommands([
            farbfeld(),
            jpeg(),
            jpeg_xl(),
            mozjpeg(),
            png(),
            ppm(),
            qoi(),
        ])
    }
}

pub trait Codecs {
    fn codecs(self) -> Self;
}

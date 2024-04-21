use clap::Command;

use self::{
    avif::avif, farbfeld::farbfeld, jpeg::jpeg, jpeg_xl::jpeg_xl, jpegli::jpegli, oxipng::oxipng,
    png::png, ppm::ppm, qoi::qoi, webp::webp,
};

mod avif;
mod farbfeld;
mod jpeg;
mod jpeg_xl;
mod jpegli;
mod oxipng;
mod png;
mod ppm;
mod qoi;
mod webp;

impl Codecs for Command {
    fn codecs(self) -> Self {
        self.subcommands([
            avif(),
            farbfeld(),
            jpeg(),
            jpeg_xl(),
            jpegli(),
            oxipng(),
            png(),
            ppm(),
            qoi(),
            webp(),
        ])
    }
}

pub trait Codecs {
    fn codecs(self) -> Self;
}

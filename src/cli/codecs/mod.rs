use clap::Command;

use self::jpeg::jpeg;

mod jpeg;

impl Codecs for Command {
    fn codecs(self) -> Self {
        self.subcommands([jpeg()])
    }
}

pub trait Codecs {
    fn codecs(self) -> Self;
}

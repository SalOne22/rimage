use clap::{command, Command};

use self::codecs::Codecs;

pub mod codecs;
pub mod common;
pub mod pipeline;
pub mod preprocessors;
pub mod utils;

pub fn cli() -> Command {
    command!().arg_required_else_help(true).codecs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        cli().debug_assert();
    }
}

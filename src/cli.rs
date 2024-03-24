use clap::{command, Command};
use indoc::indoc;

use self::codecs::Codecs;

pub mod codecs;
pub mod common;
pub mod pipeline;
pub mod preprocessors;
pub mod utils;

pub fn cli() -> Command {
    command!()
        .arg_required_else_help(true)
        .after_help(indoc! {r#"List of supported codecs

        | Image Format | Decoder       | Encoder                 |
        | ------------ | ------------- | ----------------------- |
        | bmp          | zune-bmp      | X                       |
        | jpeg         | zune-jpeg     | mozjpeg or jpeg-encoder |
        | png          | zune-png      | oxipng or zune-png      |
        | avif         | libavif       | ravif                   |
        | webp         | webp          | webp                    |
        | ppm          | zune-ppm      | zune-ppm                |
        | qoi          | zune-qoi      | zune-qoi                |
        | farbfeld     | zune-farbfeld | zune-farbfeld           |
        | psd          | zune-psd      | X                       |
        | jpeg-xl      | jxl-oxide     | zune-jpegxl             |
        | hdr          | zune-hdr      | zune-hdr                |

        List of supported preprocessing options

        - Resize
        - Quantization"#})
        .codecs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        cli().debug_assert();
    }
}

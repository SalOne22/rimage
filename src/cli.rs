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


| Image Format  | Input | Output | Note            |
| ------------- | ----- | ------ | --------------- |
| avif          | O     | O      | Static only     |
| bmp           | O     | X      |                 |
| farbfeld      | O     | O      |                 |
| hdr           | O     | O      |                 |
| jpeg          | O     | O      |                 |
| jpeg_xl(jxl)  | O     | O      |                 |
| mozjpeg(moz)  | O     | O      |                 |
| oxipng(oxi)   | O     | O      | Static only     |
| png           | O     | O      | Static only     |
| ppm           | O     | O      |                 |
| psd           | O     | X      |                 |
| qoi           | O     | O      |                 |
| webp          | O     | O      | Static only     |

List of supported preprocessing options

- Resize
- Quantization
- Alpha premultiply"#})
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

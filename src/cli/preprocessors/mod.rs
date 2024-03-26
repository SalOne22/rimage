#![allow(unused_imports)]

use clap::{arg, value_parser, Arg, ArgAction, ArgGroup, Command};
use indoc::indoc;

#[cfg(feature = "resize")]
pub use resize::{ResizeFilter, ResizeValue};

#[cfg(feature = "resize")]
mod resize;

impl Preprocessors for Command {
    #[cfg(any(feature = "resize", feature = "quantization"))]
    fn preprocessors(self) -> Self {
        self.group(
                ArgGroup::new("preprocessors")
                    .args([
                        #[cfg(feature = "resize")]
                        "resize",
                        #[cfg(feature = "quantization")]
                         "quantization",
                    ])
                    .multiple(true)
            )
            .next_help_heading("Preprocessors")
            .args([
                #[cfg(feature = "resize")]
                arg!(--resize <RESIZE> "Resize the image(s) according to the specified criteria.")
                    .long_help(indoc! {r#"Resize the image(s) according to the specified criteria.

                    Possible values:
                    - @1.5:    Enlarge image size by this multiplier
                    - 150%:    Adjust image size by this percentage
                    - 100x100: Resize image to these dimensions
                    - 200x_:   Adjust image dimensions while maintaining the aspect ratio based on the specified dimension"#})
                    .value_parser(value_parser!(ResizeValue))
                    .action(ArgAction::Append),

                #[cfg(feature = "resize")]
                arg!(--filter <FILTER> "Filter that used when resizing an image.")
                    .value_parser(value_parser!(ResizeFilter))
                    .default_value("lanczos3")
                    .requires("resize"),

                #[cfg(feature = "quantization")]
                arg!(--quantization [QUALITY] "Enables quantization with optional quality.")
                    .long_help(indoc! {r#"Enables quantization with optional quality in percentage.

                    If quality is not provided, default 75 is used"#})
                    .value_parser(value_parser!(u8).range(1..=100))
                    .action(ArgAction::Append)
                    .default_missing_value("75"),

                #[cfg(feature = "quantization")]
                arg!(--dithering [QUALITY] "Enables dithering with optional quality.")
                    .long_help(indoc! {r#"Enables dithering with optional quality in percentage.

                    Used with --quantization flag.
                    If quality is not provided, default 75 is used."#})
                    .value_parser(value_parser!(u8).range(1..=100))
                    .default_missing_value("75")
                    .requires("quantization"),

                position_sensitive_flag(arg!(--premultiply "Premultiply alpha before operation"))
                    .action(ArgAction::Append)
            ])
    }
}

fn position_sensitive_flag(arg: Arg) -> Arg {
    // Flags don't track the position of each occurrence, so we need to emulate flags with
    // value-less options to get the same result
    arg.num_args(0)
        .value_parser(value_parser!(bool))
        .default_missing_value("true")
        .default_value("false")
}

pub trait Preprocessors {
    fn preprocessors(self) -> Self;
}

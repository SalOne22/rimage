use clap::{arg, value_parser, ArgGroup, Command};
use indoc::indoc;

use resize::ResizeValue;

use crate::preprocessors::resize::ResizeFilter;

mod resize;

impl Preprocessors for Command {
    fn preprocessors(self) -> Self {
        self.group(ArgGroup::new("preprocessors").multiple(true))
            .next_help_heading("Preprocessors")
            .arg(
                arg!(--resize <RESIZE> "Resize the image(s) according to the specified criteria")
                    .long_help(indoc! {"Resize the image(s) according to the specified criteria
    
                    Possible values:
                    - @1.5:    Enlarge image size by this multiplier
                    - 150%:    Adjust image size by this percentage
                    - 100x100: Resize image to these dimensions
                    - 200x_:   Adjust image dimensions while maintaining the aspect ratio based on the specified dimension"})
                    .value_parser(value_parser!(ResizeValue)),
            )
            .arg(
                arg!(--filter <FILTER> "Filter that used when resizing an image")
                    .value_parser(value_parser!(ResizeFilter))
                    .default_value("lanczos3")
                    .requires("resize"),
            )
            .arg(
                arg!(--quantization [QUALITY] "Enables quantization with optional quality")
                    .long_help(indoc! {"Enables quantization with optional quality
    
                    If quality is not provided default 75% quality is used"})
                    .value_parser(value_parser!(u8).range(1..=100))
                    .default_missing_value("75")
            )
            .arg(
                arg!(--dithering [QUALITY] "Enables dithering with optional quality")
                    .long_help(indoc! {"Enables dithering with optional quality
    
                    Used with --quantization flag.
                    If quality is not provided default 75% quality is used"})
                    .value_parser(value_parser!(u8).range(1..=100))
                    .default_missing_value("75")
                    .requires("quantization")
            )
    }
}

pub trait Preprocessors {
    fn preprocessors(self) -> Self;
}

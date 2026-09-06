use std::path::PathBuf;

use clap::{ArgAction, Command, arg, value_parser};
use indoc::indoc;

use super::{preprocessors::Preprocessors, utils::threads};

impl CommonArgs for Command {
    fn common_args(self) -> Self {
        let cmd = self
        .next_help_heading("General").args([
            arg!(files: <FILES> ... "Input file(s) to process.")
                .long_help(indoc! {r#"Input file(s) to process.

                If the file path contains spaces, enclose the path with double quotation marks on both sides.

                A file named `file.list` is read as a UTF-8 file list with one input file per line. Blank lines are skipped, surrounding whitespace is ignored, and relative paths are resolved against the current working directory. When a `file.list` is provided, all other input file arguments are ignored."#})
                .value_parser(value_parser!(PathBuf)),
            arg!(-d --directory <DIR> "The directory to write output file(s) to.")
                .long_help(indoc! {r#"The directory to write output file(s) to.

                Output files will be written without preserving the folder structure unless the --recursive flag is used."#})
                .value_parser(value_parser!(PathBuf)),
            arg!(-r --recursive "Preserves the folder structure when writing output file(s).")
                .long_help(indoc! {r#"Preserves the folder structure when writing output file(s).

                This option should be used in conjunction with the --directory option."#})
                .requires("directory"),
            arg!(-s --suffix [SUFFIX] "Adds the suffix to the names of output file(s).")
                .long_help(indoc! {r#"Adds the suffix to the names of output file(s).

                When '2x' is provided as the value, the resulting files will be renamed with the '2x' suffix.
                For example, a file named 'file.jpeg' will become 'file2x.jpeg'.

                If no suffix is provided, the default suffix 'updated' will be added to the resulting files."#})
                .default_missing_value("updated"),
            arg!(-b --backup "Adds the '@backup' to the names of input file(s)."),
            arg!(-t --threads <NUM> "The maximum number of images to process concurrently.")
                .long_help(indoc! {r#"The maximum number of images to process concurrently.

                Limits how many images are decoded and held in memory at once.
                Higher values increase speed but use more RAM, which may cause out-of-memory errors with large images.
                By default, processes one image at a time (--threads 1)."#})
                .value_parser(value_parser!(u8).range(1i64..=threads::num_threads() as i64)),
            arg!(-x --strip "Strip metadata when encoding images (where supported)")
                .action(ArgAction::SetTrue),
            arg!(--"no-progress" "Disables progress bar.")
                .long_help(indoc! {r#"Disables progress bar.

                By default, progress bar is enabled."#}),
            arg!(--quiet "Disables all output.")
                .long_help(indoc! {r#"Disables all output.

                By default, all output is enabled."#}),
            #[cfg(feature = "metadata")]
            arg!(--metadata [FILE] "Outputs metadata of the processed image(s).")
                .long_help(indoc! {r#"Outputs metadata of the processed image(s).

                This will output the metadata of the processed image(s) in JSON format."#})
                .value_parser(value_parser!(PathBuf)),
        ]);

        #[cfg(feature = "svg")]
        let cmd = cmd.next_help_heading("SVG").args([
            arg!(--"svg-scale" <SCALE> "Uniform scale factor for SVG input(s); rendering is lossless at any size.")
                .long_help(indoc! {r#"Uniform scale factor applied when rendering SVG input(s).

SVG is rendered straight from the vector data at the requested size, so enlarging is lossless: edges, gradients and text stay exactly as sharp as at the original size. This differs from --resize on a raster image, which resamples an already-rendered bitmap and softens it when upscaling.

Without any of --svg-scale, --svg-width or --svg-height, the SVG renders at its intrinsic size (from its width/height or viewBox).

For example, --svg-scale 2 renders a 100x50 SVG at 200x100.

Conflicts with --svg-width and --svg-height. Only applies to .svg and .svgz files; raster images in the same batch are not affected."#})
                .value_parser(value_parser!(f32))
                .conflicts_with_all(["svg-width", "svg-height"]),
            arg!(--"svg-width" <PIXELS> "Target width in pixels for SVG input(s); rendering is lossless at any size.")
                .long_help(indoc! {r#"Target width in pixels when rendering SVG input(s).

SVG is rendered straight from the vector data at the requested size, so enlarging is lossless: edges, gradients and text stay exactly as sharp as at the original size. This differs from --resize on a raster image, which resamples an already-rendered bitmap and softens it when upscaling.

When provided without --svg-height, the height is derived while keeping the aspect ratio of the SVG. Conflicts with --svg-scale. Only applies to .svg and .svgz files."#})
                .value_parser(value_parser!(u32).range(1..)),
            arg!(--"svg-height" <PIXELS> "Target height in pixels for SVG input(s); rendering is lossless at any size.")
                .long_help(indoc! {r#"Target height in pixels when rendering SVG input(s).

SVG is rendered straight from the vector data at the requested size, so enlarging is lossless: edges, gradients and text stay exactly as sharp as at the original size. This differs from --resize on a raster image, which resamples an already-rendered bitmap and softens it when upscaling.

When provided without --svg-width, the width is derived while keeping the aspect ratio of the SVG. Conflicts with --svg-scale. Only applies to .svg and .svgz files."#})
                .value_parser(value_parser!(u32).range(1..)),
        ]);

        cmd.preprocessors()
    }
}

pub trait CommonArgs {
    fn common_args(self) -> Self;
}

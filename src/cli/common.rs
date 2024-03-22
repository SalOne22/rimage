use std::path::PathBuf;

use clap::{arg, value_parser, Command};
use indoc::indoc;

use super::{preprocessors::Preprocessors, utils::threads};

impl CommonArgs for Command {
    fn common_args(self) -> Self {
        self
        .next_help_heading("General")
        .arg(
            arg!(files: <FILES> ... "Input file(s) to process.")
                .long_help(indoc! {r#"Input file(s) to process.

                If the file path contains spaces, enclose the path with double quotation marks on both sides."#})
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-d --directory <DIR> "The directory to write output file(s) to.")
                .long_help(indoc! {r#"The directory to write output file(s) to.

                Output files will be written without preserving the folder structure unless the --recursive flag is used."#})
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-r --recursive "Preserves the folder structure when writing output file(s).")
                .long_help(indoc! {r#"Preserves the folder structure when writing output file(s).

                This option should be used in conjunction with the --directory option."#})
                .requires("directory"),
        )
        .arg(
            arg!(-s --suffix [SUFFIX] "Adds the '@suffix' to the names of output file(s).")
                .long_help(indoc! {r#"Adds the '@suffix' to the names of output file(s).

                When '2x' is provided as the value, the resulting files will be renamed with the '@2x' suffix.
                For example, a file named 'file.jpeg' will become 'file@2x.jpeg'.

                If no suffix is provided, the default updated suffix '@updated' will be added to the resulting files."#})
                .default_missing_value("updated"),
        )
        .arg(
            arg!(-b --backup "Adds the '@backup' to the names of input file(s).")
        )
        .arg(
            arg!(-t --threads <NUM> "The number of threads for concurrent processing.")
                .long_help(indoc! {r#"The number of threads for concurrent processing.

                Usage of multiple threads can speed up the execution of tasks, especially on multi-core processors.
                By default, the number of available threads is utilized."#})
                .value_parser(value_parser!(u8).range(1..=threads::num_threads() as i64)),
        )
        .preprocessors()
    }
}

pub trait CommonArgs {
    fn common_args(self) -> Self;
}

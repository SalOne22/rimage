use std::path::PathBuf;

use clap::{arg, command, value_parser, Command};
use indoc::indoc;

use crate::utils::threads;

pub fn cli() -> Command {
    command!()
        .arg_required_else_help(true)
        .arg(
            arg!([FILES] ... "Input file(s) to process")
                .long_help(indoc! {"Input file(s) to process

                If the file path contains spaces, enclose the path with double quotation marks on both sides."})
                .value_parser(value_parser!(PathBuf))
                .global(true)
                .last(true),
        )
        .arg(
            arg!(-d --directory <DIR> "The directory to write output file(s) to")
                .long_help(indoc! {"The directory to write output file(s) to

                Output files will be written without preserving the folder structure unless the --recursive flag is used."})
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-r --recursive "Preserves the folder structure when writing output file(s)")
                .long_help(indoc! {"Preserves the folder structure when writing output file(s).

                This option should be used in conjunction with the --directory option."})
                .requires("directory"),
        )
        .arg(
            arg!(-s --suffix [SUFFIX] "Adds the '@suffix' to the names of output file(s)")
                .long_help(indoc! {"Adds the '@suffix' to the names of output file(s)

                When '2x' is provided as the value, the resulting files will be renamed with the '@2x' suffix.
                For example, a file named 'file.jpeg' will become 'file@2x.jpeg'.

                If no suffix is provided, the default updated suffix '@updated' will be added to the resulting files."})
                .default_missing_value("updated"),
        )
        .arg(
            arg!(-b --backup "Adds the '@backup' to the names of input file(s)")
        )
        .arg(
            arg!(-t --threads <NUM> "The number of threads for concurrent processing")
                .long_help(indoc! {"The number of threads for concurrent processing

                Usage of multiple threads can speed up the execution of tasks, especially on multi-core processors.
                By default, the number of available threads is utilized"})
                .value_parser(value_parser!(u8).range(1..=threads::num_threads() as i64)),
        )
}

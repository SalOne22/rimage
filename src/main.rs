use std::{
    fs::{self, File},
    path::PathBuf,
};

use cli::{
    cli,
    pipeline::{decode, operations},
    utils::paths::{collect_files, get_paths},
};
use rayon::prelude::*;
use rimage::operations::icc::ApplySRGB;
use zune_core::colorspace::ColorSpace;
use zune_image::{core_filters::colorspace::ColorspaceConv, image::Image, pipelines::Pipeline};
use zune_imageprocs::auto_orient::AutoOrient;

use crate::cli::pipeline::encoder;

mod cli;

macro_rules! handle_error {
    ( $path:expr, $e:expr ) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                log::error!("{:?}: {e}", $path);
                return;
            }
        }
    };
}

fn main() {
    pretty_env_logger::init();

    let matches = cli().get_matches_from(
        #[cfg(not(windows))]
        {
            std::env::args()
        },
        #[cfg(windows)]
        {
            std::env::args().map(|mut arg| {
                if let Some(s) = arg.strip_suffix("\"") {
                    arg = s.to_string();
                }

                arg
            })
        },
    );

    match matches.subcommand() {
        Some((subcommand, matches)) => {
            if let Some(threads) = matches.get_one::<u8>("threads") {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(*threads as usize)
                    .build_global()
                    .unwrap();
            }

            let files = collect_files(
                matches
                    .get_many::<PathBuf>("files")
                    .expect("`files` is required")
                    .collect::<Vec<_>>()
                    .as_ref(),
            );

            let out_dir = matches.get_one::<PathBuf>("directory").cloned();

            let recursive = matches.get_flag("recursive");
            let backup = matches.get_flag("backup");

            let suffix = matches.get_one::<String>("suffix").cloned();

            get_paths(files, out_dir, suffix, recursive).for_each(|(input, mut output)| {
                let mut pipeline = Pipeline::<Image>::new();

                let img = handle_error!(input, decode(&input));

                let mut available_encoder = handle_error!(input, encoder(subcommand, matches));
                output.set_extension(available_encoder.to_extension());

                pipeline.chain_operations(Box::new(AutoOrient));
                pipeline.chain_operations(Box::new(ApplySRGB));

                operations(matches, &img)
                    .into_iter()
                    .for_each(|(_, operations)| match operations.name() {
                        "quantize" => {
                            pipeline
                                .chain_operations(Box::new(ColorspaceConv::new(ColorSpace::RGBA)));
                            pipeline.chain_operations(operations);
                        }
                        _ => {
                            pipeline.chain_operations(operations);
                        }
                    });

                pipeline.chain_decoder(img);

                handle_error!(input, pipeline.advance_to_end());

                if backup {
                    handle_error!(
                        input,
                        fs::rename(
                            &input,
                            format!(
                                "{}@backup.{}",
                                input.file_stem().unwrap().to_str().unwrap(),
                                input.extension().unwrap().to_str().unwrap()
                            ),
                        )
                    );
                }

                handle_error!(output, fs::create_dir_all(output.parent().unwrap()));
                let output_file = handle_error!(output, File::create(&output));

                handle_error!(
                    output,
                    available_encoder.encode(&pipeline.images()[0], output_file)
                );
            });
        }
        None => unreachable!(),
    }
}

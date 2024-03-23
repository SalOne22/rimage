use std::{fs, path::PathBuf};

use cli::{
    cli,
    pipeline::{decode, encoder, operations},
    utils::paths::{collect_files, get_paths},
};
use rayon::prelude::*;
use zune_core::colorspace::ColorSpace;
use zune_image::{
    core_filters::colorspace::ColorspaceConv, image::Image, metadata::AlphaState,
    pipelines::Pipeline,
};
use zune_imageprocs::{auto_orient::AutoOrient, premul_alpha::PremultiplyAlpha};

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

                let (encoder, ext) = handle_error!(input, encoder(subcommand, matches));
                output.set_extension(ext);

                pipeline.add_operation(Box::new(AutoOrient));

                operations(matches, &img)
                    .into_iter()
                    .for_each(|(_, operations)| match operations.name() {
                        "fast resize" => {
                            pipeline.add_operation(Box::new(PremultiplyAlpha::new(
                                AlphaState::PreMultiplied,
                            )));

                            pipeline.add_operation(operations);

                            pipeline.add_operation(Box::new(PremultiplyAlpha::new(
                                AlphaState::NonPreMultiplied,
                            )));
                        }
                        "quantize" => {
                            pipeline.add_operation(Box::new(ColorspaceConv::new(ColorSpace::RGBA)));
                            pipeline.add_operation(operations);
                        }
                        _ => {
                            pipeline.add_operation(operations);
                        }
                    });

                pipeline.add_decoder(img);
                pipeline.add_encoder(encoder);

                handle_error!(input, pipeline.advance_to_end());

                let data = pipeline.get_results()[0].data();

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
                handle_error!(output, fs::write(&output, data));
            });
        }
        None => unreachable!(),
    }
}

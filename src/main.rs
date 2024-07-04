use std::{
    fs::{self, File},
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use cli::{
    cli,
    pipeline::{decode, operations},
    utils::paths::{collect_files, get_paths},
};
use console::{style, Term};
use indicatif::{
    DecimalBytes, MultiProgress, ParallelProgressIterator, ProgressBar, ProgressDrawTarget,
    ProgressStyle,
};
use indicatif_log_bridge::LogWrapper;
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
                log::error!("{}: {e}", $path.display());
                return;
            }
        }
    };
}

struct Result {
    output: PathBuf,
    input_size: u64,
    output_size: u64,
}

fn main() {
    let logger = pretty_env_logger::formatted_builder()
        .parse_default_env()
        .build();

    let multi = MultiProgress::new();
    let sty_main = ProgressStyle::with_template("{bar:40.green/yellow} {pos:>4}/{len:4}").unwrap();
    let sty_aux_decode = ProgressStyle::with_template("{spinner:.blue} {msg}").unwrap();
    let sty_aux_operations = ProgressStyle::with_template("{spinner:.yellow} {msg}").unwrap();
    let sty_aux_encode = ProgressStyle::with_template("{spinner:.green} {msg}").unwrap();

    LogWrapper::new(multi.clone(), logger).try_init().unwrap();

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

    let results: Arc<Mutex<Vec<Result>>> = Arc::new(Mutex::new(vec![]));

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
            let quiet = matches.get_flag("quiet");
            let no_progress = matches.get_flag("no-progress");

            let suffix = matches.get_one::<String>("suffix").cloned();

            if quiet || no_progress {
                multi.set_draw_target(ProgressDrawTarget::hidden());
            }

            let pb_main = multi.add(ProgressBar::new(
                files.iter().filter(|f| f.is_file()).count() as u64,
            ));
            pb_main.set_style(sty_main);

            get_paths(files, out_dir, suffix, recursive)
                .progress_with(pb_main)
                .for_each(|(input, mut output)| {
                    let pb = multi.add(ProgressBar::new_spinner());
                    pb.set_style(sty_aux_decode.clone());
                    pb.set_message(format!("{}", input.display()));
                    pb.enable_steady_tick(Duration::from_millis(100));

                    let mut pipeline = Pipeline::<Image>::new();

                    let input_size = handle_error!(input, input.metadata()).len();

                    let img = handle_error!(input, decode(&input));

                    pb.set_style(sty_aux_operations.clone());

                    let mut available_encoder = handle_error!(input, encoder(subcommand, matches));
                    output.set_extension(available_encoder.to_extension());

                    pipeline.chain_operations(Box::new(AutoOrient));
                    pipeline.chain_operations(Box::new(ApplySRGB));

                    operations(matches, &img)
                        .into_iter()
                        .for_each(|(_, operations)| match operations.name() {
                            "quantize" => {
                                pipeline.chain_operations(Box::new(ColorspaceConv::new(
                                    ColorSpace::RGBA,
                                )));
                                pipeline.chain_operations(operations);
                            }
                            _ => {
                                pipeline.chain_operations(operations);
                            }
                        });

                    pipeline.chain_decoder(img);

                    handle_error!(input, pipeline.advance_to_end());

                    pb.set_style(sty_aux_encode.clone());

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

                    let output_size = handle_error!(output, output.metadata()).len();

                    let mut results = results.lock().unwrap();

                    results.push(Result {
                        output,
                        input_size,
                        output_size,
                    });

                    pb.finish_and_clear();
                });

            let mut results = results.lock().unwrap();

            results.sort_by(|a, b| b.output_size.cmp(&a.output_size));

            let path_width = results
                .iter()
                .map(|r| r.output.display().to_string().len())
                .max()
                .unwrap();

            if !quiet {
                let term = Term::stdout();

                term.write_line(&format!(
                    "{:<path_width$} {}",
                    style("File").bold(),
                    style("Size").bold(),
                ))
                .unwrap();

                for result in results.iter() {
                    let difference = (result.output_size as f64 / result.input_size as f64) * 100.0;

                    term.write_line(&format!(
                        "{:<path_width$} {} > {} {}",
                        result.output.display(),
                        style(DecimalBytes(result.input_size)).blue(),
                        style(DecimalBytes(result.output_size)).blue(),
                        if difference > 100.0 {
                            style(format!("{:.2}%", difference - 100.0)).red()
                        } else {
                            style(format!("{:.2}%", difference - 100.0)).green()
                        },
                    ))
                    .unwrap();
                }
            }
        }
        None => unreachable!(),
    }
}

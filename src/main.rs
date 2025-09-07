use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
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
use little_exif::metadata::Metadata as ExifMetadata;
use rayon::prelude::*;
use rimage::operations::icc::ApplySRGB;
use serde::{Deserialize, Serialize};
use zune_core::{bit_depth::BitDepth, colorspace::ColorSpace};
use zune_image::{
    core_filters::{colorspace::ColorspaceConv, depth::Depth},
    image::Image,
    pipelines::Pipeline,
};
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

const SUPPORTS_EXIF: &[&str] = &["mozjpeg", "oxipng", "png", "jpeg", "jpegxl", "tiff", "webp"];
const SUPPORTS_ICC: &[&str] = &["mozjpeg", "oxipng"];

struct Result {
    output: PathBuf,
    input_size: u64,
    output_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Metadata {
    #[serde(rename = "inputSize")]
    input_size: u64,
    #[serde(rename = "outputSize")]
    output_size: u64,
    #[serde(rename = "totalImages")]
    total_images: usize,
    #[serde(rename = "compressionRatio")]
    compression_ratio: f64,
    #[serde(rename = "spaceSaved")]
    space_saved: i64,
    timestamp: u64,
    images: Vec<ImageMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ImageMetadata {
    // File paths
    input: PathBuf,
    output: PathBuf,

    // File information
    #[serde(rename = "inputSize")]
    input_size: u64,
    #[serde(rename = "outputSize")]
    output_size: u64,
    #[serde(rename = "compressionRatio")]
    compression_ratio: f64,
    #[serde(rename = "spaceSaved")]
    space_saved: i64,

    // Image properties
    width: u32,
    height: u32,
    #[serde(rename = "pixelCount")]
    pixel_count: u64,
    #[serde(rename = "aspectRatio")]
    aspect_ratio: f64,

    // zune-image specific properties
    #[serde(rename = "bitDepth")]
    bit_depth: String,
    #[serde(rename = "colorSpace")]
    color_space: String,
    #[serde(rename = "hasAlpha")]
    has_alpha: bool,
    #[serde(rename = "isAnimated")]
    is_animated: bool,
    #[serde(rename = "frameCount")]
    frame_count: usize,
    channels: usize,

    // Format information
    #[serde(rename = "inputFormat")]
    input_format: Option<String>,
    #[serde(rename = "outputFormat")]
    output_format: String,

    // Processing information
    #[serde(rename = "processedAt")]
    processed_at: u64,
    #[serde(rename = "processingTimeMs")]
    processing_time_ms: u128,

    // File timestamps
    #[serde(rename = "inputModified")]
    input_modified: Option<u64>,
    #[serde(rename = "outputCreated")]
    output_created: u64,
}

fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

fn get_file_modified_time(path: &Path) -> Option<u64> {
    fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn bit_depth_to_string(depth: &BitDepth) -> String {
    match depth {
        BitDepth::Eight => "8-bit".to_string(),
        BitDepth::Sixteen => "16-bit".to_string(),
        BitDepth::Float32 => "32-bit float".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn colorspace_to_string(colorspace: &ColorSpace) -> String {
    match colorspace {
        ColorSpace::RGB => "RGB".to_string(),
        ColorSpace::RGBA => "RGBA".to_string(),
        ColorSpace::Luma => "Grayscale".to_string(),
        ColorSpace::LumaA => "Grayscale with Alpha".to_string(),
        ColorSpace::YCbCr => "YCbCr".to_string(),
        ColorSpace::YCCK => "YCCK".to_string(),
        ColorSpace::CMYK => "CMYK".to_string(),
        ColorSpace::BGR => "BGR".to_string(),
        ColorSpace::BGRA => "BGRA".to_string(),
        ColorSpace::HSL => "HSL".to_string(),
        ColorSpace::HSV => "HSV".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn main() {
    let logger = pretty_env_logger::formatted_builder()
        .parse_default_env()
        .build();
    let level = logger.filter();

    let multi = MultiProgress::new();
    let sty_main = ProgressStyle::with_template("{bar:40.green/yellow} {pos:>4}/{len:4}")
        .unwrap()
        .progress_chars("▬▬▬");
    let sty_aux_decode = ProgressStyle::with_template("{spinner:.blue} {msg}").unwrap();
    let sty_aux_operations = ProgressStyle::with_template("{spinner:.yellow} {msg}").unwrap();
    let sty_aux_encode = ProgressStyle::with_template("{spinner:.green} {msg}").unwrap();

    LogWrapper::new(multi.clone(), logger).try_init().unwrap();
    log::set_max_level(level);

    let matches = cli().get_matches_from(
        #[cfg(not(windows))]
        {
            std::env::args()
        },
        #[cfg(windows)]
        {
            std::env::args().map(|arg| {
                arg.replace("\\", "/")
                    .replace("//", "/")
                    .trim_matches(['\\', '/', '\n', '\r', '"', '\'', ' ', '\t'])
                    .to_string()
            })
        },
    );

    let results: Arc<Mutex<Vec<Result>>> = Arc::new(Mutex::new(vec![]));
    let metadata: Arc<Mutex<Option<Metadata>>> = Arc::new(Mutex::new(None));

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

            let file_count = files.iter().filter(|f| f.is_file()).count() as u64;

            let out_dir = matches.get_one::<PathBuf>("directory").cloned();

            let recursive = matches.get_flag("recursive");
            let backup = matches.get_flag("backup");
            let strip_metadata = matches.get_flag("strip");
            let quiet = matches.get_flag("quiet");
            let no_progress = matches.get_flag("no-progress");
            let output_metadata = matches.contains_id("metadata");
            let metadata_path = matches
                .get_one::<PathBuf>("metadata")
                .cloned()
                .unwrap_or(PathBuf::from("metadata.json"));

            let suffix = matches.get_one::<String>("suffix").cloned();

            if quiet || no_progress {
                multi.set_draw_target(ProgressDrawTarget::hidden());
            }

            let pb_main = multi.add(ProgressBar::new(file_count));
            pb_main.set_style(sty_main);
            if file_count <= 1 {
                pb_main.set_draw_target(ProgressDrawTarget::hidden());
            }

            get_paths(files, out_dir, suffix, recursive)
                .progress_with(pb_main)
                .for_each(|(input, mut output)| {
                    let image_start_time = std::time::Instant::now();

                    let pb = multi.add(ProgressBar::new_spinner());
                    pb.set_style(sty_aux_decode.clone());
                    pb.set_message(format!("{}", input.display()));
                    pb.enable_steady_tick(Duration::from_millis(100));

                    let mut pipeline = Pipeline::<Image>::new();

                    let input_size = handle_error!(input, input.metadata()).len();
                    let input_format = get_file_extension(&input);
                    let input_modified = get_file_modified_time(&input);

                    let img = handle_error!(input, decode(&input));
                    let exif_metadata = ExifMetadata::new_from_path(&input).ok();

                    pb.set_style(sty_aux_operations.clone());

                    // Extract zune-image properties
                    let (w, h) = img.dimensions();
                    let pixel_count = (w as u64) * (h as u64);
                    let aspect_ratio = w as f64 / h as f64;
                    let colorspace = img.colorspace();
                    let is_animated = img.is_animated();
                    let frame_count = img.frames_len();
                    let has_alpha = colorspace.has_alpha();
                    let channels = colorspace.num_components();

                    let original_bit_depth = img.depth();

                    let mut available_encoder = handle_error!(input, encoder(subcommand, matches));
                    let output_format = available_encoder.to_extension().to_string();

                    if let Some(ext) = output.extension() {
                        output.set_extension({
                            let mut os_str = ext.to_os_string();
                            os_str.push(".");
                            os_str.push(&output_format);
                            os_str
                        });
                    } else {
                        output.set_extension(&output_format);
                    }

                    pipeline.chain_operations(Box::new(Depth::new(BitDepth::Eight)));
                    pipeline.chain_operations(Box::new(ColorspaceConv::new(ColorSpace::RGBA)));

                    if strip_metadata || !SUPPORTS_EXIF.contains(&subcommand) {
                        pipeline.chain_operations(Box::new(AutoOrient));
                    }

                    if strip_metadata || !SUPPORTS_ICC.contains(&subcommand) {
                        pipeline.chain_operations(Box::new(ApplySRGB));
                    }

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

                    exif_metadata
                        .and_then(|mut metadata| {
                            if strip_metadata {
                                metadata.reduce_to_a_minimum();
                            }
                            metadata.write_to_file(&output).ok()
                        })
                        .or_else(|| {
                            log::info!("No metadata found, skipping...");
                            None
                        });

                    let output_size = handle_error!(output, output.metadata()).len();
                    let processing_time = image_start_time.elapsed().as_millis();
                    let compression_ratio = output_size as f64 / input_size as f64;
                    let space_saved = input_size as i64 - output_size as i64;
                    let processed_at = get_current_timestamp();
                    let output_created = get_current_timestamp();

                    let mut results = results.lock().unwrap();
                    let mut metadata = metadata.lock().unwrap();

                    let absolute_input_path = fs::canonicalize(&input).unwrap();
                    let absolute_output_path = fs::canonicalize(&output).unwrap();

                    results.push(Result {
                        output,
                        input_size,
                        output_size,
                    });

                    let metadata = metadata.get_or_insert(Metadata {
                        input_size: 0,
                        output_size: 0,
                        total_images: 0,
                        compression_ratio: 0.0,
                        space_saved: 0,
                        timestamp: get_current_timestamp(),
                        images: vec![],
                    });

                    metadata.input_size += input_size;
                    metadata.output_size += output_size;
                    metadata.total_images += 1;
                    metadata.space_saved += space_saved;

                    metadata.images.push(ImageMetadata {
                        input: absolute_input_path,
                        output: absolute_output_path,
                        input_size,
                        output_size,
                        compression_ratio,
                        space_saved,
                        width: w as u32,
                        height: h as u32,
                        pixel_count,
                        aspect_ratio,
                        bit_depth: bit_depth_to_string(&original_bit_depth),
                        color_space: colorspace_to_string(&colorspace),
                        has_alpha,
                        is_animated,
                        frame_count,
                        channels,
                        input_format,
                        output_format,
                        processed_at,
                        processing_time_ms: processing_time,
                        input_modified,
                        output_created,
                    });

                    pb.finish_and_clear();
                });

            let mut results = results.lock().unwrap();
            let mut metadata = metadata.lock().unwrap();

            // Update final metadata calculations
            if let Some(ref mut meta) = metadata.as_mut() {
                meta.compression_ratio = if meta.input_size > 0 {
                    meta.output_size as f64 / meta.input_size as f64
                } else {
                    0.0
                };
            }

            results.sort_by(|a, b| b.output_size.cmp(&a.output_size));

            let path_width = results
                .iter()
                .map(|r| r.output.display().to_string().len())
                .max()
                .unwrap_or(0);

            if !quiet {
                let term = Term::stdout();

                if results.len() > 1 {
                    term.write_line(&format!(
                        "{:<path_width$} {}",
                        style("File").bold(),
                        style("Size").bold(),
                    ))
                    .unwrap();

                    for result in results.iter() {
                        let difference =
                            (result.output_size as f64 / result.input_size as f64) * 100.0;

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

                let total_input_size = results.iter().map(|r| r.input_size).sum::<u64>();
                let total_output_size = results.iter().map(|r| r.output_size).sum::<u64>();

                let difference = (total_output_size as f64 / total_input_size as f64) * 100.0;

                term.write_line(&format!(
                    "Total: {} > {} {}",
                    style(DecimalBytes(total_input_size)).blue(),
                    style(DecimalBytes(total_output_size)).blue(),
                    if difference > 100.0 {
                        style(format!("{:.2}%", difference - 100.0)).red()
                    } else {
                        style(format!("{:.2}%", difference - 100.0)).green()
                    },
                ))
                .unwrap();
            }

            if output_metadata {
                if let Some(metadata) = metadata.as_ref() {
                    let json = serde_json::to_string_pretty(metadata).unwrap();
                    fs::write(metadata_path, json).unwrap();
                }
            }
        }
        None => unreachable!(),
    }
}

use std::{fs, io, path, process};

use clap::Parser;
use console::{Emoji, Style};
use glob::glob;
use indicatif::{DecimalBytes, MultiProgress};
use log::{error, info};
#[cfg(target_env = "msvc")]
use mimalloc::MiMalloc;
use rayon::prelude::*;
use rimage::{error::ConfigError, image::Codec, optimize, Config, Decoder};
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

use crate::{progress_bar::create_spinner, utils::common_path};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
#[cfg(target_env = "msvc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod progress_bar;
/// Some utils functions
mod utils;

#[derive(Parser)]
#[command(author, about, version, long_about = None)]
struct Args {
    /// Input file(s)
    input: Vec<path::PathBuf>,
    /// Output directory
    #[arg(short, long, value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    output: Option<path::PathBuf>,
    /// Quality of the output image (0-100)
    #[arg(short, long, default_value = "75")]
    quality: f32,
    /// Output format of the output image
    #[arg(short, long, default_value = "jpg")]
    format: Codec,
    /// Print image info
    #[arg(short, long)]
    info: bool,
    /// Prefix of the output file
    #[arg(short, long)]
    suffix: Option<String>,
    /// Number of threads to use
    #[arg(short, long)]
    threads: Option<usize>,
    /// Target quantization quality from 0 to 100
    #[arg(long)]
    quantization: Option<u8>,
    /// Target quantization dithering strength from 0 to 1.0
    #[arg(long)]
    dithering: Option<f32>,
    /// Target width in pixels of the output image
    #[arg(long)]
    width: Option<usize>,
    /// Target height in pixels of the output image
    #[arg(long)]
    height: Option<usize>,
    /// Resize filter to use.
    /// Supported filters: point, triangle, catmull-rom, mitchell, lanczos3
    #[arg(long)]
    filter: Option<rimage::image::ResizeType>,
}

fn main() {
    pretty_env_logger::init();

    let args = get_args();
    let m = MultiProgress::new();

    let conf = get_config(&args).unwrap_or_else(|err| {
        error!("{err}");
        process::exit(1);
    });

    let common_path = common_path(&args.input);

    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads.unwrap_or(0))
        .build_global()
        .unwrap_or_else(|err| {
            error!("{err}");
            process::exit(1);
        });

    info!("Using {} threads", rayon::current_num_threads());
    info!("Using config: {:?}", conf);
    info!("Found common path: {:?}", common_path);

    if args.info {
        get_info(args, common_path);
        process::exit(0);
    }

    bulk_optimize(args, &conf, common_path, &m);
}

fn get_args() -> Args {
    let mut args = Args::parse();
    // Get all files from stdin if no input is given
    if args.input.is_empty() {
        info!("Reading input from stdin");
        args.input = io::stdin()
            .lines()
            .map(|res| {
                let input_file = res.unwrap();
                path::PathBuf::from(input_file.trim())
            })
            .collect();
        info!("{} files read from stdin", args.input.len());
    } else {
        // Otherwise use glob pattern
        let mut new_input = vec![];

        for input_path in args.input {
            let input_path = input_path.to_str().unwrap();
            for path in glob(input_path).expect("Failed to read glob pattern") {
                let path = path.unwrap();
                new_input.push(path)
            }
        }

        args.input = new_input;
    }

    args
}

fn get_info(args: Args, common_path: Option<path::PathBuf>) {
    for path in args.input {
        let d = match Decoder::from_path(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                continue;
            }
        };

        let img = match d.decode() {
            Ok(img) => img,
            Err(e) => {
                error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                continue;
            }
        };

        println!("Full path: {:?}", &path);

        if let Some(common_path) = &common_path {
            println!(
                "Relative path: {:?}",
                &path.strip_prefix(common_path.parent().unwrap()).unwrap()
            );
        }

        println!("{:?}", &path.file_name().unwrap());
        println!("Size: {:?}", img.size());
        println!("Data length: {:?}", img.data().len());
        println!();
    }
}

fn get_config(args: &Args) -> Result<Config, ConfigError> {
    let mut conf = Config::builder(args.format);

    conf.quality(args.quality);

    if let Some(width) = args.width {
        conf.target_width(width);
    }
    if let Some(height) = args.height {
        conf.target_height(height);
    }
    if let Some(filter) = args.filter {
        conf.resize_type(filter);
    }
    if let Some(quantization) = args.quantization {
        conf.quantization_quality(quantization);
    }
    if let Some(dithering) = args.dithering {
        conf.dithering_level(dithering);
    }

    conf.build()
}

fn bulk_optimize(args: Args, conf: &Config, common_path: Option<path::PathBuf>, m: &MultiProgress) {
    args.input.into_par_iter().for_each(|path| {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let spinner = create_spinner(file_name.to_owned(), m);

        let file_size_before_optimization = fs::metadata(&path).unwrap().len();
        let file_size_after_optimization;

        info!("Decoding {}", file_name);

        let mut new_path = path.clone();

        if let Some(destination_dir) = &args.output {
            let file_name = path::Path::new(new_path.file_name().unwrap());

            let relative_path = if let Some(common_path) = &common_path {
                new_path.strip_prefix(common_path).unwrap_or(file_name)
            } else {
                file_name
            };

            new_path = destination_dir.join(relative_path);
        }

        let ext = args.format.to_string();
        let suffix = args.suffix.clone().unwrap_or_default();

        new_path.set_file_name(format!(
            "{}{}",
            path.file_stem().unwrap().to_str().unwrap(),
            suffix,
        ));
        new_path.set_extension(ext);

        match fs::create_dir_all(new_path.parent().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                error!("{} {e}", file_name);
                return;
            }
        }

        match fs::write(
            &new_path,
            match optimize(&path, conf) {
                Ok(data) => {
                    file_size_after_optimization = data.len() as u64;
                    data
                }
                Err(e) => {
                    error!("{} {e}", file_name);
                    return;
                }
            },
        ) {
            Ok(_) => (),
            Err(e) => {
                error!("{} {e}", file_name);
                return;
            }
        };
        info!("Saved to {:?}", new_path);

        let diff = file_size_after_optimization as f64 / file_size_before_optimization as f64;
        let abs_percent = diff.abs() * 100.0;
        let percent = if diff > 1.0 {
            abs_percent - 100.0
        } else {
            100.0 - abs_percent
        };

        let cyan = Style::new().cyan();
        let red = Style::new().red();
        let green = Style::new().green();

        spinner.set_prefix(format!("{}", Emoji("✅", "Done")));
        spinner.finish_with_message(format!(
            "{file_name} completed {} -> {} {}",
            cyan.apply_to(DecimalBytes(file_size_before_optimization)),
            cyan.apply_to(DecimalBytes(file_size_after_optimization)),
            if percent > 100.0 {
                red.apply_to(format!("{} {:.1}%", Emoji("🔺", "^"), percent))
            } else {
                green.apply_to(format!("{} {:.1}%", Emoji("🔻", "v"), percent))
            }
        ));
    });
}

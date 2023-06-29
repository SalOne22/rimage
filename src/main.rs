use std::{fs, io, path, process, sync::Arc};

use clap::Parser;
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
#[cfg(target_env = "msvc")]
use mimalloc::MiMalloc;
use rimage::{image::OutputFormat, optimize, Config, Decoder};
use threadpool::ThreadPool;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

use crate::utils::common_path;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
#[cfg(target_env = "msvc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
    format: OutputFormat,
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
    let pool = ThreadPool::new(args.threads.unwrap_or(num_cpus::get()));
    let pb = Arc::new(ProgressBar::new(args.input.len() as u64));

    let conf = Arc::new(get_config(&args));

    let common_path = common_path(&args.input);

    info!("Using {} threads", pool.max_count());
    info!("Using config: {:?}", conf);
    info!("Found common path: {:?}", common_path);

    pb.set_style(
        ProgressStyle::with_template("{bar:40.green/blue}  {pos}/{len}")
            .unwrap()
            .progress_chars("##-"),
    );
    pb.set_position(0);

    if args.info {
        get_info(args, common_path);
        process::exit(0);
    }

    bulk_optimize(args, &conf, common_path, &pb, &pool);

    pool.join();
    pb.finish();
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
        let file = match fs::File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                continue;
            }
        };

        let d = Decoder::new(&path, file);

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

fn get_config(args: &Args) -> Config {
    let mut conf = Config::build(args.quality, args.format).unwrap_or_else(|e| {
        error!("{e}");
        process::exit(1);
    });

    conf.set_target_width(args.width).unwrap_or_else(|e| {
        error!("{e}");
        process::exit(1);
    });

    conf.set_target_height(args.height).unwrap_or_else(|e| {
        error!("{e}");
        process::exit(1);
    });

    conf.resize_type = args.filter.clone();

    conf.set_quantization_quality(args.quantization)
        .unwrap_or_else(|e| {
            error!("{e}");
            process::exit(1);
        });

    conf.set_dithering_level(args.dithering)
        .unwrap_or_else(|e| {
            error!("{e}");
            process::exit(1);
        });

    conf
}

fn bulk_optimize(
    args: Args,
    conf: &Config,
    common_path: Option<path::PathBuf>,
    pb: &ProgressBar,
    pool: &ThreadPool,
) {
    for path in args.input {
        let pb = pb.clone();
        let conf = conf.clone();
        let suffix = args.suffix.clone();
        let common_path = common_path.clone();
        let destination_dir = args.output.clone();

        pool.execute(move || {
            info!("Decoding {}", &path.file_name().unwrap().to_str().unwrap());
            pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());

            let mut new_path = path.clone();

            if let Some(destination_dir) = &destination_dir {
                let file_name = path::Path::new(new_path.file_name().unwrap());

                let relative_path = if let Some(common_path) = &common_path {
                    new_path.strip_prefix(common_path).unwrap_or(file_name)
                } else {
                    file_name
                };

                new_path = destination_dir.join(relative_path);
            }

            let ext = args.format.to_string();
            let suffix = suffix.clone().unwrap_or_default();

            new_path.set_file_name(format!(
                "{}{}",
                path.file_stem().unwrap().to_str().unwrap(),
                suffix,
            ));
            new_path.set_extension(ext);

            match fs::create_dir_all(new_path.parent().unwrap()) {
                Ok(_) => (),
                Err(e) => {
                    error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                }
            }

            match fs::write(
                &new_path,
                match optimize(&path, &conf) {
                    Ok(data) => data,
                    Err(e) => {
                        error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                        return;
                    }
                },
            ) {
                Ok(_) => (),
                Err(e) => {
                    error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                    return;
                }
            };
            info!("{} done", &path.file_name().unwrap().to_str().unwrap());
            info!("Saved to {:?}", new_path);
            pb.inc(1);
        });
    }
}

use std::{fs, io, path, process, sync::Arc};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
#[cfg(target_env = "msvc")]
use mimalloc::MiMalloc;
use rimage::{image::OutputFormat, Config, Decoder, Encoder};
use threadpool::ThreadPool;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
#[cfg(target_env = "msvc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser)]
#[command(author, about, version, long_about = None)]
struct Args {
    /// Input file(s)
    input: Vec<path::PathBuf>,
    /// Quality of the output image (0-100)
    #[arg(short, long, default_value = "75")]
    quality: f32,
    /// Output format of the output image
    #[arg(short, long, default_value = "jpg")]
    output_format: OutputFormat,
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
    let mut args = Args::parse_from(wild::args_os());
    let pb = Arc::new(ProgressBar::new(args.input.len() as u64));
    let pool = ThreadPool::new(args.threads.unwrap_or(num_cpus::get()));
    info!("Using {} threads", pool.max_count());

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
    }

    let conf = Arc::new(
        Config::build(
            args.quality,
            args.output_format,
            args.width,
            args.height,
            args.filter,
        )
        .unwrap_or_else(|e| {
            error!("{e}");
            process::exit(1);
        }),
    );
    info!("Using config: {:?}", conf);

    pb.set_style(
        ProgressStyle::with_template("{bar:40.green/blue}  {pos}/{len}")
            .unwrap()
            .progress_chars("##-"),
    );
    pb.set_position(0);

    if args.info {
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

            println!("{:?}", &path.file_name().unwrap());
            println!("Size: {:?}", img.size());
            println!("Data length: {:?}", img.data().len());
            println!();
        }

        process::exit(0);
    }

    if args.quantization.is_some() || args.dithering.is_some() {
        let quantization = args.quantization.unwrap_or(100);
        let dithering = args.dithering.unwrap_or(1.0);
        info!(
            "Quantizing to {} with dithering {}",
            quantization, dithering
        );

        for path in args.input {
            let pb = pb.clone();
            let conf = conf.clone();
            let suffix = args.suffix.clone();
            pool.execute(move || {
                pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());
                info!("Decoding {}", &path.file_name().unwrap().to_str().unwrap());

                let file = match fs::File::open(&path) {
                    Ok(file) => file,
                    Err(e) => {
                        error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                        return;
                    }
                };

                info!("read {} bytes", file.metadata().unwrap().len());

                let d = Decoder::new(&path, file);
                let e = Encoder::new(
                    &conf,
                    match d.decode() {
                        Ok(img) => img,
                        Err(e) => {
                            error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                            return;
                        }
                    },
                );

                let mut new_path = path.clone();
                let ext = args.output_format.to_string();
                let suffix = suffix.clone().unwrap_or_default();

                new_path.set_file_name(format!(
                    "{}{}",
                    path.file_stem().unwrap().to_str().unwrap(),
                    suffix,
                ));
                new_path.set_extension(ext);

                match fs::write(
                    new_path,
                    match e.encode_quantized(quantization, dithering) {
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
                pb.inc(1);
            });
        }
        pool.join();
        pb.finish();

        process::exit(0);
    }

    for path in args.input {
        let pb = pb.clone();
        let conf = conf.clone();
        let suffix = args.suffix.clone();
        pool.execute(move || {
            info!("Decoding {}", &path.file_name().unwrap().to_str().unwrap());
            pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());

            let file = match fs::File::open(&path) {
                Ok(file) => file,
                Err(e) => {
                    error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                    return;
                }
            };

            info!("read {} bytes", file.metadata().unwrap().len());

            let d = Decoder::new(&path, file);
            let e = Encoder::new(
                &conf,
                match d.decode() {
                    Ok(img) => img,
                    Err(e) => {
                        error!("{} {e}", &path.file_name().unwrap().to_str().unwrap());
                        return;
                    }
                },
            );

            let mut new_path = path.clone();
            let ext = args.output_format.to_string();
            let suffix = suffix.clone().unwrap_or_default();

            new_path.set_file_name(format!(
                "{}{}",
                path.file_stem().unwrap().to_str().unwrap(),
                suffix,
            ));
            new_path.set_extension(ext);

            match fs::write(
                new_path,
                match e.encode() {
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
            pb.inc(1);
        });
    }
    pool.join();
    pb.finish();
}

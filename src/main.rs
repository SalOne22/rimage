use std::{fs, io, path, process};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rimage::{image::OutputFormat, Config, Decoder, Encoder};
use threadpool::ThreadPool;

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
}

fn main() {
    let mut args = Args::parse_from(wild::args_os());
    let pb = ProgressBar::new(args.input.len() as u64);
    let pool = ThreadPool::new(args.threads.unwrap_or(num_cpus::get()));

    // Get all files from stdin if no input is given
    if args.input.is_empty() {
        args.input = io::stdin()
            .lines()
            .map(|res| {
                let input_file = res.unwrap();
                path::PathBuf::from(input_file.trim())
            })
            .collect();
    }

    let conf = Config::build(args.quality, args.output_format).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    });

    pb.set_style(
        ProgressStyle::with_template("{bar:40.green/blue}  {pos}/{len}")
            .unwrap()
            .progress_chars("##-"),
    );
    pb.set_position(0);

    if args.info {
        for path in args.input {
            let data = match fs::read(&path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!(
                        "{} Error: {e}",
                        &path.file_name().unwrap().to_str().unwrap()
                    );
                    continue;
                }
            };

            let d = Decoder::new(&path, &data);

            let img = match d.decode() {
                Ok(img) => img,
                Err(e) => {
                    eprintln!(
                        "{} Error: {e}",
                        &path.file_name().unwrap().to_str().unwrap()
                    );
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

        for path in args.input {
            let pb = pb.clone();
            let conf = conf.clone();
            let suffix = args.suffix.clone();
            pool.execute(move || {
                pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());

                let data = match fs::read(&path) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!(
                            "{} Error: {e}",
                            &path.file_name().unwrap().to_str().unwrap()
                        );
                        return;
                    }
                };

                let d = Decoder::new(&path, &data);
                let e = Encoder::new(
                    &conf,
                    match d.decode() {
                        Ok(img) => img,
                        Err(e) => {
                            eprintln!("{} Error: {e}", path.file_name().unwrap().to_str().unwrap());
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
                            eprintln!("{} Error: {e}", path.file_name().unwrap().to_str().unwrap());
                            return;
                        }
                    },
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{} Error: {e}", path.file_name().unwrap().to_str().unwrap());
                        return;
                    }
                };
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
            pb.set_message(path.file_name().unwrap().to_str().unwrap().to_owned());

            let data = match fs::read(&path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!(
                        "{} Error: {e}",
                        &path.file_name().unwrap().to_str().unwrap()
                    );
                    return;
                }
            };

            let d = Decoder::new(&path, &data);
            let e = Encoder::new(
                &conf,
                match d.decode() {
                    Ok(img) => img,
                    Err(e) => {
                        eprintln!("{} Error: {e}", path.file_name().unwrap().to_str().unwrap());
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
                        eprintln!("{} Error: {e}", path.file_name().unwrap().to_str().unwrap());
                        return;
                    }
                },
            ) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{} Error: {e}", path.file_name().unwrap().to_str().unwrap());
                    return;
                }
            };
            pb.inc(1);
        });
    }
    pool.join();
    pb.finish();
}

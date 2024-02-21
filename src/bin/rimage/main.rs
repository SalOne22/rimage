use std::{error::Error, path::PathBuf, str::FromStr};

use clap::{arg, value_parser, ArgAction, Command};

use paths::collect_files;
use rimage::config::{Codec, EncoderConfig, QuantizationConfig, ResizeConfig, ResizeType};

mod optimize;
mod paths;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("rimage")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Vladyslav Vladinov <vladinov.dev@gmail.com>")
        .about("Rimage is a tool to convert/optimize/resize images in different formats")
        .arg(
            arg!(<FILES> "Input file(s) to process\n(When the file path contains spaces, double quotation marks should be used\nto wrap the file path on both sides.)")
                .num_args(1..)
                .value_delimiter(None)
                .value_parser(value_parser!(PathBuf)),
        )
        .next_help_heading("General")
        .args([
            arg!(-q --quality <QUALITY> "Optimization image quality, disabled when use Jxl format\n[range: 1 - 100]")
                .value_parser(value_parser!(f32))
                .default_value("75"),
            arg!(-f --codec <CODEC> "Image codec for output\n[possible values: jpg, png, oxipng, jxl, webp, avif]")
                .value_parser(Codec::from_str)
                .default_value("mozjpeg"),
            arg!(-o --output <DIR> "Write output file(s) to <DIR>, if \"-r\" option is not used")
                .value_parser(value_parser!(PathBuf)),
            arg!(-r --recursive "Saves output file(s) preserving folder structure")
                .action(ArgAction::SetTrue),
            arg!(-s --suffix [SUFFIX] "Appends suffix to output file(s) names"),
            arg!(-b --backup "Appends \".backup\" suffix to input file(s) extension")
                .action(ArgAction::SetTrue),
            #[cfg(feature = "parallel")]
            arg!(-t --threads <NUM> "Number of threads to use\n[range: 1 - 16] [default: number of cores]")
                .value_parser(value_parser!(usize)),
        ])
        .next_help_heading("Quantization")
        .args([
            arg!(--quantization [QUALITY] "Enables quantization with optional quality\n[range: 1 - 100] [default: 75]")
                .value_parser(value_parser!(u8).range(..=100))
                .default_missing_value("75"),
            arg!(--dithering [QUALITY] "Enables dithering with optional quality\n[range: 1 - 100] [default: 75]")
                .value_parser(value_parser!(f32))
                .default_missing_value("75")
        ])
        .next_help_heading("Resizing")
        .args([
            arg!(--width <WIDTH> "Resize image with specified width\n[integer only]")
                .value_parser(value_parser!(usize)),
            arg!(--height <HEIGHT> "Resize image with specified height\n[integer only]")
                .value_parser(value_parser!(usize)),
            arg!(--filter <FILTER> "Filter used for image resizing\n[possible values: point, triangle, catrom, mitchell]")
                .value_parser(ResizeType::from_str)
                .default_value("lanczos3")
        ])
        .get_matches();

    let codec = matches.get_one::<Codec>("codec").unwrap();
    let quality = matches.get_one::<f32>("quality").unwrap();

    #[cfg(feature = "parallel")]
    if let Some(threads) = matches.get_one::<usize>("threads") {
        rayon::ThreadPoolBuilder::new()
            .num_threads(*threads)
            .build_global()
            .unwrap();
    }

    let mut quantization_config = QuantizationConfig::new();

    if let Some(quality) = matches.get_one::<u8>("quantization") {
        quantization_config = quantization_config.with_quality(*quality)?
    }

    if let Some(dithering) = matches.get_one::<f32>("dithering") {
        quantization_config = quantization_config.with_dithering(*dithering / 100.0)?
    }

    let resize_filter = matches.get_one::<ResizeType>("filter").unwrap();

    let mut resize_config = ResizeConfig::new(*resize_filter);

    if let Some(width) = matches.get_one::<usize>("width") {
        resize_config = resize_config.with_width(*width);
    }

    if let Some(height) = matches.get_one::<usize>("height") {
        resize_config = resize_config.with_height(*height);
    }

    let mut conf = EncoderConfig::new(*codec).with_quality(*quality)?;

    if matches.get_one::<u8>("quantization").is_some()
        || matches.get_one::<f32>("dithering").is_some()
    {
        conf = conf.with_quantization(quantization_config);
    }

    if matches.get_one::<usize>("width").is_some() || matches.get_one::<usize>("height").is_some() {
        conf = conf.with_resize(resize_config);
    }

    let files = matches
        .get_many::<PathBuf>("FILES")
        .unwrap_or_default()
        .map(|v| v.into())
        .collect();

    let out_dir = matches.get_one::<PathBuf>("output").map(|p| p.into());
    let suffix = matches.get_one::<String>("suffix").map(|p| p.into());
    let recursive = matches.get_one::<bool>("recursive").unwrap_or(&false);
    let backup = matches.get_one::<bool>("backup").unwrap_or(&false);

    optimize::optimize_files(
        paths::get_paths(
            collect_files(files),
            out_dir,
            suffix,
            codec.to_extension(),
            *recursive,
        ),
        conf,
        *backup,
    );

    Ok(())
}

use std::io::{Seek, SeekFrom};
use std::{collections::BTreeMap, fs::File, io::Read, path::Path};

use clap::ArgMatches;
#[cfg(feature = "avif")]
use rimage::codecs::avif::AvifEncoder;
#[cfg(feature = "mozjpeg")]
use rimage::codecs::mozjpeg::MozJpegEncoder;
#[cfg(feature = "oxipng")]
use rimage::codecs::oxipng::OxiPngEncoder;
#[cfg(feature = "svg")]
use rimage::codecs::svg::{SvgDecoder, SvgOptions};
#[cfg(feature = "webp")]
use rimage::codecs::webp::WebPEncoder;
use zune_core::{bytestream::ZByteWriterTrait, options::EncoderOptions};
use zune_image::{
    codecs::{
        ImageFormat, farbfeld::FarbFeldEncoder, jpeg::JpegEncoder, jpeg_xl::JxlEncoder,
        png::PngEncoder, ppm::PPMEncoder, qoi::QoiEncoder,
    },
    errors::ImageErrors,
    image::Image,
    metadata::AlphaState,
    traits::{EncoderTrait, OperationsTrait},
};
use zune_imageprocs::premul_alpha::PremultiplyAlpha;

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn decode<P: AsRef<Path>>(f: P, matches: &ArgMatches) -> Result<Image, ImageErrors> {
    Image::open(f.as_ref()).or_else(|e| {
        if matches!(e, ImageErrors::ImageDecoderNotImplemented(_)) {
            #[cfg(any(feature = "avif", feature = "webp", feature = "svg"))]
            let mut file = File::open(f.as_ref())?;

            #[cfg(feature = "svg")]
            {
                if f.as_ref()
                    .extension()
                    .is_some_and(|f| f.eq_ignore_ascii_case("svg") | f.eq_ignore_ascii_case("svgz"))
                {
                    let options = svg_options(matches, f.as_ref());
                    let decoder = SvgDecoder::try_new_with_options(file, options)?;

                    return Image::from_decoder(decoder);
                }

                file.seek(SeekFrom::Start(0))?;
            }

            #[cfg(feature = "avif")]
            {
                let mut file_content = vec![];

                file.read_to_end(&mut file_content)?;
                file.seek(SeekFrom::Start(0))?;

                if libavif::is_avif(&file_content) {
                    use rimage::codecs::avif::AvifDecoder;

                    let decoder = AvifDecoder::try_new(file)?;

                    return Image::from_decoder(decoder);
                };
                file.seek(SeekFrom::Start(0))?;
            }

            #[cfg(feature = "webp")]
            {
                if f.as_ref()
                    .extension()
                    .is_some_and(|f| f.eq_ignore_ascii_case("webp"))
                {
                    use rimage::codecs::webp::WebPDecoder;

                    let decoder = WebPDecoder::try_new(file)?;

                    return Image::from_decoder(decoder);
                }

                file.seek(SeekFrom::Start(0))?;
            }

            #[cfg(feature = "tiff")]
            {
                if f.as_ref()
                    .extension()
                    .is_some_and(|f| f.eq_ignore_ascii_case("tiff") | f.eq_ignore_ascii_case("tif"))
                {
                    use rimage::codecs::tiff::TiffDecoder;

                    let decoder = TiffDecoder::try_new(file)?;

                    return Image::from_decoder(decoder);
                }

                file.seek(SeekFrom::Start(0))?;
            }

            Err(ImageErrors::ImageDecoderNotImplemented(
                ImageFormat::Unknown,
            ))
        } else {
            Err(e)
        }
    })
}

#[cfg(feature = "svg")]
fn svg_options(matches: &ArgMatches, path: &Path) -> SvgOptions {
    SvgOptions {
        resources_dir: path.parent().map(Path::to_path_buf),
        scale: matches.get_one::<f32>("svg-scale").copied().unwrap_or(1.0),
        width: matches.get_one::<u32>("svg-width").copied(),
        height: matches.get_one::<u32>("svg-height").copied(),
    }
}

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn operations(matches: &ArgMatches, img: &Image) -> BTreeMap<usize, Box<dyn OperationsTrait>> {
    let mut map: BTreeMap<usize, Box<dyn OperationsTrait>> = BTreeMap::new();

    #[cfg(feature = "resize")]
    {
        use crate::cli::preprocessors::{ResizeFilter, ResizeValue};
        use fast_image_resize::ResizeAlg;
        use rimage::operations::resize::Resize;

        if let Some(values) = matches.get_many::<ResizeValue>("resize") {
            let filter = matches.get_one::<ResizeFilter>("filter");

            let downscale = matches.get_flag("downscale") && !matches.get_flag("no-downscale");
            let upscale = matches.get_flag("upscale") && !matches.get_flag("no-upscale");

            log::debug!("downscale: {downscale}, upscale: {upscale}");

            let mut size = img.dimensions();

            values
                .into_iter()
                .zip(matches.indices_of("resize").unwrap())
                .for_each(|(value, idx)| {
                    // Each value maps the size the previous resize left behind,
                    // so a chain composes instead of every value mapping the source dimensions.
                    let (w, h) = value.map_dimensions(size.0, size.1);
                    log::trace!("setup resize {value} on index {idx}");

                    // Skip if the image is already the desired size
                    // or if both downscale and upscale are disabled
                    if (!downscale && !upscale) || (w == size.0 && h == size.1) {
                        log::trace!("skip resize to {w}x{h} on index {idx}");
                        return;
                    }

                    if !downscale && (w <= size.0 || h <= size.1) {
                        log::trace!("downscaling disabled, skip resize to {w}x{h} on index {idx}");
                        return;
                    }

                    if !upscale && (w >= size.0 || h >= size.1) {
                        log::trace!("upscaling disabled, skip resize to {w}x{h} on index {idx}");
                        return;
                    }

                    map.insert(
                        idx,
                        Box::new(Resize::new(
                            w,
                            h,
                            filter
                                .copied()
                                .map(Into::<ResizeAlg>::into)
                                .unwrap_or_default(),
                        )),
                    );

                    size = (w, h);
                })
        }
    }

    #[cfg(feature = "quantization")]
    {
        use rimage::operations::quantize::Quantize;

        if let Some(values) = matches.get_many::<u8>("quantization") {
            let dithering = matches.get_one::<u8>("dithering");

            values
                .into_iter()
                .zip(matches.indices_of("quantization").unwrap())
                .for_each(|(value, idx)| {
                    log::trace!("setup quantization {value} on index {idx}");

                    map.insert(
                        idx,
                        Box::new(Quantize::new(*value, dithering.map(|q| *q as f32 / 100.))),
                    );
                })
        }
    }

    if let Some(values) = matches.get_many::<bool>("premultiply") {
        values
            .into_iter()
            .zip(matches.indices_of("premultiply").unwrap())
            .for_each(|(value, idx)| {
                if let Some(op) = map.get(&(idx + 2)) {
                    log::trace!("setup alpha premultiply for {}", op.name());

                    map.insert(
                        idx,
                        Box::new(PremultiplyAlpha::new(AlphaState::PreMultiplied)),
                    );

                    assert!(
                        !map.contains_key(&(idx + 3)),
                        "There is a operation at {} aborting",
                        idx + 3
                    );

                    map.insert(
                        idx + 3,
                        Box::new(PremultiplyAlpha::new(AlphaState::NonPreMultiplied)),
                    );
                } else {
                    log::warn!("No operation found for premultiply at index {idx}")
                }
            })
    }

    map
}

pub enum AvailableEncoders {
    FarbFeld(Box<FarbFeldEncoder>),
    Jpeg(Box<JpegEncoder>),
    JpegXl(Box<JxlEncoder>),
    #[cfg(feature = "mozjpeg")]
    MozJpeg(Box<MozJpegEncoder>),
    #[cfg(feature = "oxipng")]
    OxiPng(Box<OxiPngEncoder>),
    #[cfg(feature = "avif")]
    Avif(Box<AvifEncoder>),
    #[cfg(feature = "webp")]
    Webp(Box<WebPEncoder>),
    Png(Box<PngEncoder>),
    Ppm(Box<PPMEncoder>),
    Qoi(Box<QoiEncoder>),
}

impl AvailableEncoders {
    pub fn to_extension(&self) -> &'static str {
        match self {
            AvailableEncoders::FarbFeld(_) => "ff",
            AvailableEncoders::Jpeg(_) => "jpg",
            AvailableEncoders::JpegXl(_) => "jxl",
            #[cfg(feature = "mozjpeg")]
            AvailableEncoders::MozJpeg(_) => "jpg",
            #[cfg(feature = "oxipng")]
            AvailableEncoders::OxiPng(_) => "png",
            #[cfg(feature = "avif")]
            AvailableEncoders::Avif(_) => "avif",
            #[cfg(feature = "webp")]
            AvailableEncoders::Webp(_) => "webp",
            AvailableEncoders::Png(_) => "png",
            AvailableEncoders::Ppm(_) => "ppm",
            AvailableEncoders::Qoi(_) => "qoi",
        }
    }

    pub fn encode<T: ZByteWriterTrait>(
        &mut self,
        img: &Image,
        sink: T,
    ) -> Result<usize, ImageErrors> {
        match self {
            AvailableEncoders::FarbFeld(enc) => enc.encode(img, sink),
            AvailableEncoders::Jpeg(enc) => enc.encode(img, sink),
            AvailableEncoders::JpegXl(enc) => enc.encode(img, sink),
            AvailableEncoders::MozJpeg(enc) => enc.encode(img, sink),
            AvailableEncoders::OxiPng(enc) => enc.encode(img, sink),
            AvailableEncoders::Avif(enc) => enc.encode(img, sink),
            AvailableEncoders::Webp(enc) => enc.encode(img, sink),
            AvailableEncoders::Png(enc) => enc.encode(img, sink),
            AvailableEncoders::Ppm(enc) => enc.encode(img, sink),
            AvailableEncoders::Qoi(enc) => enc.encode(img, sink),
        }
    }
}

pub fn encoder(name: &str, matches: &ArgMatches) -> Result<AvailableEncoders, ImageErrors> {
    match name {
        "farbfeld" => Ok(AvailableEncoders::FarbFeld(
            Box::new(FarbFeldEncoder::new()),
        )),
        "jpeg" => {
            let options = EncoderOptions::default();

            if let Some(quality) = matches.get_one::<u8>("quality") {
                options.set_quality(*quality);
            }

            options.set_jpeg_encode_progressive(matches.get_flag("progressive"));

            Ok(AvailableEncoders::Jpeg(Box::new(
                JpegEncoder::new_with_options(options),
            )))
        }
        "jpeg_xl" => Ok(AvailableEncoders::JpegXl(Box::new(JxlEncoder::new()))),
        #[cfg(feature = "mozjpeg")]
        "mozjpeg" => {
            use mozjpeg::qtable;
            use rimage::codecs::mozjpeg::MozJpegOptions;

            let quality = matches.get_one::<u8>("quality").copied().unwrap_or(75) as f32;
            let chroma_quality = matches
                .get_one::<u8>("chroma_quality")
                .map(|q| *q as f32)
                .unwrap_or(quality);

            let options = MozJpegOptions {
                quality,
                progressive: !matches.get_flag("baseline"),
                optimize_coding: !matches.get_flag("no_optimize_coding"),
                smoothing: matches
                    .get_one::<u8>("smoothing")
                    .copied()
                    .unwrap_or_default(),
                color_space: match matches
                    .get_one::<String>("colorspace")
                    .map(|s| s.as_str())
                    .unwrap_or("ycbcr")
                {
                    "ycbcr" => mozjpeg::ColorSpace::JCS_YCbCr,
                    "rgb" => mozjpeg::ColorSpace::JCS_EXT_RGB,
                    "grayscale" => mozjpeg::ColorSpace::JCS_GRAYSCALE,
                    cs => {
                        return Err(ImageErrors::GenericString(format!(
                            "Unsupported mozjpeg colorspace: {cs}",
                        )));
                    }
                },
                trellis_multipass: matches.get_flag("multipass"),
                chroma_subsample: matches.get_one::<u8>("subsample").copied(),

                luma_qtable: match matches.get_one::<String>("qtable") {
                    Some(c) => Some(match c.as_str() {
                        "AhumadaWatsonPeterson" => {
                            qtable::AhumadaWatsonPeterson.scaled(quality, quality)
                        }
                        "AnnexK" => qtable::AnnexK_Luma.scaled(quality, quality),
                        "Flat" => qtable::Flat.scaled(quality, quality),
                        "KleinSilversteinCarney" => {
                            qtable::KleinSilversteinCarney.scaled(quality, quality)
                        }
                        "MSSSIM" => qtable::MSSSIM_Luma.scaled(quality, quality),
                        "NRobidoux" => qtable::NRobidoux.scaled(quality, quality),
                        "PSNRHVS" => qtable::PSNRHVS_Luma.scaled(quality, quality),
                        "PetersonAhumadaWatson" => {
                            qtable::PetersonAhumadaWatson.scaled(quality, quality)
                        }
                        "WatsonTaylorBorthwick" => {
                            qtable::WatsonTaylorBorthwick.scaled(quality, quality)
                        }
                        q => {
                            return Err(ImageErrors::GenericString(
                                format!("Unknown qtable: {q}",),
                            ));
                        }
                    }),
                    None => None,
                },

                chroma_qtable: match matches.get_one::<String>("qtable") {
                    Some(c) => Some(match c.as_str() {
                        "AhumadaWatsonPeterson" => {
                            qtable::AhumadaWatsonPeterson.scaled(chroma_quality, chroma_quality)
                        }
                        "AnnexK" => qtable::AnnexK_Chroma.scaled(chroma_quality, chroma_quality),
                        "Flat" => qtable::Flat.scaled(chroma_quality, chroma_quality),
                        "KleinSilversteinCarney" => {
                            qtable::KleinSilversteinCarney.scaled(chroma_quality, chroma_quality)
                        }
                        "MSSSIM" => qtable::MSSSIM_Chroma.scaled(chroma_quality, chroma_quality),
                        "NRobidoux" => qtable::NRobidoux.scaled(chroma_quality, chroma_quality),
                        "PSNRHVS" => qtable::PSNRHVS_Chroma.scaled(chroma_quality, chroma_quality),
                        "PetersonAhumadaWatson" => {
                            qtable::PetersonAhumadaWatson.scaled(chroma_quality, chroma_quality)
                        }
                        "WatsonTaylorBorthwick" => {
                            qtable::WatsonTaylorBorthwick.scaled(chroma_quality, chroma_quality)
                        }
                        q => {
                            return Err(ImageErrors::GenericString(
                                format!("Unknown qtable: {q}",),
                            ));
                        }
                    }),
                    None => None,
                },
            };

            Ok(AvailableEncoders::MozJpeg(Box::new(
                MozJpegEncoder::new_with_options(options),
            )))
        }
        #[cfg(feature = "oxipng")]
        "oxipng" => {
            use rimage::codecs::oxipng::OxiPngOptions;

            let mut options =
                OxiPngOptions::from_preset(*matches.get_one::<u8>("effort").unwrap_or(&2));

            options.interlace = if matches.get_flag("interlace") {
                Some(true)
            } else {
                None
            };

            Ok(AvailableEncoders::OxiPng(Box::new(
                OxiPngEncoder::new_with_options(options),
            )))
        }
        #[cfg(feature = "avif")]
        "avif" => {
            use rimage::codecs::avif::AvifOptions;

            let options = AvifOptions {
                quality: matches.get_one::<u8>("quality").copied().unwrap_or(50) as f32,
                alpha_quality: matches.get_one::<u8>("alpha_quality").map(|q| *q as f32),
                speed: matches.get_one::<u8>("speed").copied().unwrap_or(6),
                color_space: match matches
                    .get_one::<String>("colorspace")
                    .map(|s| s.as_str())
                    .unwrap_or("ycbcr")
                {
                    "ycbcr" => ravif::ColorModel::YCbCr,
                    "rgb" => ravif::ColorModel::RGB,
                    cs => {
                        return Err(ImageErrors::GenericString(format!(
                            "Unsupported avif colorspace: {cs}",
                        )));
                    }
                },
                alpha_color_mode: match matches
                    .get_one::<String>("alpha_mode")
                    .map(|s| s.as_str())
                    .unwrap_or("UnassociatedClean")
                {
                    "UnassociatedDirty" => ravif::AlphaColorMode::UnassociatedDirty,
                    "UnassociatedClean" => ravif::AlphaColorMode::UnassociatedClean,
                    "Premultiplied" => ravif::AlphaColorMode::Premultiplied,
                    mode => {
                        return Err(ImageErrors::GenericString(format!(
                            "Unsupported avif alpha mode: {mode}",
                        )));
                    }
                },
            };

            Ok(AvailableEncoders::Avif(Box::new(
                AvifEncoder::new_with_options(options),
            )))
        }
        #[cfg(feature = "webp")]
        "webp" => {
            use rimage::codecs::webp::WebPOptions;

            let mut options = WebPOptions::new().unwrap();

            options.quality = matches.get_one::<u8>("quality").copied().unwrap_or(75) as f32;
            options.lossless = matches.get_flag("lossless") as i32;
            options.near_lossless =
                100 - matches.get_one::<u8>("slight_loss").copied().unwrap_or(0) as i32;
            options.exact = matches.get_flag("exact") as i32;

            Ok(AvailableEncoders::Webp(Box::new(
                WebPEncoder::new_with_options(options),
            )))
        }
        "png" => Ok(AvailableEncoders::Png(Box::new(PngEncoder::new()))),
        "ppm" => Ok(AvailableEncoders::Ppm(Box::new(PPMEncoder::new()))),
        "qoi" => Ok(AvailableEncoders::Qoi(Box::new(QoiEncoder::new()))),

        name => Err(ImageErrors::GenericString(format!(
            "Encoder \"{name}\" not found",
        ))),
    }
}

#[cfg(all(test, feature = "resize"))]
mod tests {
    use zune_core::colorspace::ColorSpace;

    use super::*;
    use crate::cli::cli;

    /// Builds the codec subcommand matches the way `main` passes them to [`operations`].
    ///
    /// `farbfeld` is used because it is the one codec that is never feature gated.
    fn matches_from(args: &[&str]) -> ArgMatches {
        cli()
            .get_matches_from(args)
            .subcommand()
            .expect("clap ensures a subcommand is always provided")
            .1
            .clone()
    }

    fn test_image(width: usize, height: usize) -> Image {
        Image::from_fn(width, height, ColorSpace::RGB, |x, y, px: &mut [u8; 4]| {
            px[0] = x as u8;
            px[1] = 0;
            px[2] = y as u8;
        })
    }

    /// Runs the preprocessing pipeline over an image of the given size.
    ///
    /// Reports how many resize operations were queued next to the resulting
    /// dimensions, because "the image already fits" means none were queued at all.
    fn run(resize_args: &[&str], width: usize, height: usize) -> (usize, (usize, usize)) {
        let mut args = vec!["rimage", "farbfeld"];
        args.extend_from_slice(resize_args);
        args.push("image.ff");

        let matches = matches_from(&args);
        let mut img = test_image(width, height);

        let ops = operations(&matches, &img);
        let queued = ops.values().filter(|op| op.name() == "fast resize").count();

        for op in ops.values() {
            op.execute(&mut img).unwrap();
        }

        (queued, img.dimensions())
    }

    #[test]
    fn longest_side_anchors_per_orientation() {
        assert_eq!(run(&["--resize", "1000l"], 2000, 1000), (1, (1000, 500)));
        assert_eq!(run(&["--resize", "1000l"], 1000, 2000), (1, (500, 1000)));
        assert_eq!(run(&["--resize", "1000l"], 1600, 1600), (1, (1000, 1000)));
    }

    #[test]
    fn shortest_side_anchors_per_orientation() {
        assert_eq!(run(&["--resize", "500s"], 2000, 1000), (1, (1000, 500)));
        assert_eq!(run(&["--resize", "500s"], 1000, 2000), (1, (500, 1000)));
        assert_eq!(run(&["--resize", "500s"], 1600, 1600), (1, (500, 500)));
    }

    #[test]
    fn side_value_is_skipped_when_the_image_already_fits_exactly() {
        assert_eq!(run(&["--resize", "1000l"], 1000, 500), (0, (1000, 500)));
        assert_eq!(run(&["--resize", "500s"], 1000, 500), (0, (1000, 500)));
    }

    #[test]
    fn no_upscale_leaves_smaller_images_untouched() {
        // The whole point of the flag pair: images already under the target
        // keep their original size instead of being blown up to it.
        assert_eq!(
            run(&["--resize", "1000l", "--no-upscale"], 800, 400),
            (0, (800, 400))
        );

        assert_eq!(
            run(&["--resize", "500s", "--no-upscale"], 800, 400),
            (0, (800, 400))
        );
    }

    #[test]
    fn no_upscale_still_shrinks_larger_images() {
        assert_eq!(
            run(&["--resize", "1000l", "--no-upscale"], 4000, 2000),
            (1, (1000, 500))
        );

        assert_eq!(
            run(&["--resize", "1000l", "--no-upscale"], 2000, 4000),
            (1, (500, 1000))
        );
    }

    #[test]
    fn no_downscale_leaves_larger_images_untouched() {
        assert_eq!(
            run(&["--resize", "1000l", "--no-downscale"], 4000, 2000),
            (0, (4000, 2000))
        );

        assert_eq!(
            run(&["--resize", "500s", "--no-downscale"], 4000, 2000),
            (0, (4000, 2000))
        );
    }

    #[test]
    fn no_downscale_still_enlarges_smaller_images() {
        assert_eq!(
            run(&["--resize", "1000l", "--no-downscale"], 800, 400),
            (1, (1000, 500))
        );

        assert_eq!(
            run(&["--resize", "1000l", "--no-downscale"], 400, 800),
            (1, (500, 1000))
        );
    }

    #[test]
    fn a_mixed_batch_ends_up_with_a_consistent_longest_side() {
        // This is the case `100w` cannot express: with a fixed width anchor the
        // portrait images below would come out far taller than the landscape
        // ones are wide.
        for (width, height) in [(4000, 3000), (3000, 4000), (2000, 2000), (1200, 900)] {
            let (_, (new_width, new_height)) = run(&["--resize", "1000l"], width, height);

            assert_eq!(
                new_width.max(new_height),
                1000,
                "{width}x{height} gave {new_width}x{new_height}"
            );
        }
    }

    #[test]
    fn a_mixed_batch_ends_up_with_a_consistent_shortest_side() {
        for (width, height) in [(4000, 3000), (3000, 4000), (2000, 2000), (1200, 900)] {
            let (_, (new_width, new_height)) = run(&["--resize", "500s"], width, height);

            assert_eq!(
                new_width.min(new_height),
                500,
                "{width}x{height} gave {new_width}x{new_height}"
            );
        }
    }

    #[test]
    fn shrink_only_batch_touches_only_oversized_images() {
        // 1000l with --no-upscale is the shrink only mode from the feature
        // request: everything ends up at or below 1000px, and anything that was
        // already small keeps its exact original size.
        let batch = [
            ((4000, 3000), (1000, 750)),
            ((3000, 4000), (750, 1000)),
            ((800, 600), (800, 600)),
            ((1000, 1000), (1000, 1000)),
        ];

        for ((width, height), expected) in batch {
            let (_, dimensions) = run(&["--resize", "1000l", "--no-upscale"], width, height);

            assert_eq!(dimensions, expected, "failed on {width}x{height}");
        }
    }

    #[test]
    fn reduce_only_is_an_alias_for_no_upscale() {
        for (width, height) in [(4000, 2000), (800, 400), (1000, 500)] {
            assert_eq!(
                run(&["--resize", "1000l", "--reduce-only"], width, height),
                run(&["--resize", "1000l", "--no-upscale"], width, height),
                "--reduce-only diverged from --no-upscale on {width}x{height}"
            );
        }
    }

    #[test]
    fn enlarge_only_is_an_alias_for_no_downscale() {
        for (width, height) in [(4000, 2000), (800, 400), (1000, 500)] {
            assert_eq!(
                run(&["--resize", "1000l", "--enlarge-only"], width, height),
                run(&["--resize", "1000l", "--no-downscale"], width, height),
                "--enlarge-only diverged from --no-downscale on {width}x{height}"
            );
        }
    }

    #[test]
    fn reduce_only_and_enlarge_only_pick_the_direction_their_names_promise() {
        // Guards against the aliases being wired to the opposite flag, which is
        // the whole risk of naming a double negative.
        assert_eq!(
            run(&["--resize", "1000l", "--reduce-only"], 4000, 2000),
            (1, (1000, 500))
        );
        assert_eq!(
            run(&["--resize", "1000l", "--reduce-only"], 800, 400),
            (0, (800, 400))
        );

        assert_eq!(
            run(&["--resize", "1000l", "--enlarge-only"], 800, 400),
            (1, (1000, 500))
        );
        assert_eq!(
            run(&["--resize", "1000l", "--enlarge-only"], 4000, 2000),
            (0, (4000, 2000))
        );
    }

    #[test]
    fn width_anchor_with_reduce_caps_the_width_only() {
        // Capping one dimension however long the other one is does not need a
        // side value at all, the existing width anchor already covers it. The
        // height follows the aspect ratio, and images at or under the cap keep
        // their original size.
        assert_eq!(
            run(&["--resize", "400w", "--reduce-only"], 200, 100),
            (0, (200, 100))
        );
        assert_eq!(
            run(&["--resize", "400w", "--reduce-only"], 400, 2000),
            (0, (400, 2000))
        );
        assert_eq!(
            run(&["--resize", "400w", "--reduce-only"], 800, 900),
            (1, (400, 450))
        );
    }

    #[test]
    fn disabling_both_directions_skips_every_resize() {
        // Neither flag wins over the other, they both apply, which leaves no
        // direction to resize in. The order they are passed does not matter.
        for args in [
            ["--resize", "1000l", "--no-upscale", "--no-downscale"],
            ["--resize", "1000l", "--no-downscale", "--no-upscale"],
            ["--resize", "1000l", "--reduce-only", "--enlarge-only"],
        ] {
            assert_eq!(run(&args, 4000, 2000), (0, (4000, 2000)));
            assert_eq!(run(&args, 800, 400), (0, (800, 400)));
            assert_eq!(run(&args, 1000, 500), (0, (1000, 500)));
        }
    }

    #[test]
    fn chained_resize_values_compose() {
        // Each value maps the size the previous resize left behind, so the
        // chain applies in order instead of every value mapping the source.
        assert_eq!(
            run(&["--resize", "100x400", "--resize", "200s"], 800, 400),
            (2, (200, 800))
        );

        assert_eq!(
            run(&["--resize", "1000l", "--resize", "50%"], 800, 400),
            (2, (500, 250))
        );

        // The first resize already lands on the target the second one maps to,
        // so the second is skipped as "already fits" instead of running twice.
        assert_eq!(
            run(&["--resize", "1000l", "--resize", "500s"], 800, 400),
            (1, (1000, 500))
        );

        // A skipped resize leaves the current size unchanged, so the next value
        // maps the size the resize actually ran on.
        assert_eq!(
            run(
                &["--resize", "2000l", "--resize", "50%", "--no-upscale"],
                800,
                400
            ),
            (1, (400, 200))
        );
    }

    #[test]
    fn side_values_compose_with_the_filter_flag() {
        assert_eq!(
            run(&["--resize", "1000l", "--filter", "nearest"], 2000, 1000),
            (1, (1000, 500))
        );
    }
}

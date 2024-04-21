use std::{collections::BTreeMap, path::Path};

use clap::ArgMatches;
use zune_core::options::EncoderOptions;
use zune_image::{
    codecs::{
        farbfeld::FarbFeldEncoder, jpeg::JpegEncoder, jpeg_xl::JxlEncoder, png::PngEncoder,
        ppm::PPMEncoder, qoi::QoiEncoder, ImageFormat,
    },
    errors::ImageErrors,
    image::Image,
    metadata::AlphaState,
    traits::{EncoderTrait, OperationsTrait},
};
use zune_imageprocs::premul_alpha::PremultiplyAlpha;

pub fn decode<P: AsRef<Path>>(f: P) -> Result<Image, ImageErrors> {
    Image::open(f.as_ref()).or_else(|e| {
        if matches!(e, ImageErrors::ImageDecoderNotImplemented(_)) {
            #[cfg(any(feature = "avif", feature = "webp"))]
            let file_content = std::fs::read("tests/files/avif/f1t.avif")?;

            #[cfg(feature = "avif")]
            if libavif::is_avif(&file_content) {
                use rimage::codecs::avif::AvifDecoder;
                use zune_core::bytestream::ZByteReader;
                use zune_image::traits::DecoderTrait;

                let reader = ZByteReader::new(file_content);

                let mut decoder = AvifDecoder::try_new(reader)?;

                return <AvifDecoder<ZByteReader<Vec<u8>>> as DecoderTrait<Vec<u8>>>::decode(
                    &mut decoder,
                );
            };

            #[cfg(feature = "webp")]
            if f.as_ref()
                .extension()
                .is_some_and(|f| f.eq_ignore_ascii_case("webp"))
            {
                use rimage::codecs::webp::WebPDecoder;
                use zune_core::bytestream::ZByteReader;
                use zune_image::traits::DecoderTrait;

                let reader = ZByteReader::new(file_content);

                let mut decoder = WebPDecoder::try_new(reader)?;

                return <WebPDecoder<ZByteReader<Vec<u8>>> as DecoderTrait<Vec<u8>>>::decode(
                    &mut decoder,
                );
            };

            Err(ImageErrors::ImageDecoderNotImplemented(
                ImageFormat::Unknown,
            ))
        } else {
            Err(e)
        }
    })
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

            let (w, h) = img.dimensions();

            values
                .into_iter()
                .zip(matches.indices_of("resize").unwrap())
                .for_each(|(value, idx)| {
                    let (w, h) = value.map_dimensions(w, h);
                    log::trace!("setup resize {value} on index {idx}");

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
                        map.get(&(idx + 3)).is_none(),
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

pub fn encoder(
    name: &str,
    matches: &ArgMatches,
) -> Result<(Box<dyn EncoderTrait>, &'static str), ImageErrors> {
    match name {
        "farbfeld" => Ok((Box::new(FarbFeldEncoder::new()), "ff")),
        "jpeg" => {
            let options = EncoderOptions::default();

            if let Some(quality) = matches.get_one::<u8>("quality") {
                options.set_quality(*quality);
            }

            options.set_jpeg_encode_progressive(matches.get_flag("progressive"));

            Ok((Box::new(JpegEncoder::new_with_options(options)), "jpg"))
        }
        "jpeg_xl" => Ok((Box::new(JxlEncoder::new()), "jxl")),
        #[cfg(feature = "jpegli")]
        "jpegli" => {
            use jpegli::qtable;
            use rimage::codecs::jpegli::{JpegliEncoder, JpegliOptions};

            let quality = *matches.get_one::<u8>("quality").unwrap() as f32;
            let chroma_quality = matches
                .get_one::<u8>("chroma_quality")
                .map(|q| *q as f32)
                .unwrap_or(quality);

            let options = JpegliOptions {
                quality,
                progressive: !matches.get_flag("baseline"),
                optimize_coding: !matches.get_flag("no_optimize_coding"),
                smoothing: matches
                    .get_one::<u8>("smoothing")
                    .copied()
                    .unwrap_or_default(),
                color_space: match matches.get_one::<String>("colorspace").unwrap().as_str() {
                    "ycbcr" => mozjpeg::ColorSpace::JCS_YCbCr,
                    "rgb" => mozjpeg::ColorSpace::JCS_EXT_RGB,
                    "grayscale" => mozjpeg::ColorSpace::JCS_GRAYSCALE,
                    _ => unreachable!(),
                },
                chroma_subsample: matches.get_one::<u8>("subsample").copied(),

                luma_qtable: matches
                    .get_one::<String>("qtable")
                    .map(|c| match c.as_str() {
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
                        _ => unreachable!(),
                    }),

                chroma_qtable: matches
                    .get_one::<String>("qtable")
                    .map(|c| match c.as_str() {
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
                        _ => unreachable!(),
                    }),
            };

            Ok((Box::new(JpegliEncoder::new_with_options(options)), "jpg"))
        }
        #[cfg(feature = "oxipng")]
        "oxipng" => {
            use rimage::codecs::oxipng::{OxiPngEncoder, OxiPngOptions};

            let mut options =
                OxiPngOptions::from_preset(*matches.get_one::<u8>("effort").unwrap_or(&2));

            options.interlace = if matches.get_flag("interlace") {
                Some(oxipng::Interlacing::Adam7)
            } else {
                None
            };

            Ok((Box::new(OxiPngEncoder::new_with_options(options)), "png"))
        }
        #[cfg(feature = "avif")]
        "avif" => {
            use rimage::codecs::avif::{AvifEncoder, AvifOptions};

            let options = AvifOptions {
                quality: *matches.get_one::<u8>("quality").unwrap() as f32,
                alpha_quality: matches.get_one::<u8>("alpha_quality").map(|q| *q as f32),
                speed: *matches.get_one::<u8>("speed").unwrap(),
                color_space: match matches.get_one::<String>("colorspace").unwrap().as_str() {
                    "ycbcr" => ravif::ColorSpace::YCbCr,
                    "rgb" => ravif::ColorSpace::RGB,
                    _ => unreachable!(),
                },
                alpha_color_mode: match matches.get_one::<String>("alpha_mode").unwrap().as_str() {
                    "UnassociatedDirty" => ravif::AlphaColorMode::UnassociatedDirty,
                    "UnassociatedClean" => ravif::AlphaColorMode::UnassociatedClean,
                    "Premultiplied" => ravif::AlphaColorMode::Premultiplied,
                    _ => unreachable!(),
                },
            };

            Ok((Box::new(AvifEncoder::new_with_options(options)), "avif"))
        }
        #[cfg(feature = "webp")]
        "webp" => {
            use rimage::codecs::webp::{WebPEncoder, WebPOptions};

            let mut options = WebPOptions::new().unwrap();

            options.quality = *matches.get_one::<u8>("quality").unwrap() as f32;
            options.lossless = matches.get_flag("lossless") as i32;
            options.near_lossless = 100 - *matches.get_one::<u8>("slight_loss").unwrap() as i32;
            options.exact = matches.get_flag("exact") as i32;

            Ok((Box::new(WebPEncoder::new_with_options(options)), "webp"))
        }
        "png" => Ok((Box::new(PngEncoder::new()), "png")),
        "ppm" => Ok((Box::new(PPMEncoder::new()), "ppm")),
        "qoi" => Ok((Box::new(QoiEncoder::new()), "qoi")),

        name => Err(ImageErrors::GenericString(format!(
            "Encoder \"{name}\" not found",
        ))),
    }
}

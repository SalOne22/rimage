use std::io::Write;

use clap::ArgMatches;
use rayon::iter::{
    IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge, ParallelIterator,
};
use zune_core::{bit_depth::BitType, colorspace::ColorSpace};
use zune_image::{channel::Channel, errors::ImageErrors, traits::OperationsTrait};

pub struct Quantization {
    quality: u8,
    dithering: u8,
}

impl Quantization {
    pub fn from_matches(matches: &ArgMatches) -> Option<impl Iterator<Item = (Self, usize)> + '_> {
        if let Some(quantization_processors) = matches.get_occurrences::<u8>("quantization") {
            let dithering = matches.get_one::<u8>("dithering");

            return Some(
                quantization_processors
                    .flatten()
                    .zip(matches.indices_of("quantization").unwrap())
                    .map(move |(value, index)| {
                        (
                            Self {
                                quality: *value,
                                dithering: *dithering.unwrap_or(&75),
                            },
                            index,
                        )
                    }),
            );
        }

        None
    }
}

impl OperationsTrait for Quantization {
    fn name(&self) -> &'static str {
        "quantization"
    }

    fn execute_impl(
        &self,
        image: &mut zune_image::image::Image,
    ) -> Result<(), zune_image::errors::ImageErrors> {
        if image.colorspace() != ColorSpace::RGBA {
            return Err(zune_image::errors::ImageErrors::OperationsError(
                zune_image::errors::ImageOperationsErrors::WrongColorspace(
                    ColorSpace::RGBA,
                    image.colorspace(),
                ),
            ));
        }

        let (width, height) = image.dimensions();
        let depth = image.depth().bit_type();

        let mut liq = imagequant::new();
        liq.set_speed(5).unwrap();
        liq.set_quality(0, self.quality).map_err(|e| {
            ImageErrors::OperationsError(zune_image::errors::ImageOperationsErrors::GenericString(
                e.to_string(),
            ))
        })?;

        image.channels_mut(false).par_iter_mut().try_for_each(
            |old_channel| -> Result<(), ImageErrors> {
                let mut new_channel = Channel::new_with_bit_type(old_channel.len(), depth);

                {
                    let mut img = liq
                        .new_image_borrowed(old_channel.reinterpret_as()?, width, height, 0.0)
                        .map_err(|e| {
                            ImageErrors::OperationsError(
                                zune_image::errors::ImageOperationsErrors::GenericString(
                                    e.to_string(),
                                ),
                            )
                        })?;

                    let mut res = liq.quantize(&mut img).map_err(|e| {
                        ImageErrors::OperationsError(
                            zune_image::errors::ImageOperationsErrors::GenericString(e.to_string()),
                        )
                    })?;

                    res.set_dithering_level(self.dithering as f32 / 100.)
                        .map_err(|e| {
                            ImageErrors::OperationsError(
                                zune_image::errors::ImageOperationsErrors::GenericString(
                                    e.to_string(),
                                ),
                            )
                        })?;

                    let (palette, pixels) = res.remapped(&mut img).map_err(|e| {
                        ImageErrors::OperationsError(
                            zune_image::errors::ImageOperationsErrors::GenericString(e.to_string()),
                        )
                    })?;

                    new_channel.reinterpret_as_mut()?.write(
                        &pixels
                            .par_iter()
                            .flat_map(|pix| palette[*pix as usize].iter().par_bridge())
                            .collect::<Vec<u8>>(),
                    )?;
                }

                **old_channel = new_channel;

                Ok(())
            },
        )?;

        todo!()
    }

    fn supported_types(&self) -> &'static [BitType] {
        &[BitType::U8]
    }
}

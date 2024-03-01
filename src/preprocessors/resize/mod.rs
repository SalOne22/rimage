mod filter;
mod fit;
mod value;

use std::{io::Write, num::NonZeroU32};

pub use self::fit::ResizeFit;
use clap::ArgMatches;
use fast_image_resize as fr;
pub use filter::ResizeFilter;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
pub use value::ResizeValue;
use zune_core::colorspace::ColorSpace;
use zune_image::{
    channel::Channel, codecs::qoi::zune_core::bit_depth::BitType, errors::ImageErrors,
    traits::OperationsTrait,
};

pub struct Resize {
    value: ResizeValue,
    filter: ResizeFilter,
    fit: ResizeFit,
}

impl Resize {
    pub fn from_matches(matches: &ArgMatches) -> Option<impl Iterator<Item = (Self, usize)> + '_> {
        if let Some(resize_processors) = matches.get_occurrences::<ResizeValue>("resize") {
            let resize_filter = matches.get_one::<ResizeFilter>("filter");
            let resize_fit = matches.get_one::<ResizeFit>("fit");

            return Some(
                resize_processors
                    .flatten()
                    .zip(matches.indices_of("resize").unwrap())
                    .map(move |(value, index)| {
                        (
                            Self {
                                value: *value,
                                filter: *resize_filter.unwrap_or(&ResizeFilter::Lanczos3),
                                fit: *resize_fit.unwrap_or(&ResizeFit::Stretch),
                            },
                            index,
                        )
                    }),
            );
        }

        None
    }
}

impl OperationsTrait for Resize {
    fn name(&self) -> &'static str {
        "Resize"
    }

    fn execute_impl(
        &self,
        image: &mut zune_image::image::Image,
    ) -> Result<(), zune_image::errors::ImageErrors> {
        let (width, height) = image.dimensions();
        let depth = image.depth().bit_type();

        let width = NonZeroU32::new(width as u32).unwrap();
        let height = NonZeroU32::new(height as u32).unwrap();

        let (dst_width, dst_height) = self.value.map_dimensions(width.get(), height.get());

        let new_length = dst_width * dst_height * image.depth().size_of() as u32;

        let dst_width = NonZeroU32::new(dst_width).ok_or(ImageErrors::OperationsError(
            zune_image::errors::ImageOperationsErrors::Generic("width cannot be zero"),
        ))?;
        let dst_height = NonZeroU32::new(dst_height).ok_or(ImageErrors::OperationsError(
            zune_image::errors::ImageOperationsErrors::Generic("height cannot be zero"),
        ))?;

        if image.colorspace() != ColorSpace::RGBA {
            return Err(zune_image::errors::ImageErrors::OperationsError(
                zune_image::errors::ImageOperationsErrors::WrongColorspace(
                    ColorSpace::RGBA,
                    image.colorspace(),
                ),
            ));
        }

        image.channels_mut(false).par_iter_mut().try_for_each(
            |old_channel| -> Result<(), ImageErrors> {
                let mut new_channel = Channel::new_with_bit_type(new_length as usize, depth);

                let mut src_image = fr::Image::from_slice_u8(
                    width,
                    height,
                    old_channel.reinterpret_as_mut()?,
                    match depth {
                        BitType::U8 => fr::PixelType::U8x4,
                        BitType::U16 => fr::PixelType::U16x4,
                        BitType::F32 => fr::PixelType::F32,
                        d => {
                            return Err(ImageErrors::OperationsError(
                                zune_image::errors::ImageOperationsErrors::UnsupportedType(
                                    "resize", d,
                                ),
                            ))
                        }
                    },
                )
                .map_err(|e| {
                    ImageErrors::OperationsError(
                        zune_image::errors::ImageOperationsErrors::GenericString(e.to_string()),
                    )
                })?;

                match self.fit {
                    ResizeFit::Stretch => {}
                    ResizeFit::Cover => {
                        src_image.view().set_crop_box_to_fit_dst_size(
                            dst_width,
                            dst_height,
                            Some((0.5, 0.5)),
                        );
                    }
                }

                let alpha_mul_div = fr::MulDiv::default();
                alpha_mul_div
                    .multiply_alpha_inplace(&mut src_image.view_mut())
                    .unwrap();

                let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

                let mut dst_view = dst_image.view_mut();

                let mut resizer = fr::Resizer::new(self.filter.into());

                resizer.resize(&src_image.view(), &mut dst_view).unwrap();

                alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

                new_channel
                    .reinterpret_as_mut()?
                    .write(dst_image.buffer())?;

                **old_channel = new_channel;

                Ok(())
            },
        )?;

        image.set_dimensions(dst_width.get() as usize, dst_height.get() as usize);

        Ok(())
    }

    fn supported_types(&self) -> &'static [zune_image::codecs::qoi::zune_core::bit_depth::BitType] {
        &[BitType::U8, BitType::U16, BitType::F32]
    }
}

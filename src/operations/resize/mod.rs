use std::num::NonZeroU32;

use fast_image_resize as fr;
pub use fast_image_resize::{FilterType, ResizeAlg};
use zune_core::bit_depth::BitType;
use zune_image::{
    channel::Channel,
    errors::{ImageErrors, ImageOperationsErrors},
    image::Image,
    traits::OperationsTrait,
};

/// Resize an image to a new dimensions
/// using the resize algorithm specified
pub struct Resize {
    new_dimensions: (usize, usize),
    algorithm: fr::ResizeAlg,
}

impl Resize {
    /// Create a new resize operation
    ///
    /// # Argument
    /// - width: The new image width
    /// - height: The new image height.
    /// - algorithm: The resize algorithm to use
    #[must_use]
    pub fn new(width: usize, height: usize, algorithm: fr::ResizeAlg) -> Self {
        Self {
            new_dimensions: (width, height),
            algorithm,
        }
    }
}

impl OperationsTrait for Resize {
    fn name(&self) -> &'static str {
        "fast resize"
    }

    fn execute_impl(&self, image: &mut Image) -> Result<(), ImageErrors> {
        let (src_width, src_height) = image.dimensions();
        let (dst_width, dst_height) = self.new_dimensions;

        let depth = image.depth().bit_type();

        let new_length = dst_width * dst_height * image.depth().size_of();

        let width = NonZeroU32::new(src_width as u32).unwrap();
        let height = NonZeroU32::new(src_height as u32).unwrap();

        let dst_width = NonZeroU32::new(dst_width as u32).unwrap();
        let dst_height = NonZeroU32::new(dst_height as u32).unwrap();

        #[cfg(feature = "threads")]
        std::thread::scope(|f| {
            let mut errors = vec![];

            for old_channel in image.channels_mut(false) {
                let result = f.spawn(|| {
                    let mut new_channel = Channel::new_with_bit_type(new_length, depth);

                    let src_image = fr::Image::from_slice_u8(
                        width,
                        height,
                        unsafe { old_channel.alias_mut() },
                        match depth {
                            BitType::U8 => fr::PixelType::U8,
                            BitType::U16 => fr::PixelType::U16,
                            BitType::F32 => fr::PixelType::F32,

                            d => {
                                return Err(ImageErrors::ImageOperationNotImplemented("resize", d))
                            }
                        },
                    )
                    .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

                    let mut dst_image =
                        fr::Image::new(dst_width, dst_height, src_image.pixel_type());

                    let mut dst_view = dst_image.view_mut();

                    let mut resizer = fr::Resizer::new(self.algorithm);

                    resizer
                        .resize(&src_image.view(), &mut dst_view)
                        .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

                    unsafe {
                        new_channel.alias_mut().copy_from_slice(dst_image.buffer());
                    }

                    *old_channel = new_channel;
                    Ok(())
                });
                errors.push(result);
            }

            errors
                .into_iter()
                .map(|x| x.join().unwrap())
                .collect::<Result<Vec<()>, ImageErrors>>()
        })?;

        #[cfg(not(feature = "threads"))]
        for old_channel in image.channels_mut(false) {
            let mut new_channel = Channel::new_with_bit_type(new_length, depth);

            let src_image = fr::Image::from_slice_u8(
                width,
                height,
                unsafe { old_channel.alias_mut() },
                match depth {
                    BitType::U8 => fr::PixelType::U8,
                    BitType::U16 => fr::PixelType::U16,
                    BitType::F32 => fr::PixelType::F32,

                    d => return Err(ImageErrors::ImageOperationNotImplemented("resize", d)),
                },
            )
            .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

            let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

            let mut dst_view = dst_image.view_mut();

            let mut resizer = fr::Resizer::new(self.algorithm);

            resizer
                .resize(&src_image.view(), &mut dst_view)
                .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

            unsafe {
                new_channel.alias_mut().copy_from_slice(dst_image.buffer());
            }

            *old_channel = new_channel;
        }

        image.set_dimensions(dst_width.get() as usize, dst_height.get() as usize);

        Ok(())
    }

    fn supported_types(&self) -> &'static [BitType] {
        &[BitType::U8, BitType::U16, BitType::F32]
    }
}

#[cfg(test)]
mod tests;

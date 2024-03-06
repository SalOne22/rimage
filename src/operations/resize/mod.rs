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

pub struct Resize {
    new_dimensions: (usize, usize),
    algorithm: fr::ResizeAlg,
}

impl Resize {
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
mod tests {
    use zune_core::colorspace::ColorSpace;

    use crate::test_utils::{
        create_test_image_animated, create_test_image_f32, create_test_image_u16,
        create_test_image_u8,
    };

    use super::*;

    #[test]
    fn resize_u8() {
        let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
        let mut image = create_test_image_u8(200, 200, ColorSpace::RGB);

        let result = resize.execute(&mut image);
        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(image.dimensions(), (100, 100));
    }

    #[test]
    fn resize_u16() {
        let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
        let mut image = create_test_image_u16(200, 200, ColorSpace::RGB);

        let result = resize.execute(&mut image);
        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(image.dimensions(), (100, 100));
    }

    #[test]
    fn resize_f32() {
        let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
        let mut image = create_test_image_f32(200, 200, ColorSpace::RGB);

        let result = resize.execute(&mut image);
        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(image.dimensions(), (100, 100));
    }

    #[test]
    fn resize_animated() {
        let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
        let mut image = create_test_image_animated(200, 200, ColorSpace::RGB);

        let result = resize.execute(&mut image);
        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(image.dimensions(), (100, 100));
    }
}

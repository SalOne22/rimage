use zune_core::{bit_depth::BitDepth, colorspace::ColorSpace};
use zune_image::{
    codecs::ImageFormat,
    errors::{ImageErrors, ImgEncodeErrors},
    traits::EncoderTrait,
};

/// Alias to [`oxipng::Options`]
pub type OxiPngOptions = oxipng::Options;

/// A OxiPNG encoder
#[derive(Default)]
pub struct OxiPngEncoder {
    options: OxiPngOptions,
}

impl OxiPngEncoder {
    /// Create a new encoder
    pub fn new() -> OxiPngEncoder {
        OxiPngEncoder::default()
    }
    /// Create a new encoder with specified options
    pub fn new_with_options(options: OxiPngOptions) -> OxiPngEncoder {
        OxiPngEncoder { options }
    }
}

impl EncoderTrait for OxiPngEncoder {
    fn name(&self) -> &'static str {
        "oxipng"
    }

    fn encode_inner(
        &mut self,
        image: &zune_image::image::Image,
    ) -> Result<Vec<u8>, zune_image::errors::ImageErrors> {
        let (width, height) = image.dimensions();

        // inlined `to_u8` method because its private
        let colorspace = image.colorspace();
        let data = if image.depth() == BitDepth::Eight {
            image.flatten_frames::<u8>()
        } else if image.depth() == BitDepth::Sixteen {
            image
                .frames_ref()
                .iter()
                .map(|z| z.u16_to_native_endian(colorspace))
                .collect()
        } else {
            unreachable!()
        }
        .into_iter()
        .next()
        .unwrap();

        let img = oxipng::RawImage::new(
            width as u32,
            height as u32,
            match image.colorspace() {
                ColorSpace::Luma => oxipng::ColorType::Grayscale {
                    transparent_shade: None,
                },
                ColorSpace::RGB => oxipng::ColorType::RGB {
                    transparent_color: None,
                },

                ColorSpace::LumaA => oxipng::ColorType::GrayscaleAlpha,
                ColorSpace::RGBA => oxipng::ColorType::RGBA,

                cs => {
                    return Err(ImageErrors::EncodeErrors(
                        ImgEncodeErrors::UnsupportedColorspace(cs, self.supported_colorspaces()),
                    ))
                }
            },
            match image.depth() {
                BitDepth::Eight => oxipng::BitDepth::Eight,
                BitDepth::Sixteen => oxipng::BitDepth::Sixteen,
                d => {
                    return Err(ImageErrors::EncodeErrors(ImgEncodeErrors::Generic(
                        format!("{d:?} is not supported"),
                    )))
                }
            },
            data,
        )
        .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?;

        Ok(img
            .create_optimized_png(&self.options)
            .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?)
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[
            ColorSpace::Luma,
            ColorSpace::LumaA,
            ColorSpace::RGB,
            ColorSpace::RGBA,
        ]
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::PNG
    }

    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Eight, BitDepth::Sixteen]
    }

    fn default_depth(&self, depth: BitDepth) -> BitDepth {
        match depth {
            BitDepth::Sixteen | BitDepth::Float32 => BitDepth::Sixteen,
            _ => BitDepth::Eight,
        }
    }
}

#[cfg(test)]
mod tests;

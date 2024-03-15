use ravif::Img;
use rgb::FromSlice;
use zune_core::{bit_depth::BitDepth, colorspace::ColorSpace};
use zune_image::{
    codecs::ImageFormat,
    errors::{ImageErrors, ImgEncodeErrors},
    traits::EncoderTrait,
};

/// Advanced options for AVIF encoding
pub struct AvifOptions {
    pub quality: f32,
    pub alpha_quality: Option<f32>,
    pub speed: u8,
    pub color_space: ravif::ColorSpace,
    pub alpha_color_mode: ravif::AlphaColorMode,
}

/// A AVIF encoder
pub struct AvifEncoder {
    options: AvifOptions,
}

impl Default for AvifOptions {
    fn default() -> Self {
        Self {
            quality: 50.,
            alpha_quality: None,
            speed: 6,
            color_space: ravif::ColorSpace::YCbCr,
            alpha_color_mode: ravif::AlphaColorMode::UnassociatedClean,
        }
    }
}

impl Default for AvifEncoder {
    fn default() -> Self {
        Self {
            options: Default::default(),
        }
    }
}

impl AvifEncoder {
    /// Create a new encoder
    pub fn new() -> AvifEncoder {
        AvifEncoder::default()
    }

    /// Create a new encoder with specified options
    pub fn new_with_options(options: AvifOptions) -> AvifEncoder {
        AvifEncoder { options }
    }
}

impl EncoderTrait for AvifEncoder {
    fn name(&self) -> &'static str {
        "avif"
    }

    fn encode_inner(&mut self, image: &zune_image::image::Image) -> Result<Vec<u8>, ImageErrors> {
        let (width, height) = image.dimensions();
        let data = &image.flatten_to_u8()[0];

        let encoder = ravif::Encoder::new()
            .with_quality(self.options.quality)
            .with_alpha_quality(self.options.alpha_quality.unwrap_or(self.options.quality))
            .with_speed(self.options.speed)
            .with_internal_color_space(self.options.color_space)
            .with_alpha_color_mode(self.options.alpha_color_mode);

        match image.colorspace() {
            ColorSpace::RGB => {
                let img = Img::new(data.as_slice().as_rgb(), width, height);
                let result = encoder
                    .encode_rgb(img)
                    .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?;

                Ok(result.avif_file)
            }
            ColorSpace::RGBA => {
                let img = Img::new(data.as_slice().as_rgba(), width, height);
                let result = encoder
                    .encode_rgba(img)
                    .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?;

                Ok(result.avif_file)
            }
            cs => Err(ImageErrors::EncodeErrors(
                ImgEncodeErrors::UnsupportedColorspace(cs, self.supported_colorspaces()),
            )),
        }
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[ColorSpace::RGB, ColorSpace::RGBA]
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Unknown
    }

    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Eight]
    }

    fn default_depth(&self, depth: BitDepth) -> BitDepth {
        match depth {
            _ => BitDepth::Eight,
        }
    }
}

#[cfg(test)]
mod tests;

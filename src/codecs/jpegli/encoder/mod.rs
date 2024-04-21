use std::mem;

use jpegli::qtable::QTable;
use zune_core::{bit_depth::BitDepth, colorspace::ColorSpace};
use zune_image::{codecs::ImageFormat, errors::ImageErrors, image::Image, traits::EncoderTrait};

/// Advanced options for Jpegli encoding
pub struct JpegliOptions {
    /// Quality, values 60-80 are recommended. `1..=100`
    pub quality: f32,
    /// Sets progressive mode for image
    pub progressive: bool,
    /// Set to false to make files larger for no reason
    pub optimize_coding: bool,
    /// If `1..=100` (non-zero), it will use Jpegli's smoothing.
    pub smoothing: u8,
    /// Set color space of JPEG being written, different from input color space
    pub color_space: jpegli::ColorSpace,
    /// Sets chroma subsampling, leave as `None` to use auto subsampling
    pub chroma_subsample: Option<u8>,
    /// Instead of quality setting, use a specific quantization table.
    pub luma_qtable: Option<QTable>,
    /// Instead of quality setting, use a specific quantization table for color.
    pub chroma_qtable: Option<QTable>,
}

/// A Jpegli encoder
#[derive(Default)]
pub struct JpegliEncoder {
    options: JpegliOptions,
}

impl Default for JpegliOptions {
    fn default() -> Self {
        Self {
            quality: 75.,
            progressive: true,
            optimize_coding: true,
            smoothing: 0,
            color_space: jpegli::ColorSpace::JCS_YCbCr,
            chroma_subsample: None,
            luma_qtable: None,
            chroma_qtable: None,
        }
    }
}

impl JpegliEncoder {
    /// Create a new encoder
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new encoder with specified options
    pub fn new_with_options(options: JpegliOptions) -> Self {
        Self { options }
    }
}

impl EncoderTrait for JpegliEncoder {
    fn name(&self) -> &'static str {
        "jpegli-encoder"
    }

    fn encode_inner(&mut self, image: &Image) -> Result<Vec<u8>, ImageErrors> {
        let (width, height) = image.dimensions();
        let data = &image.flatten_to_u8()[0];

        let luma_qtable = self.options.luma_qtable.as_ref();
        let chroma_qtable = self.options.chroma_qtable.as_ref();

        std::panic::catch_unwind(|| -> Result<Vec<u8>, ImageErrors> {
            let format = match image.colorspace() {
                ColorSpace::RGB => jpegli::ColorSpace::JCS_RGB,
                ColorSpace::RGBA => jpegli::ColorSpace::JCS_EXT_RGBA,
                ColorSpace::YCbCr => jpegli::ColorSpace::JCS_YCbCr,
                ColorSpace::Luma => jpegli::ColorSpace::JCS_GRAYSCALE,
                ColorSpace::YCCK => jpegli::ColorSpace::JCS_YCCK,
                ColorSpace::CMYK => jpegli::ColorSpace::JCS_CMYK,
                ColorSpace::BGR => jpegli::ColorSpace::JCS_EXT_BGR,
                ColorSpace::BGRA => jpegli::ColorSpace::JCS_EXT_BGRA,
                ColorSpace::ARGB => jpegli::ColorSpace::JCS_EXT_ARGB,
                ColorSpace::Unknown => jpegli::ColorSpace::JCS_UNKNOWN,
                _ => jpegli::ColorSpace::JCS_UNKNOWN,
            };

            let mut comp = jpegli::Compress::new(format);

            comp.set_size(width, height);
            comp.set_quality(self.options.quality);

            if self.options.progressive {
                comp.set_progressive_mode();
            }

            comp.set_optimize_coding(self.options.optimize_coding);
            comp.set_smoothing_factor(self.options.smoothing);
            comp.set_color_space(match format {
                jpegli::ColorSpace::JCS_GRAYSCALE => {
                    log::warn!("Input colorspace is GRAYSCALE, using GRAYSCALE as output");

                    jpegli::ColorSpace::JCS_GRAYSCALE
                }
                jpegli::ColorSpace::JCS_CMYK => {
                    log::warn!("Input colorspace is CMYK, using CMYK as output");

                    jpegli::ColorSpace::JCS_CMYK
                }
                jpegli::ColorSpace::JCS_YCCK => {
                    log::warn!("Input colorspace is YCCK, using YCCK as output");

                    jpegli::ColorSpace::JCS_YCCK
                }

                _ => self.options.color_space,
            });

            if let Some(sb) = self.options.chroma_subsample {
                comp.set_chroma_sampling_pixel_sizes((sb, sb), (sb, sb))
            }

            if let Some(qtable) = luma_qtable {
                comp.set_luma_qtable(qtable)
            }

            if let Some(qtable) = chroma_qtable {
                comp.set_chroma_qtable(qtable)
            }

            let mut comp = comp.start_compress(Vec::new())?;

            #[cfg(feature = "metadata")]
            {
                use exif::experimental::Writer;

                if let Some(metadata) = &image.metadata().exif() {
                    let mut writer = Writer::new();
                    // write first tags for exif
                    let mut buf = std::io::Cursor::new(b"Exif\x00\x00".to_vec());
                    // set buffer position to be bytes written, to ensure we don't overwrite anything
                    buf.set_position(6);

                    for metadatum in *metadata {
                        writer.push_field(metadatum);
                    }
                    let result = writer.write(&mut buf, false);
                    if result.is_ok() {
                        // add the exif tag to APP1 segment
                        comp.write_marker(jpegli::Marker::APP(1), buf.get_ref());
                    } else {
                        log::warn!("Writing exif failed {:?}", result);
                    }
                }
            }

            comp.write_scanlines(data)?;

            Ok(comp.finish()?)
        })
        .map_err(|err| {
            if let Ok(mut err) = err.downcast::<String>() {
                ImageErrors::EncodeErrors(zune_image::errors::ImgEncodeErrors::Generic(mem::take(
                    &mut *err,
                )))
            } else {
                ImageErrors::EncodeErrors(zune_image::errors::ImgEncodeErrors::GenericStatic(
                    "Unknown error occurred during encoding",
                ))
            }
        })?
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[
            ColorSpace::Luma,
            ColorSpace::RGBA,
            ColorSpace::RGB,
            ColorSpace::YCCK,
            ColorSpace::CMYK,
            ColorSpace::BGR,
            ColorSpace::BGRA,
            ColorSpace::ARGB,
            ColorSpace::YCbCr,
        ]
    }

    fn format(&self) -> zune_image::codecs::ImageFormat {
        ImageFormat::JPEG
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

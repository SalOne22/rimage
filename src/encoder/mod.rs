use image::{ColorType, DynamicImage, ImageBuffer, ImageResult};
use rgb::FromSlice;
use std::io::Write;

use crate::config::ResizeType;
use crate::{config::EncoderConfig, error::EncoderError};

/// A struct for encoding images using various codecs.
pub struct Encoder<W: Write> {
    w: W,
    data: DynamicImage,
    conf: EncoderConfig,
}

impl<W: Write + std::panic::UnwindSafe> Encoder<W> {
    /// Creates a new [`Encoder`] instance with the specified writer and image data.
    ///
    /// # Parameters
    ///
    /// - `w`: The writer to which the encoded image will be written.
    /// - `data`: The image data to be encoded.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::Encoder;
    /// # use std::fs::File;
    /// use image::{DynamicImage, RgbaImage};
    /// # let image_data = vec![0; 800 * 600 * 4];
    ///
    /// let image = RgbaImage::from_raw(800, 600, image_data).unwrap();
    ///
    /// let file = File::create("output.jpg").expect("Failed to create file");
    ///
    /// let encoder = Encoder::new(file, DynamicImage::ImageRgba8(image)); // uses default config
    /// ```
    #[inline]
    pub fn new(w: W, data: DynamicImage) -> Self {
        Self {
            w,
            data,
            conf: EncoderConfig::default(),
        }
    }

    /// Configures the encoder with the specified [`EncoderConfig`].
    ///
    /// # Parameters
    ///
    /// - `conf`: The configuration to use for encoding.
    ///
    /// # Returns
    ///
    /// Returns a modified [`Encoder`] instance with the updated configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rimage::rgb::RGBA8;
    /// use rimage::{Encoder, config::{EncoderConfig, Codec}};
    /// # use std::fs::File;
    /// # use std::fs;
    /// use image::{DynamicImage, RgbaImage};
    /// # let image_data = vec![0; 800 * 600 * 4];
    ///
    /// let image = RgbaImage::from_raw(800, 600, image_data).unwrap();
    ///
    /// let file = File::create("output.png").expect("Failed to create file");
    ///
    /// let config = EncoderConfig::new(Codec::Png)
    ///     .with_quality(90.0)
    ///     .unwrap();
    ///
    /// let encoder = Encoder::new(file, DynamicImage::ImageRgba8(image)).with_config(config);
    ///
    /// # fs::remove_file("output.png").unwrap_or(());
    /// # Ok::<(), rimage::error::EncoderError>(())
    /// ```
    #[inline]
    pub fn with_config(mut self, conf: EncoderConfig) -> Self {
        self.conf = conf;
        self
    }

    /// Encodes the image using the configured settings.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful encoding or an [`EncoderError`] on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::{Encoder, config::EncoderConfig};
    /// # use std::fs;
    /// use std::fs::File;
    /// use image::{DynamicImage, RgbaImage};
    ///
    /// let image_data = vec![0; 800 * 600 * 4];
    /// let image = RgbaImage::from_raw(800, 600, image_data).unwrap();
    ///
    /// let file = File::create("output.jpg").expect("Failed to create file");
    ///
    /// let config = EncoderConfig::default();
    ///
    /// let encoder = Encoder::new(file, DynamicImage::ImageRgba8(image)).with_config(config);
    ///
    /// encoder.encode()?;
    /// # fs::remove_file("output.jpg")?;
    /// # Ok::<(), rimage::error::EncoderError>(())
    /// ```
    #[allow(unused_mut)]
    // TODO: Change Error type to ImageError
    pub fn encode(mut self) -> Result<(), EncoderError> {
        // TODO: Move resize out from encoder to operations
        #[cfg(feature = "resizing")]
        if let Some(resize_config) = self.conf.resize_config() {
            let aspect_ratio = self.data.width() as f64 / self.data.height() as f64;

            let width = resize_config.width().unwrap_or(
                resize_config
                    .height()
                    .map(|h| (h as f64 * aspect_ratio) as usize)
                    .unwrap_or(self.data.width() as usize),
            );
            let height = resize_config.height().unwrap_or(
                resize_config
                    .width()
                    .map(|w| (w as f64 / aspect_ratio) as usize)
                    .unwrap_or(self.data.height() as usize),
            );

            let filter = match resize_config.filter_type() {
                ResizeType::Point => image::imageops::Nearest,
                ResizeType::Triangle => image::imageops::Triangle,
                ResizeType::CatmullRom => image::imageops::CatmullRom,
                ResizeType::Mitchell => image::imageops::Gaussian, // TODO: rename Mitchell to Gaussian
                ResizeType::Lanczos3 => image::imageops::Lanczos3,
            };

            self.data.resize(width as u32, height as u32, filter);
        }

        // TODO: Move quantization out from encoder to operations
        #[cfg(feature = "quantization")]
        if let Some(quantization_config) = self.conf.quantization_config() {
            let mut image = self.data.to_rgba8();

            let pixels = image.as_raw();

            let mut liq = imagequant::new();

            liq.set_speed(5)?;
            liq.set_quality(0, quantization_config.quality())?;

            let mut img = liq.new_image_borrowed(
                pixels.as_rgba(),
                image.width() as usize,
                image.height() as usize,
                0.0,
            )?;

            let mut res = liq.quantize(&mut img)?;

            res.set_dithering_level(quantization_config.dithering_level())?;

            let (palette, pixels) = res.remapped(&mut img)?;

            self.data = DynamicImage::ImageRgba8(
                ImageBuffer::from_raw(
                    image.width(),
                    image.height(),
                    pixels
                        .iter()
                        .flat_map(|pix| palette[*pix as usize].iter())
                        .collect::<Vec<u8>>(),
                )
                .ok_or(EncoderError::General)?,
            );
        }

        match self.conf.codec() {
            crate::config::Codec::MozJpeg => self.encode_mozjpeg(),
            crate::config::Codec::Png => self.encode_png(),
            #[cfg(feature = "jxl")]
            crate::config::Codec::JpegXl => self.encode_jpegxl(),
            #[cfg(feature = "oxipng")]
            crate::config::Codec::OxiPng => self.encode_oxipng(),
            #[cfg(feature = "webp")]
            crate::config::Codec::WebP => self.encode_webp(),
            #[cfg(feature = "avif")]
            crate::config::Codec::Avif => self.encode_avif(),
        }
    }

    fn encode_mozjpeg(self) -> Result<(), EncoderError> {
        let width = self.data.width();
        let height = self.data.height();
        let quality = self.conf.quality();

        std::panic::catch_unwind(|| -> Result<(), EncoderError> {
            let format = match self.data.color() {
                ColorType::L8 | ColorType::L16 => mozjpeg::ColorSpace::JCS_GRAYSCALE,
                ColorType::La8 | ColorType::La16 => mozjpeg::ColorSpace::JCS_GRAYSCALE,
                ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgb32F => {
                    mozjpeg::ColorSpace::JCS_RGB
                }
                ColorType::Rgba8 | ColorType::Rgba16 | ColorType::Rgba32F => {
                    mozjpeg::ColorSpace::JCS_EXT_RGBA
                }
                _ => mozjpeg::ColorSpace::JCS_EXT_RGBA,
            };

            let data = match self.data.color() {
                ColorType::La8 | ColorType::La16 => {
                    DynamicImage::ImageLuma8(self.data.into_luma8())
                }
                _ => self.data,
            };

            let mut comp = mozjpeg::Compress::new(format);

            comp.set_size(width as usize, height as usize);
            comp.set_quality(quality);
            comp.set_progressive_mode();

            let mut comp = comp.start_compress(self.w)?;

            comp.write_scanlines(data.as_bytes())?;

            comp.finish()?;

            Ok(())
        })
        .map_err(|_| EncoderError::General)?
    }

    fn encode_png(self) -> Result<(), EncoderError> {
        let width = self.data.width();
        let height = self.data.height();

        let mut encoder = png::Encoder::new(self.w, width, height);

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(self.data.as_bytes())?;
        writer.finish()?;

        Ok(())
    }

    #[cfg(feature = "jxl")]
    fn encode_jpegxl(mut self) -> Result<(), EncoderError> {
        unimplemented!()
    }

    #[cfg(feature = "oxipng")]
    fn encode_oxipng(mut self) -> Result<(), EncoderError> {
        let width = self.data.width();
        let height = self.data.height();

        let options = oxipng::Options::default();

        let img = oxipng::RawImage::new(
            width,
            height,
            oxipng::ColorType::RGBA,
            oxipng::BitDepth::Eight,
            self.data.as_bytes().to_vec(),
        )?;

        self.w.write_all(&img.create_optimized_png(&options)?)?;

        Ok(())
    }

    #[cfg(feature = "webp")]
    fn encode_webp(mut self) -> Result<(), EncoderError> {
        let width = self.data.width();
        let height = self.data.height();

        let encoder = webp::Encoder::from_rgba(self.data.as_bytes(), width, height);

        self.w.write_all(&encoder.encode(self.conf.quality()))?;
        self.w.flush()?;

        Ok(())
    }

    #[cfg(feature = "avif")]
    fn encode_avif(mut self) -> Result<(), EncoderError> {
        let width = self.data.width();
        let height = self.data.height();

        let img = ravif::Encoder::new()
            .with_quality(self.conf.quality())
            .with_speed(4)
            .encode_rgba(ravif::Img::new(
                self.data.into_rgba8().as_rgba(),
                width as usize,
                height as usize,
            ))?;

        self.w.write_all(&img.avif_file)?;
        self.w.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;

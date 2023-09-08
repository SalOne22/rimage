use std::io::Write;

use rgb::ComponentBytes;

use crate::{config::EncoderConfig, error::EncoderError, image::Image};

/// A struct for encoding images using various codecs.
pub struct Encoder<W: Write> {
    w: W,
    data: Image,
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
    /// # use rimage::rgb::RGBA8;
    /// use rimage::{Encoder, Image};
    /// # use std::fs::File;
    /// # let image_data = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    ///
    /// let image = Image::new(image_data, 800, 600);
    ///
    /// let file = File::create("output.jpg").expect("Failed to create file");
    ///
    /// let encoder = Encoder::new(file, image); // uses default config
    /// ```
    #[inline]
    pub fn new(w: W, data: Image) -> Self {
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
    /// use rimage::{Encoder, Image, config::{EncoderConfig, Codec}};
    /// # use std::fs::File;
    /// # let image_data = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    ///
    /// let image = Image::new(image_data, 800, 600);
    ///
    /// let file = File::create("output.jpg").expect("Failed to create file");
    ///
    /// let config = EncoderConfig::new(Codec::WebP)
    ///     .with_quality(90.0)
    ///     .unwrap();
    ///
    /// let encoder = Encoder::new(file, image).with_config(config);
    ///
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
    /// use rimage::rgb::RGBA8;
    /// use rimage::{Encoder, Image, config::EncoderConfig};
    /// # use std::fs;
    /// use std::fs::File;
    ///
    /// let image_data = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    /// let image = Image::new(image_data, 800, 600);
    ///
    /// let file = File::create("output.jpg").expect("Failed to create file");
    ///
    /// let config = EncoderConfig::default();
    ///
    /// let encoder = Encoder::new(file, image).with_config(config);
    ///
    /// encoder.encode()?;
    /// # fs::remove_file("output.jpg")?;
    /// # Ok::<(), rimage::error::EncoderError>(())
    /// ```
    #[allow(unused_mut)]
    pub fn encode(mut self) -> Result<(), EncoderError> {
        #[cfg(feature = "resizing")]
        if let Some(resize_config) = self.conf.resize_config() {
            self.data.resize(resize_config)?;
        }

        #[cfg(feature = "quantization")]
        if let Some(quantization_config) = self.conf.quantization_config() {
            self.data.quantize(quantization_config)?;
        }

        match self.conf.codec() {
            crate::config::Codec::MozJpeg => self.encode_mozjpeg(),
            crate::config::Codec::JpegXl => self.encode_jpegxl(),
            crate::config::Codec::Png => self.encode_png(),
            crate::config::Codec::OxiPng => self.encode_oxipng(),
            crate::config::Codec::WebP => self.encode_webp(),
            crate::config::Codec::Avif => self.encode_avif(),
        }
    }

    fn encode_mozjpeg(self) -> Result<(), EncoderError> {
        let width = self.data.width();
        let height = self.data.height();
        let quality = self.conf.quality();

        std::panic::catch_unwind(|| -> Result<(), EncoderError> {
            let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGBA);

            comp.set_size(width, height);
            comp.set_quality(quality);
            comp.set_progressive_mode();

            let mut comp = comp.start_compress(self.w)?;

            comp.write_scanlines(self.data.data().as_bytes())?;

            comp.finish()?;

            Ok(())
        })
        .map_err(|_| EncoderError::General)?
    }

    fn encode_jpegxl(mut self) -> Result<(), EncoderError> {
        let mut encoder = jpegxl_rs::encoder_builder()
            .quality(self.conf.quality())
            .speed(jpegxl_rs::encode::EncoderSpeed::Falcon)
            .build()?;

        let buf: jpegxl_rs::encode::EncoderResult<u8> = encoder.encode(
            self.data.data().as_bytes(),
            self.data.width().try_into()?,
            self.data.height().try_into()?,
        )?;

        self.w.write_all(&buf)?;
        self.w.flush()?;

        Ok(())
    }

    fn encode_png(self) -> Result<(), EncoderError> {
        let width: u32 = self.data.width().try_into()?;
        let height: u32 = self.data.height().try_into()?;

        let mut encoder = png::Encoder::new(self.w, width, height);

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(self.data.data().as_bytes())?;
        writer.finish()?;

        Ok(())
    }

    fn encode_oxipng(mut self) -> Result<(), EncoderError> {
        let width: u32 = self.data.width().try_into()?;
        let height: u32 = self.data.height().try_into()?;

        let options = oxipng::Options::default();

        let img = oxipng::RawImage::new(
            width,
            height,
            oxipng::ColorType::RGBA,
            oxipng::BitDepth::Eight,
            self.data.data().as_bytes().to_vec(),
        )?;

        self.w.write_all(&img.create_optimized_png(&options)?)?;

        Ok(())
    }

    fn encode_webp(mut self) -> Result<(), EncoderError> {
        let width: u32 = self.data.width().try_into()?;
        let height: u32 = self.data.height().try_into()?;

        let encoder = webp::Encoder::from_rgba(self.data.data().as_bytes(), width, height);

        self.w.write_all(&encoder.encode(self.conf.quality()))?;
        self.w.flush()?;

        Ok(())
    }

    fn encode_avif(mut self) -> Result<(), EncoderError> {
        let width = self.data.width();
        let height = self.data.height();

        let img = ravif::Encoder::new()
            .with_quality(self.conf.quality())
            .with_speed(4)
            .encode_rgba(ravif::Img::new(self.data.data(), width, height))?;

        self.w.write_all(&img.avif_file)?;
        self.w.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;

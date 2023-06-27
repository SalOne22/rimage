use std::panic;

use log::info;
use rgb::{AsPixels, ComponentBytes, FromSlice, RGBA};
use simple_error::SimpleError;

use crate::{error::EncodingError, Config, ImageData, OutputFormat, ResizeType};

/// Encoder for images
///
/// # Example
/// ```
/// # use rimage::{Encoder, Config, ImageData, OutputFormat};
/// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
/// # let file = std::fs::File::open(&path).unwrap();
/// # let decoder = rimage::Decoder::new(&path, file);
/// # let image = decoder.decode().unwrap();
/// let config = Config::default();
///
/// // image is ImageData
/// let encoder = Encoder::new(&config, image);
/// let result = encoder.encode();
/// assert!(result.is_ok());
/// ```
pub struct Encoder<'a> {
    image_data: ImageData,
    config: &'a Config,
}

impl<'a> Encoder<'a> {
    /// Create new encoder
    ///
    /// # Example
    /// ```
    /// # use rimage::{Encoder, Config, ImageData, OutputFormat};
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let file = std::fs::File::open(&path).unwrap();
    /// # let decoder = rimage::Decoder::new(&path, file);
    /// # let image = decoder.decode().unwrap();
    /// let config = Config::default();
    /// let encoder = Encoder::new(&config, image); // where image is ImageData
    /// ```
    pub fn new(conf: &'a Config, image_data: ImageData) -> Self {
        Encoder {
            image_data,
            config: conf,
        }
    }
    /// Encode image
    ///
    /// # Example
    /// ```
    /// # use rimage::{Encoder, Config, ImageData, OutputFormat};
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let file = std::fs::File::open(&path).unwrap();
    /// # let decoder = rimage::Decoder::new(&path, file);
    /// # let image = decoder.decode().unwrap();
    /// let config = Config::default();
    /// let encoder = Encoder::new(&config, image); // where image is ImageData
    /// let result = encoder.encode();
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`EncodingError`] if encoding failed
    pub fn encode(mut self) -> Result<Vec<u8>, EncodingError> {
        if self.config.target_height.is_some() || self.config.target_width.is_some() {
            info!("Resizing image");
            self.resize()?;
        }

        if self.config.quantization_quality.is_some() || self.config.dithering_level.is_some() {
            let quantization = self.config.quantization_quality.unwrap_or(100);
            let dithering = self.config.dithering_level.unwrap_or(1.0);

            info!(
                "Quantizing to {} with dithering {}",
                quantization, dithering
            );

            self.quantize(quantization, dithering)?;
        }

        match self.config.output_format {
            OutputFormat::Png => self.encode_png(),
            OutputFormat::Oxipng => self.encode_oxipng(),
            OutputFormat::MozJpeg => self.encode_mozjpeg(),
            OutputFormat::WebP => self.encode_webp(),
            OutputFormat::Avif => self.encode_avif(),
        }
    }

    /// Encode image with quantization
    ///
    /// # Example
    /// ```
    /// # use rimage::{Encoder, Config, ImageData, OutputFormat};
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let file = std::fs::File::open(&path).unwrap();
    /// # let decoder = rimage::Decoder::new(&path, file);
    /// # let image = decoder.decode().unwrap();
    /// let config = Config::default();
    /// let encoder = Encoder::new(&config, image); // where image is ImageData
    /// let result = encoder.encode_quantized(75, 1.0);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`EncodingError`] if encoding failed
    pub fn encode_quantized(
        mut self,
        quantization_quality: u8,
        dithering_level: f32,
    ) -> Result<Vec<u8>, EncodingError> {
        if self.config.target_height.is_some() || self.config.target_width.is_some() {
            self.resize()?;
        }

        self.quantize(quantization_quality, dithering_level)?;

        match self.config.output_format {
            OutputFormat::Png => self.encode_png(),
            OutputFormat::Oxipng => self.encode_oxipng(),
            OutputFormat::MozJpeg => self.encode_mozjpeg(),
            OutputFormat::WebP => self.encode_webp(),
            OutputFormat::Avif => self.encode_avif(),
        }
    }

    fn resize(&mut self) -> Result<(), EncodingError> {
        let (width, height) = self.image_data.size();
        let aspect_ratio = width as f32 / height as f32;
        info!("Aspect ratio: {}", aspect_ratio);
        info!("Original size: {}x{}", width, height);

        // If target width or height is not set, calculate it from the other
        // or use the original size
        let target_width = self.config.target_width.unwrap_or(
            self.config
                .target_height
                .map(|h| (h as f32 * aspect_ratio) as usize)
                .unwrap_or(width),
        );
        let target_height = self.config.target_height.unwrap_or(
            self.config
                .target_width
                .map(|w| (w as f32 / aspect_ratio) as usize)
                .unwrap_or(height),
        );

        info!("Target size: {}x{}", target_width, target_height);
        info!(
            "Resize type: {:?}",
            self.config
                .resize_type
                .as_ref()
                .unwrap_or(&ResizeType::Lanczos3)
        );

        let mut dest = vec![RGBA::new(0, 0, 0, 0); target_width * target_height];

        let mut resizer = resize::new(
            width,
            height,
            target_width,
            target_height,
            resize::Pixel::RGBA8,
            resize::Type::from(
                self.config
                    .resize_type
                    .as_ref()
                    .unwrap_or(&ResizeType::Lanczos3),
            ),
        )?;

        resizer.resize(self.image_data.data().as_rgba(), &mut dest)?;

        self.image_data = ImageData::new(target_width, target_height, dest.as_bytes());

        Ok(())
    }

    fn quantize(
        &mut self,
        quantization_quality: u8,
        dithering_level: f32,
    ) -> Result<(), EncodingError> {
        let mut liq = imagequant::new();

        liq.set_speed(5)?;
        liq.set_quality(0, quantization_quality)?;

        let mut img = liq.new_image(
            self.image_data.data().as_pixels(),
            self.image_data.size().0,
            self.image_data.size().1,
            0.0,
        )?;

        let mut res = liq.quantize(&mut img)?;

        res.set_dithering_level(dithering_level)?;

        let (palette, pixels) = res.remapped(&mut img)?;

        let mut data = Vec::with_capacity(pixels.len() * 4);

        pixels.iter().for_each(|pix| {
            let color = palette[*pix as usize];
            data.extend_from_slice(&[color.r, color.g, color.b, color.a]);
        });

        self.image_data = ImageData::new(self.image_data.size().0, self.image_data.size().1, &data);

        Ok(())
    }

    fn encode_mozjpeg(self) -> Result<Vec<u8>, EncodingError> {
        info!("Encoding with mozjpeg");
        panic::catch_unwind(|| -> Result<Vec<u8>, EncodingError> {
            let mut encoder = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGBA);

            encoder.set_size(self.image_data.size().0, self.image_data.size().1);
            encoder.set_quality(self.config.quality);
            encoder.set_progressive_mode();
            encoder.set_mem_dest();
            encoder.start_compress();
            encoder.write_scanlines(self.image_data.data());
            encoder.finish_compress();

            encoder.data_to_vec().map_err(|_| {
                EncodingError::Encoding(Box::new(SimpleError::new("Failed to convert data to vec")))
            })
        })
        .unwrap_or_else(|e| {
            Err(EncodingError::Encoding(Box::new(SimpleError::new(
                format!("Failed to encode image: {:?}", e),
            ))))
        })
    }

    fn encode_png(&self) -> Result<Vec<u8>, EncodingError> {
        info!("Encoding PNG");
        let mut buf = Vec::new();

        {
            let mut encoder = png::Encoder::new(
                &mut buf,
                self.image_data.size().0 as u32,
                self.image_data.size().1 as u32,
            );

            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);

            let mut writer = encoder.write_header()?;
            writer.write_image_data(self.image_data.data())?;
            writer.finish()?;
        }

        info!("Encoded {} bytes", buf.len());

        Ok(buf)
    }

    fn encode_oxipng(&self) -> Result<Vec<u8>, EncodingError> {
        info!("Encoding with OxiPNG");
        let mut buf = Vec::new();

        {
            let mut encoder = png::Encoder::new(
                &mut buf,
                self.image_data.size().0 as u32,
                self.image_data.size().1 as u32,
            );

            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);

            let mut writer = encoder.write_header()?;
            writer.write_image_data(self.image_data.data())?;
            writer.finish()?;
        }

        info!("Encoded {} bytes (Not optimized)", buf.len());

        oxipng::optimize_from_memory(&buf, &oxipng::Options::default())
            .map_err(|e| EncodingError::Encoding(Box::new(e)))
    }

    fn encode_webp(&self) -> Result<Vec<u8>, EncodingError> {
        info!("Encoding with WebP");
        let (width, height) = self.image_data.size();

        let data = libwebp::WebPEncodeRGBA(
            self.image_data.data(),
            width as u32,
            height as u32,
            (width * 4) as u32,
            self.config.quality,
        )
        .map_err(|e| EncodingError::Encoding(Box::new(e)))?;

        Ok(data.to_owned())
    }

    fn encode_avif(&self) -> Result<Vec<u8>, EncodingError> {
        info!("Encoding with AVIF");

        let (width, height) = self.image_data.size();
        let data = ravif::Img::new(self.image_data.data().as_rgba(), width, height);

        Ok(ravif::Encoder::new()
            .with_quality(self.config.quality)
            .with_speed(6)
            .encode_rgba(data)
            .map_err(|e| EncodingError::Encoding(Box::new(e)))?
            .avif_file)
    }
}

#[cfg(test)]
mod tests;

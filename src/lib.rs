/*!
This crate provides a cli tool and library for image processing.
Similar to [squoosh!](https://squoosh.app/) using same codecs,
but fully written on rust and with bulk processing support.

Current features:
- Decoding jpeg and png
- Encoding with optimizations
- Get image information

# Usage

First add this crate to your dependencies:
```text
cargo add rimage
```

or add this to Cargo.toml:
```toml
[dependencies]
rimage = "0.2"
```

After that you can use this crate:

## Decoding
```
use rimage::Decoder;

// Create decoder from file path and data
let path = std::path::PathBuf::from("tests/files/basi0g01.jpg"); // Or any other image
let file = std::fs::File::open(&path).unwrap();
let decoder = Decoder::new(&path, file);

// Decode image to image data
let image = match decoder.decode() {
    Ok(img) => img,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Get image data
println!("Size: {:?}", image.size());
println!("Data length: {:?}", image.data().len());

// Do something with image...
```

## Encoding

```
# use rimage::Decoder;
use rimage::{Config, Encoder, OutputFormat};
# let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
# let file = std::fs::File::open(&path).unwrap();
# let decoder = Decoder::new(&path, file);
# let image = decoder.decode().unwrap();

// Build config for encoding
let config = match Config::build(
    75.0,
    OutputFormat::MozJpeg,
    None,
    None,
    None,
) {
    Ok(config) => config,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

let encoder = Encoder::new(&config, image); // where image is image::ImageData

// Get encoded image data from encoder
let data = match encoder.encode() {
    Ok(data) => data,
    Err(e) => {
        eprintln!("Oh no, there is error! {e}");
        std::process::exit(1);
    }
};

// Write image to file
std::fs::write("output.jpg", data);
# std::fs::remove_file("output.jpg").unwrap();
```
*/
#![warn(missing_docs)]

use error::{ConfigError, DecodingError, EncodingError};
use log::info;
use rgb::{
    alt::{GRAY8, GRAYA8},
    AsPixels, ComponentBytes, FromSlice, RGB8, RGBA, RGBA8,
};
use simple_error::SimpleError;
use std::{ffi::CString, fs, io::Read, panic, path};

pub use image::{ImageData, OutputFormat, ResizeType};

/// Decoders for images
#[deprecated(since = "0.2.0", note = "use the Decoder struct instead")]
pub mod decoders;
/// Encoders for images
#[deprecated(since = "0.2.0", note = "use the Encoder struct instead")]
pub mod encoders;

/// Errors that can occur during image processing
pub mod error;

/// Image data structs
pub mod image;

/// Config for image encoding
///
/// # Example
/// ```
/// use rimage::{Config, OutputFormat, ResizeType};
///
/// // Without resize
/// let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
///
/// // With resize
/// let config_resize = Config::build(75.0, OutputFormat::MozJpeg, Some(200), Some(200), Some(ResizeType::Lanczos3)).unwrap();
/// ```
///
/// # Default
/// ```
/// use rimage::{Config, OutputFormat};
///
/// let config = Config::default();
/// assert_eq!(config.quality(), 75.0);
/// assert_eq!(config.output_format(), &OutputFormat::MozJpeg);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    quality: f32,
    output_format: OutputFormat,
    target_width: Option<usize>,
    target_height: Option<usize>,
    resize_type: Option<ResizeType>,
}

impl Config {
    /// Create new config
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat, ResizeType};
    ///
    /// // Without resize
    /// let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
    ///
    /// // With resize
    /// let config_resize = Config::build(75.0, OutputFormat::MozJpeg, Some(200), Some(200), Some(ResizeType::Lanczos3)).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If quality is not in range 0.0..=100.0
    ///
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(200.0, OutputFormat::MozJpeg, None, None, None);
    /// assert!(config.is_err());
    /// ```
    pub fn build(
        quality: f32,
        output_format: OutputFormat,
        width: Option<usize>,
        height: Option<usize>,
        resize_type: Option<ResizeType>,
    ) -> Result<Self, ConfigError> {
        if !(0.0..=100.0).contains(&quality) {
            return Err(ConfigError::QualityOutOfBounds);
        }

        if let Some(width) = width {
            if width == 0 {
                return Err(ConfigError::WidthIsZero);
            }
        }
        if let Some(height) = height {
            if height == 0 {
                return Err(ConfigError::HeightIsZero);
            }
        }

        Ok(Config {
            quality,
            output_format,
            target_width: width,
            target_height: height,
            resize_type,
        })
    }
    /// Get quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
    /// assert_eq!(config.quality(), 75.0);
    /// ```
    #[inline]
    pub fn quality(&self) -> f32 {
        self.quality
    }
    /// Get output format
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
    /// assert_eq!(config.output_format(), &OutputFormat::MozJpeg);
    /// ```
    #[inline]
    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            quality: 75.0,
            output_format: OutputFormat::MozJpeg,
            target_width: None,
            target_height: None,
            resize_type: Some(ResizeType::Lanczos3),
        }
    }
}

/// Decoder for images
///
/// # Example
/// ```
/// # use rimage::Decoder;
/// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
/// let file = std::fs::File::open(&path).unwrap();
///
/// let decoder = Decoder::new(&path, file);
///
/// // Decode image to image data
/// let image = match decoder.decode() {
///     Ok(img) => img,
///     Err(e) => {
///         eprintln!("Oh no there is error! {e}");
///         std::process::exit(1);
///     }
/// };
/// ```
pub struct Decoder<'a> {
    path: &'a path::Path,
    file: fs::File,
}

impl<'a> Decoder<'a> {
    /// Create new decoder
    ///
    /// # Example
    /// ```
    /// # use rimage::Decoder;
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// let file = std::fs::File::open(&path).unwrap();
    ///
    /// let decoder = Decoder::new(&path, file);
    /// ```
    #[inline]
    pub fn new(path: &'a path::Path, file: fs::File) -> Self {
        Decoder { path, file }
    }

    /// Decode image
    ///
    /// # Example
    /// ```
    /// # use rimage::Decoder;
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let file = std::fs::File::open(&path).unwrap();
    /// # let decoder = Decoder::new(&path, file);
    /// // Decode image to image data
    /// let image = match decoder.decode() {
    ///     Ok(img) => img,
    ///     Err(e) => {
    ///         eprintln!("Oh no there is error! {e}");
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// // Do something with image data...
    /// ```
    ///
    /// # Errors
    ///
    /// If image format is not supported
    ///
    /// ```
    /// # use rimage::Decoder;
    /// let path = std::path::PathBuf::from("tests/files/test.bmp");
    /// let file = std::fs::File::open(&path).unwrap();
    ///
    /// let decoder = Decoder::new(&path, file);
    ///
    /// let result = decoder.decode();
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().to_string(), "Format Error: bmp not supported");
    /// ```
    ///
    /// If image format is supported but there is error during decoding
    ///
    /// ```
    /// # use rimage::Decoder;
    /// let path = std::path::PathBuf::from("tests/files/test_corrupted.jpg");
    /// let file = std::fs::File::open(&path).unwrap();
    ///
    /// let decoder = Decoder::new(&path, file);
    ///
    /// let result = decoder.decode();
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().to_string(), "Parsing Error: Failed to decode jpeg");
    /// ```
    pub fn decode(self) -> Result<ImageData, DecodingError> {
        let extension = match self.path.extension() {
            Some(ext) => ext,
            None => {
                return Err(DecodingError::Format(Box::new(SimpleError::new(
                    "No extension",
                ))))
            }
        };

        match extension.to_str() {
            Some("jpg") | Some("jpeg") => self.decode_jpeg(),
            Some("png") => self.decode_png(),
            Some("webp") => self.decode_webp(),
            Some("avif") => self.decode_avif(),
            Some(ext) => Err(DecodingError::Format(Box::new(SimpleError::new(format!(
                "{} not supported",
                ext
            ))))),
            None => Err(DecodingError::Parsing(Box::new(SimpleError::new(
                "Failed to get extension",
            )))),
        }
    }

    // mut for not unix case
    #[allow(unused_mut)]
    fn decode_jpeg(mut self) -> Result<ImageData, DecodingError> {
        info!("Processing jpeg decoding");
        panic::catch_unwind(move || -> Result<ImageData, DecodingError> {
            #[cfg(unix)]
            let d = mozjpeg::Decompress::new_file(self.file)?;
            #[cfg(not(unix))]
            let buf = {
                let metadata = self.file.metadata()?;
                let mut buf = Vec::with_capacity(metadata.len() as usize);
                self.file.read_to_end(&mut buf)?;
                buf
            };
            #[cfg(not(unix))]
            let d = mozjpeg::Decompress::new_mem(&buf)?;

            let mut image = d.rgba()?;

            let data: Vec<RGBA8> =
                image
                    .read_scanlines()
                    .ok_or(DecodingError::Parsing(Box::new(SimpleError::new(
                        "Failed to read scanlines",
                    ))))?;

            info!("JPEG Color space: {:?}", image.color_space());
            info!("Dimensions: {}x{}", image.width(), image.height());

            Ok(ImageData::new(
                image.width(),
                image.height(),
                data.as_bytes(),
            ))
        })
        .unwrap_or(Err(DecodingError::Parsing(Box::new(SimpleError::new(
            "Failed to decode jpeg",
        )))))
    }

    fn expand_pixels<T: Copy>(buf: &mut [u8], to_rgba: impl Fn(T) -> RGBA8)
    where
        [u8]: AsPixels<T> + FromSlice<u8>,
    {
        assert!(std::mem::size_of::<T>() <= std::mem::size_of::<RGBA8>());
        for i in (0..buf.len() / 4).rev() {
            let src_pix = buf.as_pixels()[i];
            buf.as_rgba_mut()[i] = to_rgba(src_pix);
        }
    }

    fn decode_png(self) -> Result<ImageData, DecodingError> {
        info!("Processing png decoding");
        let mut d = png::Decoder::new(self.file);
        d.set_transformations(png::Transformations::normalize_to_color8());

        let mut reader = d.read_info()?;
        let width = reader.info().width;
        let height = reader.info().height;

        let buf_size = width as usize * height as usize * 4;
        let mut buf = vec![0; buf_size];

        let info = reader.next_frame(&mut buf)?;

        info!("PNG Color type: {:?}", info.color_type);
        info!("Dimensions: {}x{}", width, height);

        match info.color_type {
            png::ColorType::Grayscale => Self::expand_pixels(&mut buf, |gray: GRAY8| gray.into()),
            png::ColorType::GrayscaleAlpha => Self::expand_pixels(&mut buf, GRAYA8::into),
            png::ColorType::Rgb => Self::expand_pixels(&mut buf, RGB8::into),
            png::ColorType::Rgba => {}
            png::ColorType::Indexed => {
                return Err(DecodingError::Parsing(Box::new(SimpleError::new(
                    "Indexed color type is not supported",
                ))))
            }
        }

        Ok(ImageData::new(width as usize, height as usize, &buf))
    }

    fn decode_webp(mut self) -> Result<ImageData, DecodingError> {
        let metadata = self.file.metadata()?;
        let mut buf = Vec::with_capacity(metadata.len() as usize);
        self.file.read_to_end(&mut buf)?;
        let (width, height, buf) = libwebp::WebPDecodeRGBA(&buf)?;

        Ok(ImageData::new(width as usize, height as usize, &buf))
    }

    fn decode_avif(&self) -> Result<ImageData, DecodingError> {
        use libavif_sys::*;

        let image = unsafe { avifImageCreateEmpty() };
        let decoder = unsafe { avifDecoderCreate() };
        let decode_result = unsafe {
            avifDecoderReadFile(
                decoder,
                image,
                CString::new(self.path.to_str().unwrap())
                    .map_err(|e| DecodingError::Parsing(Box::new(e)))?
                    .as_ptr(),
            )
        };
        unsafe { avifDecoderDestroy(decoder) };

        let mut result = Err(DecodingError::Parsing(Box::new(SimpleError::new(
            "Failed to decode avif",
        ))));

        if decode_result == AVIF_RESULT_OK {
            let mut rgb: avifRGBImage = Default::default();
            unsafe { avifRGBImageSetDefaults(&mut rgb, image) };
            rgb.depth = 8;

            unsafe {
                avifRGBImageAllocatePixels(&mut rgb);
                avifImageYUVToRGB(image, &mut rgb);
            };

            let pixels = unsafe {
                std::slice::from_raw_parts(rgb.pixels, (rgb.width * rgb.height * 4) as usize)
            };

            result = Ok(ImageData::new(
                rgb.width as usize,
                rgb.height as usize,
                pixels,
            ));

            unsafe { avifRGBImageFreePixels(&mut rgb) };
        }

        unsafe {
            avifImageDestroy(image);
        };

        result
    }
}

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
mod tests {
    use std::fs;

    use regex::Regex;

    use super::*;

    #[test]
    fn config_edge_cases() {
        let config = Config::default();
        assert_eq!(config.output_format, OutputFormat::MozJpeg);
        assert_eq!(config.quality, 75.0);
        let config = Config::build(100.0, OutputFormat::Png, None, None, None).unwrap();
        assert_eq!(config.output_format, OutputFormat::Png);
        assert_eq!(config.quality, 100.0);
        let config = Config::build(0.0, OutputFormat::Oxipng, None, None, None).unwrap();
        assert_eq!(config.output_format, OutputFormat::Oxipng);
        assert_eq!(config.quality, 0.0);
        let config_result = Config::build(101.0, OutputFormat::MozJpeg, None, None, None);
        assert!(config_result.is_err());
        let config_result = Config::build(-1.0, OutputFormat::MozJpeg, None, None, None);
        assert!(config_result.is_err());
    }

    #[test]
    fn decode_unsupported() {
        let path = path::Path::new("tests/files/test.bmp");

        let file = fs::File::open(path).unwrap();

        let decoder = Decoder::new(path, file);
        let result = decoder.decode();
        assert!(result.is_err());
    }

    #[test]
    fn decode_grayscale() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x]&[^t].+0g\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_grayscale_alpha() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+4a\d\d\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_rgb() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^^tests/files/[^x]&[^t].+2c\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_rgb_transparent() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^^tests/files/[^x]&[t].+2c\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_rgba() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+6a\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_indexed() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x]&[^t].+3p\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_indexed_transparent() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x]&[t].+3p\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_corrupted() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/x.+\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let d = Decoder::new(path, file);

            let img = d.decode();
            assert!(img.is_err());
        })
    }

    #[test]
    fn decode_webp() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/.+.webp$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let d = Decoder::new(path, file);

            let img = d.decode().unwrap();
            println!("{:?}", img.size());
            println!("{:?}", img.data().len());

            assert_ne!(img.data().len(), 0);
            assert_ne!(img.size(), (0, 0));
        })
    }

    #[test]
    fn decode_avif() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/.+.avif$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let d = Decoder::new(path, file);

            let img = d.decode().unwrap();
            println!("{:?}", img.size());
            println!("{:?}", img.data().len());

            assert_ne!(img.data().len(), 0);
            assert_ne!(img.size(), (0, 0));
        })
    }

    #[test]
    fn encode_jpeg() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            let conf = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();

            assert!(result.is_ok());
            let result = result.unwrap();
            assert!(!result.is_empty());
        })
    }

    #[test]
    fn encode_png() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            let conf = Config::build(75.0, OutputFormat::Png, None, None, None).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();

            assert!(result.is_ok());
            let result = result.unwrap();
            assert!(!result.is_empty());
        })
    }

    #[test]
    fn encode_oxipng() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            let conf = Config::build(75.0, OutputFormat::Oxipng, None, None, None).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();

            assert!(result.is_ok());
            let result = result.unwrap();
            assert!(!result.is_empty());
        })
    }

    #[test]
    fn encode_webp() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            let conf = Config::build(75.0, OutputFormat::WebP, None, None, None).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();

            assert!(result.is_ok());
            let result = result.unwrap();
            assert!(!result.is_empty());
        })
    }

    #[test]
    fn encode_avif() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            println!("{path:?}");
            let file = fs::File::open(path).unwrap();
            let image = Decoder::new(path, file).decode().unwrap();

            let conf = Config::build(75.0, OutputFormat::Avif, None, None, None).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();

            assert!(result.is_ok());
            let result = result.unwrap();
            assert!(!result.is_empty());
        })
    }

    #[test]
    fn encode_quantized() {
        let path = path::PathBuf::from("tests/files/basi2c08.png");
        let file = fs::File::open(&path).unwrap();

        let image = Decoder::new(&path, file).decode().unwrap();

        let conf = Config::build(75.0, OutputFormat::Oxipng, None, None, None).unwrap();

        let encoder = Encoder::new(&conf, image);
        let result = encoder.encode_quantized(50, 1.0);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn encode_quantized_out_of_bounds() {
        let path = path::PathBuf::from("tests/files/basi2c08.png");
        let file = fs::File::open(&path).unwrap();

        let image = Decoder::new(&path, file).decode().unwrap();

        let conf = Config::build(75.0, OutputFormat::Oxipng, None, None, None).unwrap();

        let encoder = Encoder::new(&conf, image);
        let result = encoder.encode_quantized(120, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn resize_image() {
        let data = [255; 100 * 100 * 4];
        let image = ImageData::new(100, 100, &data);

        let conf = Config::build(75.0, OutputFormat::Oxipng, Some(50), Some(50), None).unwrap();

        let mut encoder = Encoder::new(&conf, image);

        let result = encoder.resize();

        assert!(result.is_ok());
        assert_eq!(encoder.image_data.size(), (50, 50));
        assert!(encoder.image_data.data().len() < 100 * 100 * 4);
    }
}

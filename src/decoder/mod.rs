use image::error::{DecodingError, ImageFormatHint, UnsupportedError, UnsupportedErrorKind};
use image::DynamicImage::{ImageLuma8, ImageLumaA8, ImageRgb8, ImageRgba8};
use image::{
    DynamicImage, GrayAlphaImage, GrayImage, ImageError, ImageResult, RgbImage, RgbaImage,
};
use std::io::Seek;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use image::io::Reader as ImageReader;

use crate::config::ImageFormat;

/// Decoder for reading and decoding images from various formats.
pub struct Decoder<R: BufRead + Seek> {
    r: ImageReader<R>,
    format: Option<ImageFormat>,
    #[cfg(feature = "transform")]
    fix_orientation: Option<u32>,
}

impl<R: BufRead + Seek> Decoder<R> {
    /// Creates a new [`Decoder`] with the specified input reader.
    ///
    /// # Parameters
    ///
    /// - `r`: The input reader implementing [`Read`] from which image data will be read.
    ///
    /// # Returns
    ///
    /// Returns a [`Decoder`] instance.
    #[inline]
    pub fn new(r: R) -> Self {
        Self {
            r: ImageReader::new(r),
            format: None,
            #[cfg(feature = "transform")]
            fix_orientation: None,
        }
    }

    /// Sets the image format for the decoder.
    ///
    /// # Parameters
    ///
    /// - `format`: The desired image format as an [`ImageFormat`] enum variant.
    ///
    /// # Returns
    ///
    /// Returns a modified [`Decoder`] with the specified image format.
    #[inline]
    pub fn with_format(mut self, format: ImageFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Sets the fixed orientation for image decoding.
    ///
    /// This method allows you to specify a fixed orientation for decoding images that may have
    /// incorrect orientation metadata. The specified orientation will be used during the decoding
    /// process to ensure the image is correctly oriented.
    ///
    /// # Parameters
    ///
    /// - `orientation`: An integer value representing the fixed orientation for decoding. It should
    ///   be a value between 1 and 8, inclusive. Use this to correct images with incorrect orientation
    ///   metadata.
    ///
    /// # Returns
    ///
    /// Returns a modified [`Decoder`] instance with the specified fixed orientation.
    ///
    /// # Example
    ///
    /// ```no run
    /// use your_crate::{Decoder, ImageFormat};
    ///
    /// let decoder = Decoder::new(/* ... */)
    ///     .with_fixed_orientation(6); // Set a fixed orientation for decoding
    ///
    /// let decoded_image = decoder.decode().unwrap();
    /// ```
    #[inline]
    #[cfg(feature = "transform")]
    pub fn with_fixed_orientation(mut self, orientation: u32) -> Self {
        self.fix_orientation = Some(orientation);
        self
    }

    /// Decodes the image using the specified format and input data.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing the decoded [`Image`] on success or a [`DecoderError`] on failure.
    pub fn decode(self) -> ImageResult<DynamicImage> {
        let image = match self.format {
            #[cfg(feature = "jxl")]
            Some(ImageFormat::JpegXl) => self.decode_jpegxl(),
            #[cfg(feature = "avif")]
            Some(ImageFormat::Avif) => self.decode_avif(),
            _ => self.r.with_guessed_format()?.decode(),
        }?;

        Ok(image)
    }

    #[cfg(feature = "jxl")]
    fn decode_jpegxl(self) -> ImageResult<DynamicImage> {
        use jxl_oxide::{JxlImage, PixelFormat};

        let image = JxlImage::from_reader(self.r.into_inner()).map_err(|e| {
            ImageError::Decoding(DecodingError::new(
                ImageFormatHint::Name("JpegXL".to_string()),
                e,
            ))
        })?;
        let render = image.render_frame(0).map_err(|e| {
            ImageError::Decoding(DecodingError::new(
                ImageFormatHint::Name("JpegXL".to_string()),
                e,
            ))
        })?;

        let format = image.pixel_format();

        let framebuffer = render.image();

        Ok(match format {
            PixelFormat::Gray => ImageLuma8(
                GrayImage::from_raw(
                    framebuffer.width() as u32,
                    framebuffer.height() as u32,
                    framebuffer
                        .buf()
                        .iter()
                        .map(|x| x * 255. + 0.5)
                        .map(|x| x as u8)
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            ),
            PixelFormat::Graya => ImageLumaA8(
                GrayAlphaImage::from_raw(
                    framebuffer.width() as u32,
                    framebuffer.height() as u32,
                    framebuffer
                        .buf()
                        .iter()
                        .map(|x| x * 255. + 0.5)
                        .map(|x| x as u8)
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            ),
            PixelFormat::Rgb => ImageRgb8(
                RgbImage::from_raw(
                    framebuffer.width() as u32,
                    framebuffer.height() as u32,
                    framebuffer
                        .buf()
                        .iter()
                        .map(|x| x * 255. + 0.5)
                        .map(|x| x as u8)
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            ),
            PixelFormat::Rgba => ImageRgba8(
                RgbaImage::from_raw(
                    framebuffer.width() as u32,
                    framebuffer.height() as u32,
                    framebuffer
                        .buf()
                        .iter()
                        .map(|x| x * 255. + 0.5)
                        .map(|x| x as u8)
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            ),
            _ => Err(ImageError::Unsupported(
                UnsupportedError::from_format_and_kind(
                    ImageFormatHint::Name("JpegXL".to_string()),
                    UnsupportedErrorKind::GenericFeature("CMYK Colorspace".to_string()),
                ),
            ))?,
        })
    }

    #[cfg(feature = "avif")]
    fn decode_avif(self) -> ImageResult<DynamicImage> {
        let mut r = self.r.into_inner();

        let mut buf: Vec<u8> = vec![];

        r.read_to_end(&mut buf)?;

        libavif_image::read(&buf).map_err(|e| {
            ImageError::Decoding(DecodingError::new(
                ImageFormatHint::Exact(image::ImageFormat::Avif),
                e,
            ))
        })
    }
}

impl Decoder<BufReader<File>> {
    /// Creates a new [`Decoder`] from a file specified by the given path.
    ///
    /// This method opens the file at the specified path, sets up a `Reader` for reading, and
    /// determines the image format based on the file extension.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the image file to be decoded.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing the initialized [`Decoder`] instance on success or a [`DecoderError`]
    /// on failure. The [`Decoder`] is ready to decode the image from the specified file.
    ///
    /// # Errors
    ///
    /// This method may return a [`DecoderError`] if there are issues with opening the file, determining
    /// the image format, or other I/O-related errors.
    #[inline]
    pub fn from_path(path: impl AsRef<Path>) -> ImageResult<Self> {
        Ok(Self {
            r: ImageReader::open(path.as_ref())?,
            format: Some(ImageFormat::from_path(path.as_ref()).map_err(|e| {
                ImageError::Unsupported(UnsupportedError::from(ImageFormatHint::Unknown))
            })?),
            #[cfg(feature = "transform")]
            fix_orientation: Self::get_orientation(path.as_ref()),
        })
    }

    #[cfg(feature = "transform")]
    #[allow(unused_variables)]
    fn get_orientation(path: &Path) -> Option<u32> {
        #[cfg(not(feature = "exif"))]
        return None;

        #[cfg(feature = "exif")]
        {
            let exif_reader = exif::Reader::new();
            let mut reader = BufReader::new(File::open(path).ok()?);

            let exif = exif_reader.read_from_container(&mut reader).ok()?;

            let orientation = exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY)?;

            orientation.value.get_uint(0)
        }
    }
}

#[cfg(test)]
mod tests;

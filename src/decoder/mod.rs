use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use rgb::{
    alt::{GRAY8, GRAYA8},
    AsPixels, FromSlice, RGB8, RGBA8,
};

use crate::{config::ImageFormat, error::DecoderError, Image};

/// Decoder for reading and decoding images from various formats.
pub struct Decoder<R: BufRead> {
    r: R,
    format: Option<ImageFormat>,
    #[cfg(feature = "transform")]
    fix_orientation: Option<u32>,
}

impl<R: BufRead + std::panic::UnwindSafe> Decoder<R> {
    /// Creates a new [`Decoder`] with the specified input reader.
    ///
    /// # Parameters
    ///
    /// - `r`: The input reader implementing [`BufRead`] from which image data will be read.
    ///
    /// # Returns
    ///
    /// Returns a [`Decoder`] instance.
    #[inline]
    pub fn new(r: R) -> Self {
        Self {
            r,
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
    pub fn decode(self) -> Result<Image, DecoderError> {
        #[cfg(feature = "transform")]
        let orientation = self.fix_orientation;

        #[allow(unused_mut)]
        let mut image = match self.format {
            #[cfg(feature = "avif")]
            Some(ImageFormat::Avif) => unsafe { self.decode_avif() },
            Some(ImageFormat::Jpeg) => self.decode_jpeg(),
            #[cfg(feature = "jxl")]
            Some(ImageFormat::JpegXl) => self.decode_jpegxl(),
            Some(ImageFormat::Png) => self.decode_png(),
            #[cfg(feature = "webp")]
            Some(ImageFormat::WebP) => self.decode_webp(),
            None => Err(DecoderError::Format(
                crate::error::ImageFormatError::Missing,
            )),
        }?;

        #[cfg(feature = "transform")]
        if let Some(orientation) = orientation {
            image.fix_orientation(orientation)
        }

        Ok(image)
    }

    #[cfg(feature = "avif")]
    unsafe fn decode_avif(mut self) -> Result<Image, DecoderError> {
        use libavif_sys::*;

        let image = avifImageCreateEmpty();
        let decoder = avifDecoderCreate();

        let mut buf = Vec::new();

        self.r.read_to_end(&mut buf)?;

        let decode_result = avifDecoderReadMemory(decoder, image, buf.as_ptr(), buf.len());
        avifDecoderDestroy(decoder);

        if decode_result == AVIF_RESULT_OK {
            let mut rgb = avifRGBImage::default();
            avifRGBImageSetDefaults(&mut rgb, image);

            rgb.depth = 8;

            avifRGBImageAllocatePixels(&mut rgb);
            avifImageYUVToRGB(image, &mut rgb);

            let pixels =
                std::slice::from_raw_parts(rgb.pixels, (rgb.rowBytes * rgb.height) as usize)
                    .as_rgba()
                    .to_owned();

            let result = Image::new(pixels, rgb.width as usize, rgb.height as usize);

            avifRGBImageFreePixels(&mut rgb);

            Ok(result)
        } else {
            Err(DecoderError::Avif(decode_result))
        }
    }

    fn decode_jpeg(self) -> Result<Image, DecoderError> {
        std::panic::catch_unwind(|| -> Result<Image, DecoderError> {
            let decoder =
                mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_reader(self.r)?;

            let mut image = decoder.rgba()?;

            let pixels = image.read_scanlines()?;

            Ok(Image::new(pixels, image.width(), image.height()))
        })
        .map_err(|_| DecoderError::MozJpeg)?
    }

    #[cfg(feature = "jxl")]
    fn decode_jpegxl(mut self) -> Result<Image, DecoderError> {
        let runner = jpegxl_rs::ThreadsRunner::default();

        let decoder = jpegxl_rs::decoder_builder()
            .parallel_runner(&runner)
            .pixel_format(jpegxl_rs::decode::PixelFormat {
                num_channels: 4,
                endianness: jpegxl_rs::Endianness::Native,
                align: 0,
            })
            .build()?;

        let mut buf = Vec::new();

        self.r.read_to_end(&mut buf)?;

        let (info, pixels) = decoder.decode_with::<u8>(&buf)?;

        Ok(Image::new(
            pixels.as_rgba().to_owned(),
            info.width as usize,
            info.height as usize,
        ))
    }

    fn decode_png(self) -> Result<Image, DecoderError> {
        let mut decoder = png::Decoder::new(self.r);

        decoder.set_transformations(png::Transformations::normalize_to_color8());

        let mut reader = decoder.read_info()?;

        let mut buf: Vec<u8> = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut buf)?;

        match info.color_type {
            png::ColorType::Grayscale => Self::expand_pixels(&mut buf, |gray: GRAY8| gray.into()),
            png::ColorType::GrayscaleAlpha => Self::expand_pixels(&mut buf, GRAYA8::into),
            png::ColorType::Rgb => Self::expand_pixels(&mut buf, RGB8::into),
            png::ColorType::Rgba => {}
            png::ColorType::Indexed => {
                unreachable!("Indexed color type is expected to be expanded")
            }
        };

        Ok(Image::new(
            buf.as_rgba().to_owned(),
            info.width as usize,
            info.height as usize,
        ))
    }

    #[cfg(feature = "webp")]
    fn decode_webp(mut self) -> Result<Image, DecoderError> {
        let mut buf = Vec::new();

        self.r.read_to_end(&mut buf)?;

        let decoder = webp::Decoder::new(&buf);

        let mut image = decoder.decode().ok_or(DecoderError::WebP)?;

        if !image.is_alpha() {
            Self::expand_pixels(&mut image, RGB8::into);
        }

        Ok(Image::new(
            image.as_rgba().to_vec(),
            image.width() as usize,
            image.height() as usize,
        ))
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
}

impl Decoder<BufReader<File>> {
    /// Creates a new [`Decoder`] from a file specified by the given path.
    ///
    /// This method opens the file at the specified path, sets up a `BufReader` for reading, and
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
    pub fn from_path(path: &Path) -> Result<Self, DecoderError> {
        Ok(Self {
            r: BufReader::new(File::open(path)?),
            format: Some(ImageFormat::from_path(path)?),
            #[cfg(feature = "transform")]
            fix_orientation: Self::get_orientation(path),
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

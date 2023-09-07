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
        Self { r, format: None }
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

    /// Decodes the image using the specified format and input data.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing the decoded [`Image`] on success or a [`DecoderError`] on failure.
    pub fn decode(self) -> Result<Image, DecoderError> {
        match self.format {
            Some(ImageFormat::Avif) => unsafe { self.decode_avif() },
            Some(ImageFormat::Jpeg) => self.decode_jpeg(),
            Some(ImageFormat::Png) => self.decode_png(),
            Some(ImageFormat::WebP) => self.decode_webp(),
            None => Err(DecoderError::Format(
                crate::error::ImageFormatError::Missing,
            )),
        }
    }

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
        })
    }
}

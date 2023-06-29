use std::{ffi::CString, fs, io::Read, panic, path};

use log::info;
use rgb::{
    alt::{GRAY8, GRAYA8},
    AsPixels, ComponentBytes, FromSlice, RGB8, RGBA8,
};

use crate::{error::DecodingError, ImageData};

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
            None => return Err(DecodingError::Format("No extension".to_string())),
        };

        match extension.to_ascii_lowercase().to_str() {
            Some("jpg") | Some("jpeg") => self.decode_jpeg(),
            Some("png") => self.decode_png(),
            Some("webp") => self.decode_webp(),
            Some("avif") => self.decode_avif(),
            Some(ext) => Err(DecodingError::Format(ext.to_string())),
            None => Err(DecodingError::Parsing(
                "Failed to get extension".to_string(),
            )),
        }
    }

    // mut for not unix case
    #[allow(unused_mut)]
    fn decode_jpeg(mut self) -> Result<ImageData, DecodingError> {
        info!("Processing jpeg decoding");
        panic::catch_unwind(move || -> Result<ImageData, DecodingError> {
            // #[cfg(unix)]
            let d = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_file(self.file)?;
            // #[cfg(not(unix))]
            // let buf = {
            //     let metadata = self.file.metadata()?;
            //     let mut buf = Vec::with_capacity(metadata.len() as usize);
            //     self.file.read_to_end(&mut buf)?;
            //     buf
            // };
            // #[cfg(not(unix))]
            // let d = mozjpeg::Decompress::new_mem(&buf)?;

            let mut image = d.rgba()?;

            let data: Vec<RGBA8> = image
                .read_scanlines()
                .ok_or(DecodingError::Jpeg("Failed to read scanlines".to_string()))?;

            info!("JPEG Color space: {:?}", image.color_space());
            info!("Dimensions: {}x{}", image.width(), image.height());

            Ok(ImageData::new(
                image.width(),
                image.height(),
                data.as_bytes(),
            ))
        })
        .unwrap_or(Err(DecodingError::Jpeg(
            "Failed to decode jpeg".to_string(),
        )))
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
                return Err(DecodingError::Parsing(
                    "Indexed color type is not supported".to_string(),
                ))
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
                    .map_err(|e| DecodingError::Parsing(e.to_string()))?
                    .as_ptr(),
            )
        };
        unsafe { avifDecoderDestroy(decoder) };

        let mut result = Err(DecodingError::Avif("Failed to decode avif".to_string()));

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

#[cfg(test)]
mod tests;

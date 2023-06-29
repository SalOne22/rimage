use std::panic;

use log::info;
use rgb::{
    alt::{GRAY8, GRAYA8},
    ComponentBytes, RGB8, RGBA8,
};

use crate::{
    error::DecodingError,
    image::{ImageData, InputFormat},
};

use super::Decode;

pub struct MemoryDecoder<'a> {
    data: &'a [u8],
    format: InputFormat,
}

impl<'a> MemoryDecoder<'a> {
    #[inline]
    pub fn new(data: &'a [u8], format: InputFormat) -> Self {
        MemoryDecoder { data, format }
    }
}

impl<'a> Decode for MemoryDecoder<'a> {
    fn decode(self) -> Result<ImageData, DecodingError> {
        match self.format {
            InputFormat::Jpeg => self.decode_jpeg(),
            InputFormat::Png => self.decode_png(),
            InputFormat::WebP => self.decode_webp(),
            InputFormat::Avif => self.decode_avif(),
        }
    }

    fn decode_jpeg(self) -> Result<ImageData, DecodingError> {
        info!("Processing jpeg decoding");
        panic::catch_unwind(move || -> Result<ImageData, DecodingError> {
            let d = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_mem(self.data)?;

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

    fn decode_png(self) -> Result<ImageData, DecodingError> {
        info!("Processing png decoding");
        let mut d = png::Decoder::new(self.data);
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

    fn decode_webp(self) -> Result<ImageData, DecodingError> {
        let (width, height, buf) = libwebp::WebPDecodeRGBA(self.data)?;

        Ok(ImageData::new(width as usize, height as usize, &buf))
    }

    fn decode_avif(self) -> Result<ImageData, DecodingError> {
        use libavif_sys::*;

        let image = unsafe { avifImageCreateEmpty() };
        let decoder = unsafe { avifDecoderCreate() };
        let decode_result =
            unsafe { avifDecoderReadMemory(decoder, image, self.data.as_ptr(), self.data.len()) };
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

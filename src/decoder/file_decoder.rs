use std::{fs, io::Read, panic};

use jpegxl_rs::{
    decode::{Metadata, PixelFormat},
    decoder_builder, Endianness,
};
use log::info;
use rgb::{
    alt::{GRAY8, GRAYA8},
    ComponentBytes, RGB8, RGBA8,
};

use crate::{
    error::DecodingError,
    image::{ImageData, ImageFormat},
};

use super::Decode;

pub struct FileDecoder {
    format: ImageFormat,
    file: fs::File,
}

impl FileDecoder {
    #[inline]
    pub fn new(file: fs::File, format: ImageFormat) -> Self {
        FileDecoder { format, file }
    }
}

impl Decode for FileDecoder {
    fn decode(self) -> Result<ImageData, DecodingError> {
        match self.format {
            ImageFormat::Jpeg => self.decode_jpeg(),
            ImageFormat::Png => self.decode_png(),
            ImageFormat::WebP => self.decode_webp(),
            ImageFormat::Avif => self.decode_avif(),
            ImageFormat::Jxl => self.decode_jxl(),
        }
    }

    fn decode_jpeg(self) -> Result<ImageData, DecodingError> {
        info!("Processing jpeg decoding");
        panic::catch_unwind(move || -> Result<ImageData, DecodingError> {
            let d = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_file(self.file)?;

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

    fn decode_avif(mut self) -> Result<ImageData, DecodingError> {
        use libavif_sys::*;

        let image = unsafe { avifImageCreateEmpty() };
        let decoder = unsafe { avifDecoderCreate() };

        let metadata = self.file.metadata()?;
        let mut buf = Vec::with_capacity(metadata.len() as usize);
        self.file.read_to_end(&mut buf)?;

        let decode_result =
            unsafe { avifDecoderReadMemory(decoder, image, buf.as_ptr(), buf.len()) };
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

    fn decode_jxl(mut self) -> Result<ImageData, DecodingError> {
        let decoder = decoder_builder()
            .pixel_format(PixelFormat {
                num_channels: 4,
                endianness: Endianness::Big,
                align: 0,
            })
            .build()?;

        let metadata = self.file.metadata()?;
        let mut buf = Vec::with_capacity(metadata.len() as usize);
        self.file.read_to_end(&mut buf)?;

        let (Metadata { width, height, .. }, pixels) = decoder.decode_with::<u8>(&buf)?;

        Ok(ImageData::new(width as usize, height as usize, &pixels))
    }
}

#[cfg(test)]
mod tests;

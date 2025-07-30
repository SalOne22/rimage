use std::io::{Read, Seek};

use zune_core::colorspace::ColorSpace;
use zune_image::{errors::ImageErrors, image::Image, traits::DecoderTrait};

/// A Tiff decoder
pub struct TiffDecoder<R: Read + Seek> {
    inner: tiff::decoder::Decoder<R>,
    dimensions: Option<(usize, usize)>,
    colorspace: ColorSpace,
}

impl<R: Read + Seek> TiffDecoder<R> {
    /// Create a new tiff decoder that reads data from `source`
    pub fn try_new(source: R) -> Result<Self, ImageErrors> {
        let inner = tiff::decoder::Decoder::new(source).map_err(|e| {
            ImageErrors::ImageDecodeErrors(format!("Unable to create TIFF decoder: {e}"))
        })?;

        Ok(Self {
            inner,
            dimensions: None,
            colorspace: ColorSpace::Unknown,
        })
    }
}

impl<R> DecoderTrait for TiffDecoder<R>
where
    R: Read + Seek,
{
    fn decode(&mut self) -> Result<Image, ImageErrors> {
        let (width, height) = self.inner.dimensions().map_err(|e| {
            ImageErrors::ImageDecodeErrors(format!("Unable to read dimensions - {e}"))
        })?;

        let (width, height) = (width as usize, height as usize);
        self.dimensions = Some((width, height));

        let colorspace = self
            .inner
            .colortype()
            .map(|colortype| match colortype {
                tiff::ColorType::RGB(_) => ColorSpace::RGB,
                tiff::ColorType::RGBA(_) => ColorSpace::RGBA,
                tiff::ColorType::CMYK(_) => ColorSpace::CMYK,
                tiff::ColorType::Gray(_) => ColorSpace::Luma,
                tiff::ColorType::GrayA(_) => ColorSpace::LumaA,
                tiff::ColorType::YCbCr(_) => ColorSpace::YCbCr,
                _ => ColorSpace::Unknown,
            })
            .map_err(|e| {
                ImageErrors::ImageDecodeErrors(format!("Unable to read colorspace - {e}"))
            })?;

        self.colorspace = colorspace;

        let result = self.inner.read_image().map_err(|e| {
            ImageErrors::ImageDecodeErrors(format!("Unable to decode TIFF file - {e}"))
        })?;

        match result {
            tiff::decoder::DecodingResult::U8(data) => {
                Ok(Image::from_u8(&data, width, height, colorspace))
            }
            tiff::decoder::DecodingResult::U16(data) => {
                Ok(Image::from_u16(&data, width, height, colorspace))
            }
            tiff::decoder::DecodingResult::F32(data) => {
                Ok(Image::from_f32(&data, width, height, colorspace))
            }
            _ => Err(ImageErrors::ImageDecodeErrors(
                "Tiff Data format not supported".to_string(),
            )),
        }
    }

    fn dimensions(&self) -> Option<(usize, usize)> {
        self.dimensions
    }

    fn out_colorspace(&self) -> ColorSpace {
        self.colorspace
    }

    fn name(&self) -> &'static str {
        "tiff-decoder"
    }
}

#[cfg(test)]
mod tests;

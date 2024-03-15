use std::{io::Read, marker::PhantomData};

use zune_core::{bytestream::ZReaderTrait, colorspace::ColorSpace};
use zune_image::{errors::ImageErrors, image::Image, traits::DecoderTrait};

pub struct AvifDecoder<R: Read> {
    inner: Vec<u8>,
    dimensions: Option<(usize, usize)>,
    phantom: PhantomData<R>,
}

impl<R: Read> AvifDecoder<R> {
    pub fn try_new(mut source: R) -> Result<AvifDecoder<R>, ImageErrors> {
        let mut buf = Vec::new();
        source.read_to_end(&mut buf)?;

        Ok(AvifDecoder {
            inner: buf,
            dimensions: None,
            phantom: PhantomData,
        })
    }
}

impl<R, T> DecoderTrait<T> for AvifDecoder<R>
where
    R: Read,
    T: ZReaderTrait,
{
    fn decode(&mut self) -> Result<Image, ImageErrors> {
        let img = libavif::decode_rgb(&self.inner)
            .map_err(|e| ImageErrors::ImageDecodeErrors(e.to_string()))?;

        let (w, h) = (img.width() as usize, img.height() as usize);
        self.dimensions = Some((w, h));

        Ok(Image::from_u8(&img, w, h, ColorSpace::RGBA))
    }

    fn dimensions(&self) -> Option<(usize, usize)> {
        self.dimensions
    }

    fn out_colorspace(&self) -> ColorSpace {
        ColorSpace::RGBA
    }

    fn name(&self) -> &'static str {
        "avif-decoder (aom)"
    }
}

#[cfg(test)]
mod tests;

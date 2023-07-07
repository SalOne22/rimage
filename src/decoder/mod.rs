use std::{fs, path, str::FromStr};

use rgb::{AsPixels, FromSlice, RGBA8};

use crate::{
    error::DecodingError,
    image::{ImageData, InputFormat},
};

use self::{file_decoder::FileDecoder, memory_decoder::MemoryDecoder};

mod file_decoder;
mod memory_decoder;

pub trait Decode {
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

    fn decode(self) -> Result<ImageData, DecodingError>;

    fn decode_jpeg(self) -> Result<ImageData, DecodingError>;
    fn decode_png(self) -> Result<ImageData, DecodingError>;
    fn decode_webp(self) -> Result<ImageData, DecodingError>;
    fn decode_avif(self) -> Result<ImageData, DecodingError>;
}

/// Used to build Decoder
///
/// # Example
///
/// ```
/// # use rimage::Decoder;
/// let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
///
/// let decoder = Decoder::from_path(&path).unwrap();
/// ```
pub struct Decoder;

impl Decoder {
    /// Builds Decoder from file path
    ///
    /// # Example
    ///
    /// ```
    /// # use rimage::Decoder;
    /// let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    ///
    /// let decoder = Decoder::from_path(&path).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Can return error if file format is not supported or file failed to open
    pub fn from_path(path: &path::Path) -> Result<GenericDecoder<FileDecoder>, DecodingError> {
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let format = InputFormat::from_str(extension)
            .map_err(|_| DecodingError::Format(extension.to_string()))?;

        Ok(Self::from_file(fs::File::open(path)?, format))
    }

    /// Builds Decoder from opened file
    ///
    /// # Example
    ///
    /// ```
    /// # use rimage::{Decoder, image::InputFormat};
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// let file = std::fs::File::open(path).unwrap();
    ///
    /// let decoder = Decoder::from_file(file, InputFormat::Jpeg);
    /// ```
    #[inline]
    pub fn from_file(file: fs::File, format: InputFormat) -> GenericDecoder<FileDecoder> {
        GenericDecoder {
            inner: FileDecoder::new(file, format),
        }
    }
    /// Builds Decoder from bytes
    ///
    /// # Example
    ///
    /// ```
    /// # use std::io::Read;
    /// # use rimage::{Decoder, image::InputFormat};
    /// # let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let mut file = std::fs::File::open(path).unwrap();
    /// let metadata = file.metadata().unwrap();
    /// let mut buf = Vec::with_capacity(metadata.len() as usize);
    /// file.read_to_end(&mut buf).unwrap();
    ///
    /// let decoder = Decoder::from_mem(&buf, InputFormat::Jpeg);
    /// ```
    #[inline]
    pub fn from_mem(data: &[u8], format: InputFormat) -> GenericDecoder<MemoryDecoder> {
        GenericDecoder {
            inner: MemoryDecoder::new(data, format),
        }
    }
}

pub struct GenericDecoder<T: Decode> {
    inner: T,
}

impl<T: Decode> GenericDecoder<T> {
    pub fn decode(self) -> Result<ImageData, DecodingError> {
        self.inner.decode()
    }
}

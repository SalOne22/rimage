use std::{ffi::OsStr, path::Path};

use crate::error::ImageFormatError;

/// Enum representing supported image formats.
#[derive(Debug, PartialEq, Eq)]
pub enum ImageFormat {
    /// JPEG image format.
    Jpeg,
    /// PNG image format.
    Png,
    /// JPEG XL image format.
    #[cfg(feature = "jxl")]
    JpegXl,
    /// WebP image format.
    #[cfg(feature = "webp")]
    WebP,
    /// AVIF image format.
    #[cfg(feature = "avif")]
    Avif,
}

impl ImageFormat {
    /// Attempts to create an [`ImageFormat`] variant from a file extension.
    ///
    /// # Parameters
    ///
    /// - `ext`: The file extension as an [`OsStr`].
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] with the parsed [`ImageFormat`] on success or an [`ImageFormatError`] on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::{config::ImageFormat, error::ImageFormatError};
    ///
    /// let ext = "jpg";
    ///
    /// match ImageFormat::from_ext(ext) {
    ///     Ok(format) => println!("Image format: {:?}", format),
    ///     Err(err) => eprintln!("Error parsing image format: {:?}", err),
    /// }
    /// ```
    #[inline]
    pub fn from_ext(ext: impl AsRef<OsStr>) -> Result<Self, ImageFormatError> {
        Ok(
            match ext
                .as_ref()
                .to_str()
                .ok_or(ImageFormatError::Missing)?
                .to_lowercase()
                .as_str()
            {
                "jpg" | "jpeg" => Self::Jpeg,
                "png" => Self::Png,
                #[cfg(feature = "jxl")]
                "jxl" => Self::JpegXl,
                #[cfg(feature = "webp")]
                "webp" => Self::WebP,
                #[cfg(feature = "avif")]
                "avif" => Self::Avif,
                ext => return Err(ImageFormatError::Unknown(ext.to_string())),
            },
        )
    }

    /// Attempts to create an [`ImageFormat`] variant from a file path.
    ///
    /// # Parameters
    ///
    /// - `path`: The file path from which the extension is extracted.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] with the parsed [`ImageFormat`] on success or an [`ImageFormatError`] on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::{config::ImageFormat, error::ImageFormatError};
    /// use std::path::Path;
    ///
    /// let file_path = Path::new("image.jpg");
    /// match ImageFormat::from_path(&file_path) {
    ///     Ok(format) => println!("Image format: {:?}", format),
    ///     Err(err) => eprintln!("Error parsing image format: {:?}", err),
    /// }
    /// ```
    #[inline]
    pub fn from_path(path: &Path) -> Result<Self, ImageFormatError> {
        path.extension()
            .map(Self::from_ext)
            .ok_or(ImageFormatError::Missing)?
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn to_unknown() {
        let img_format = ImageFormat::from_ext("bmp");
        assert!(img_format.is_err());
        assert_eq!(
            img_format.unwrap_err(),
            ImageFormatError::Unknown("bmp".to_string())
        );
    }

    #[test]
    fn to_jpeg() {
        let img_format = ImageFormat::from_ext("jpg");
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::Jpeg);

        let img_format = ImageFormat::from_ext("jpeg");
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::Jpeg);

        let img_format = ImageFormat::from_path(&PathBuf::from("image.jpg"));
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::Jpeg);

        let img_format = ImageFormat::from_path(&PathBuf::from("image.jpeg"));
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::Jpeg);
    }

    #[test]
    fn to_png() {
        let img_format = ImageFormat::from_ext("png");
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::Png);

        let img_format = ImageFormat::from_path(&PathBuf::from("image.png"));
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::Png);
    }

    #[test]
    #[cfg(feature = "jxl")]
    fn to_jxl() {
        let img_format = ImageFormat::from_ext("jxl");
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::JpegXl);

        let img_format = ImageFormat::from_path(&PathBuf::from("image.jxl"));
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::JpegXl);
    }

    #[test]
    #[cfg(feature = "webp")]
    fn to_webp() {
        let img_format = ImageFormat::from_ext("webp");
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::WebP);

        let img_format = ImageFormat::from_path(&PathBuf::from("image.webp"));
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::WebP);
    }

    #[test]
    #[cfg(feature = "avif")]
    fn to_avif() {
        let img_format = ImageFormat::from_ext("avif");
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::Avif);

        let img_format = ImageFormat::from_path(&PathBuf::from("image.avif"));
        assert!(img_format.is_ok());
        assert_eq!(img_format.unwrap(), ImageFormat::Avif);
    }
}

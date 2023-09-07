use std::{ffi::OsStr, path::Path};

use crate::error::ImageFormatError;

/// Enum representing supported image formats.
#[derive(Debug)]
pub enum ImageFormat {
    /// PNG image format.
    Png,
    /// JPEG image format.
    Jpeg,
    /// WebP image format.
    WebP,
    /// AVIF image format.
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
            match ext.as_ref().to_str().ok_or(ImageFormatError::Missing)? {
                "png" => Self::Png,
                "jpg" | "jpeg" => Self::Jpeg,
                "webp" => Self::WebP,
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

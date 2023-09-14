use std::str::FromStr;

/// Enum representing various image codecs supported by rimage.
///
/// This enum defines the codecs that can be used for encoding and decoding images.
/// Each variant corresponds to a specific image codec, allowing you to specify the
/// desired codec when working with images.
///
/// # Examples
///
/// ```
/// use rimage::config::Codec;
///
/// let jpeg_codec = Codec::MozJpeg;
/// let png_codec = Codec::Png;
///
/// // Use the codec with image processing functions.
/// // ...
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Codec {
    /// Mozilla JPEG codec for image encoding and decoding.
    MozJpeg,

    /// PNG codec for image encoding and decoding.
    Png,

    /// JPEG XL codec for image encoding and decoding.
    #[cfg(feature = "jxl")]
    JpegXl,

    /// OxiPNG codec for better PNG image optimization.
    #[cfg(feature = "oxipng")]
    OxiPng,

    /// WebP codec for image encoding and decoding with WebP format.
    #[cfg(feature = "webp")]
    WebP,

    /// AVIF codec for image encoding and decoding.
    #[cfg(feature = "avif")]
    Avif,
}

impl Codec {
    /// Converts a codec enum variant into its corresponding file extension.
    ///
    /// # Returns
    ///
    /// Returns the file extension as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::Codec;
    ///
    /// let jpeg_extension = Codec::MozJpeg.to_extension();
    /// assert_eq!(jpeg_extension, "jpg");
    ///
    /// let png_extension = Codec::Png.to_extension();
    /// assert_eq!(png_extension, "png");
    /// ```
    pub fn to_extension(&self) -> &str {
        match self {
            Codec::MozJpeg => "jpg",
            Codec::Png => "png",
            #[cfg(feature = "jxl")]
            Codec::JpegXl => "jxl",
            #[cfg(feature = "oxipng")]
            Codec::OxiPng => "png",
            #[cfg(feature = "webp")]
            Codec::WebP => "webp",
            #[cfg(feature = "avif")]
            Codec::Avif => "avif",
        }
    }
}

impl FromStr for Codec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "mozjpeg" | "jpeg" | "jpg" => Self::MozJpeg,
            "png" => Self::Png,
            #[cfg(feature = "jxl")]
            "jpegxl" | "jxl" => Self::JpegXl,
            #[cfg(feature = "oxipng")]
            "oxipng" => Self::OxiPng,
            #[cfg(feature = "webp")]
            "webp" => Self::WebP,
            #[cfg(feature = "avif")]
            "avif" => Self::Avif,

            codec => return Err(format!("{codec} is not supported codec.")),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_unknown() {
        let codec = Codec::from_str("bmp");
        assert!(codec.is_err());
        assert_eq!(codec.unwrap_err(), "bmp is not supported codec.");
    }

    #[test]
    fn to_mozjpeg() {
        let codec = Codec::from_str("mozjpeg");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::MozJpeg);
        let codec = Codec::from_str("jpeg");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::MozJpeg);
        let codec = Codec::from_str("jpg");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::MozJpeg);
    }

    #[test]
    fn to_png() {
        let codec = Codec::from_str("png");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::Png);
    }

    #[test]
    #[cfg(feature = "jxl")]
    fn to_jpegxl() {
        let codec = Codec::from_str("jpegxl");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::JpegXl);
        let codec = Codec::from_str("jxl");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::JpegXl);
    }

    #[test]
    #[cfg(feature = "oxipng")]
    fn to_oxipng() {
        let codec = Codec::from_str("oxipng");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::OxiPng);
    }

    #[test]
    #[cfg(feature = "webp")]
    fn to_webp() {
        let codec = Codec::from_str("webp");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::WebP);
    }

    #[test]
    #[cfg(feature = "avif")]
    fn to_avif() {
        let codec = Codec::from_str("avif");
        assert!(codec.is_ok());
        assert_eq!(codec.unwrap(), Codec::Avif);
    }
}

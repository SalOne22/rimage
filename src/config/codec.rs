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

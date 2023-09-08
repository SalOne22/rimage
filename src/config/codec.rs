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
#[derive(Debug, PartialEq, Eq)]
pub enum Codec {
    /// Mozilla JPEG codec for image encoding and decoding.
    MozJpeg,

    /// JPEG XL codec for image encoding and decoding.
    JpegXl,

    /// PNG codec for image encoding and decoding.
    Png,

    /// OxiPNG codec for better PNG image optimization.
    OxiPng,

    /// WebP codec for image encoding and decoding with WebP format.
    WebP,

    /// AVIF codec for image encoding and decoding.
    Avif,
}

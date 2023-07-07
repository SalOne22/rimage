use std::{borrow::Cow, fmt, str::FromStr};

/// Image data
///
/// Used to store dimensions and data of an image
#[derive(Debug, Clone)]
pub struct ImageData {
    width: usize,
    height: usize,
    data: Box<[u8]>,
}

impl ImageData {
    /// Creates a new [`ImageData`]
    ///
    /// # Examples
    /// ```
    /// # use rimage::image::ImageData;
    /// let image = ImageData::new(100, 100, &[0; 100 * 100 * 4]); // 100x100 RGBA image
    /// ```
    pub fn new(width: usize, height: usize, data: &[u8]) -> Self {
        Self {
            width,
            height,
            data: data.into(),
        }
    }
    /// Get the width and height of the image
    #[inline]
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    /// Get image data
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    /// Get image data as mutable
    #[inline]
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

/// Image format for decoder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// Jpeg image
    Jpeg,
    /// Png image
    Png,
    /// WebP image
    WebP,
    /// AVIF image
    Avif,
}

impl std::str::FromStr for ImageFormat {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mozjpeg" | "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "png" => Ok(ImageFormat::Png),
            "webp" => Ok(ImageFormat::WebP),
            "avif" => Ok(ImageFormat::Avif),
            _ => Err(format!("{} is not a valid input format", s).into()),
        }
    }
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageFormat::Jpeg => write!(f, "jpg"),
            ImageFormat::Png => write!(f, "png"),
            ImageFormat::WebP => write!(f, "webp"),
            ImageFormat::Avif => write!(f, "avif"),
        }
    }
}

/// Codec for processing output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    /// MozJpeg codec, outputs .jpg
    MozJpeg,
    /// Browser Png codec, outputs .png
    Png,
    /// OxiPng codec, outputs .png
    Oxipng,
    /// WebP codec, outputs .webp
    WebP,
    /// AVIF codec, outputs .avif
    Avif,
}

impl std::str::FromStr for Codec {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mozjpeg" | "jpg" | "jpeg" => Ok(Codec::MozJpeg),
            "png" => Ok(Codec::Png),
            "oxipng" => Ok(Codec::Oxipng),
            "webp" => Ok(Codec::WebP),
            "avif" => Ok(Codec::Avif),
            _ => Err(format!("{} is not a valid output format", s).into()),
        }
    }
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Codec::MozJpeg => write!(f, "jpg"),
            Codec::Png => write!(f, "png"),
            Codec::Oxipng => write!(f, "png"),
            Codec::WebP => write!(f, "webp"),
            Codec::Avif => write!(f, "avif"),
        }
    }
}

/// Wrapper around [`resize::Type`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeType {
    /// [`resize::Type::Point`]
    Point,
    /// [`resize::Type::Triangle`]
    Triangle,
    /// [`resize::Type::Catrom`]
    CatmullRom,
    /// [`resize::Type::Mitchell`]
    Mitchell,
    /// [`resize::Type::Lanczos3`]
    Lanczos3,
}

// Implement to [`resize::Type`] for [`ResizeType`]
impl From<&ResizeType> for resize::Type {
    fn from(resize_type: &ResizeType) -> Self {
        match resize_type {
            ResizeType::Point => resize::Type::Point,
            ResizeType::Triangle => resize::Type::Triangle,
            ResizeType::CatmullRom => resize::Type::Catrom,
            ResizeType::Mitchell => resize::Type::Mitchell,
            ResizeType::Lanczos3 => resize::Type::Lanczos3,
        }
    }
}

impl FromStr for ResizeType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "point" => Ok(Self::Point),
            "triangle" => Ok(Self::Triangle),
            "catmull-rom" => Ok(Self::CatmullRom),
            "mitchell" => Ok(Self::Mitchell),
            "lanczos3" => Ok(Self::Lanczos3),
            _ => Err(format!("{} is not a valid resize type", s).into()),
        }
    }
}

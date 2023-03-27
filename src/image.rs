use std::{borrow::Cow, fmt};

/// Image data
#[derive(Debug, Clone)]
pub struct ImageData {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ImageData {
    /// Creates a new [`ImageData`]
    ///
    /// # Examples
    /// ```
    /// # use rimage::{ImageData, image};
    /// let image = ImageData::new(100, 100, vec![0; 100 * 100]);
    /// ```
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data,
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
}

/// Image format to encode
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// MozJpeg image
    MozJpeg,
    /// Browser Png image
    Png,
    /// OxiPng image
    Oxipng,
}

impl std::str::FromStr for OutputFormat {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mozjpeg" | "jpg" | "jpeg" => Ok(OutputFormat::MozJpeg),
            "png" => Ok(OutputFormat::Png),
            "oxipng" => Ok(OutputFormat::Oxipng),
            _ => Err(format!("{} is not a valid output format", s).into()),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::MozJpeg => write!(f, "jpeg"),
            OutputFormat::Png => write!(f, "png"),
            OutputFormat::Oxipng => write!(f, "oxipng"),
        }
    }
}

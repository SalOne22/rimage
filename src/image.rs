use std::{borrow::Cow, fmt};

/// Image data
///
/// Used to store dimensions and data of an image
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
    /// let image = ImageData::new(100, 100, vec![0; 100 * 100 * 4]); // 100x100 RGBA image
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
    /// Get image data as mutable
    #[inline]
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

/// Image format for output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            OutputFormat::MozJpeg => write!(f, "jpg"),
            OutputFormat::Png => write!(f, "png"),
            OutputFormat::Oxipng => write!(f, "png"),
        }
    }
}

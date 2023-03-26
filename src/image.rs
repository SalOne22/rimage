use std::{borrow::Cow, fmt};

/// Image data from decoder
///
/// # Examples
///
/// ```
/// # use rimage::{Decoder, error::DecodingError};
/// # use std::{path, fs};
/// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
/// let data = fs::read(&path)?;
/// let d = Decoder::new(&path, &data);
///
/// let image = d.decode()?;
///
/// // Get something from image data
/// println!("Color Space: {:?}", image.color_space());
/// println!("Size: {:?}", image.size());
/// println!("Data length: {:?}", image.data().len());
/// # Ok::<(), DecodingError>(())
/// ```
#[derive(Debug)]
pub struct ImageData {
    color_space: ColorSpace,
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
    /// let image = ImageData::new(image::ColorSpace::Gray, 100, 100, vec![0; 100 * 100]);
    /// ```
    pub fn new(color_space: ColorSpace, width: usize, height: usize, data: Vec<u8>) -> Self {
        Self {
            color_space,
            width,
            height,
            data,
        }
    }
    /// Returns size of image (Width, Height)
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData};
    /// # use std::path;
    /// # let path = path::Path::new("tests/files/basi0g01.jpg");
    /// # let data = std::fs::read(&path).unwrap();
    /// # let d = Decoder::new(&path, &data);
    /// # let image = d.decode().unwrap();
    /// let (width, height) = image.size();
    /// ```
    #[inline]
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    /// Returns a ref to color space of image [`ColorSpace`]
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData, image::ColorSpace};
    /// # use std::path;
    /// # let path = path::Path::new("tests/files/basi0g01.jpg");
    /// # let data = std::fs::read(&path).unwrap();
    /// # let d = Decoder::new(&path, &data);
    /// # let image = d.decode().unwrap();
    /// let color_space = image.color_space();
    /// match color_space {
    ///     ColorSpace::Gray => println!("Grayscale"),
    ///     ColorSpace::Rgb => println!("RGB"),
    ///     ColorSpace::Cmyk => println!("CMYK"),
    ///     ColorSpace::Rgba => println!("RGBA"),
    ///     ColorSpace::Indexed => println!("Indexed"),
    ///     ColorSpace::GrayAlpha => println!("Grayscale Alpha"),
    /// }
    /// ```
    /// [`ColorSpace`]: enum.ColorSpace.html
    #[inline]
    pub fn color_space(&self) -> &ColorSpace {
        &self.color_space
    }
    /// Returns a ref to array of bytes in image
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData};
    /// # use std::path;
    /// # let path = path::Path::new("tests/files/basi0g01.jpg");
    /// # let data = std::fs::read(&path).unwrap();
    /// # let d = Decoder::new(&path, &data);
    /// # let image = d.decode().unwrap();
    /// let data = image.data();
    /// ```
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    #[inline]
    pub(crate) fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}

/// Supported output format
///
/// # Examples
/// ```
/// # use rimage::OutputFormat;
/// # use std::str::FromStr;
/// let format = OutputFormat::from_str("mozjpeg").unwrap();
/// println!("Format: {}", format);
/// ```
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

/// Color space of image
///
/// # Examples
/// ```
/// # use rimage::image::ColorSpace;
/// # use std::str::FromStr;
/// let color_space = ColorSpace::from_str("rgb").unwrap();
/// println!("Color Space: {}", color_space);
/// ```
///
/// # Errors
///
/// - [`ColorSpace::from_str`] if color space is not supported
///
/// [`ColorSpace::from_str`]: enum.ColorSpace.html#method.from_str
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    /// **R**ed/**G**reen/**B**lue
    Rgb,
    /// **R**ed/**G**reen/**B**lue/**A**lpha
    Rgba,
    /// **C**yan/**M**agenta/**Y**ellow/Blac**K**
    Cmyk,
    /// Indexed color palette
    Indexed,
    /// Grayscale
    Gray,
    /// Grayscale/Alpha
    GrayAlpha,
}

impl From<png::ColorType> for ColorSpace {
    fn from(color_type: png::ColorType) -> Self {
        match color_type {
            png::ColorType::Grayscale => ColorSpace::Gray,
            png::ColorType::Rgb => ColorSpace::Rgb,
            png::ColorType::Indexed => ColorSpace::Indexed,
            png::ColorType::GrayscaleAlpha => ColorSpace::GrayAlpha,
            png::ColorType::Rgba => ColorSpace::Rgba,
        }
    }
}

impl From<mozjpeg::ColorSpace> for ColorSpace {
    fn from(color_space: mozjpeg::ColorSpace) -> Self {
        match color_space {
            mozjpeg::ColorSpace::JCS_GRAYSCALE => ColorSpace::Gray,
            mozjpeg::ColorSpace::JCS_CMYK => ColorSpace::Cmyk,
            mozjpeg::ColorSpace::JCS_RGB => ColorSpace::Rgb,
            _ => ColorSpace::Rgb,
        }
    }
}

impl Into<mozjpeg::ColorSpace> for ColorSpace {
    fn into(self) -> mozjpeg::ColorSpace {
        match self {
            ColorSpace::Rgb => mozjpeg::ColorSpace::JCS_RGB,
            ColorSpace::Rgba => mozjpeg::ColorSpace::JCS_EXT_RGBA,
            ColorSpace::Cmyk => mozjpeg::ColorSpace::JCS_CMYK,
            ColorSpace::Indexed => mozjpeg::ColorSpace::JCS_RGB,
            ColorSpace::Gray => mozjpeg::ColorSpace::JCS_GRAYSCALE,
            ColorSpace::GrayAlpha => mozjpeg::ColorSpace::JCS_GRAYSCALE,
        }
    }
}

impl Into<png::ColorType> for ColorSpace {
    fn into(self) -> png::ColorType {
        match self {
            ColorSpace::Rgb => png::ColorType::Rgb,
            ColorSpace::Rgba => png::ColorType::Rgba,
            ColorSpace::Cmyk => png::ColorType::Rgb,
            ColorSpace::Indexed => png::ColorType::Indexed,
            ColorSpace::Gray => png::ColorType::Grayscale,
            ColorSpace::GrayAlpha => png::ColorType::GrayscaleAlpha,
        }
    }
}

impl std::str::FromStr for ColorSpace {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rgb" => Ok(ColorSpace::Rgb),
            "rgba" => Ok(ColorSpace::Rgba),
            "cmyk" => Ok(ColorSpace::Cmyk),
            "indexed" => Ok(ColorSpace::Indexed),
            "grayscale" => Ok(ColorSpace::Gray),
            "grayscale_alpha" => Ok(ColorSpace::GrayAlpha),
            _ => Err(format!("{} is not a valid color space", s).into()),
        }
    }
}

impl fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorSpace::Rgb => write!(f, "rgb"),
            ColorSpace::Rgba => write!(f, "rgba"),
            ColorSpace::Cmyk => write!(f, "cmyk"),
            ColorSpace::Indexed => write!(f, "indexed"),
            ColorSpace::Gray => write!(f, "grayscale"),
            ColorSpace::GrayAlpha => write!(f, "grayscale_alpha"),
        }
    }
}

use std::{borrow::Cow, fmt};

/// Image data from decoder
///
/// # Examples
///
/// ```
/// # use rimage::{Decoder, DecodingError};
/// # use std::path;
/// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
/// let d = Decoder::build(&path)?;
///
/// let image = d.decode()?;
///
/// // Get something from image data
/// println!("Color Space: {:?}", image.color_space());
/// println!("Bit Depth: {:?}", image.bit_depth());
/// println!("Size: {:?}", image.size());
/// println!("Data length: {:?}", image.data().len());
/// # Ok::<(), DecodingError>(())
/// ```
pub struct ImageData {
    color_space: ColorSpace,
    bit_depth: BitDepth,
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ImageData {
    /// Creates a new [`ImageData`]
    ///
    /// # Examples
    /// ```
    /// # use rimage::{ImageData, ColorSpace, BitDepth};
    /// let image = ImageData::new(ColorSpace::Gray, BitDepth::Eight, 100, 100, vec![0; 100 * 100]);
    /// ```
    pub fn new(
        color_space: ColorSpace,
        bit_depth: BitDepth,
        width: usize,
        height: usize,
        data: Vec<u8>,
    ) -> Self {
        Self {
            color_space,
            bit_depth,
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
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
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
    /// # use rimage::{Decoder, ImageData, ColorSpace};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
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
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
    /// # let image = d.decode().unwrap();
    /// let data = image.data();
    /// ```
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    /// Returns a ref to bit depth of image [`BitDepth`]
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData, BitDepth};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
    /// # let image = d.decode().unwrap();
    /// let bit_depth = image.bit_depth();
    /// match bit_depth {
    ///     BitDepth::One => println!("1 bit"),
    ///     BitDepth::Two => println!("2 bits"),
    ///     BitDepth::Four => println!("4 bits"),
    ///     BitDepth::Eight => println!("8 bits"),
    ///     BitDepth::Sixteen => println!("16 bits"),
    /// }
    /// ```
    ///
    /// [`BitDepth`]: enum.BitDepth.html
    #[inline]
    pub fn bit_depth(&self) -> &BitDepth {
        &self.bit_depth
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
/// # use rimage::ColorSpace;
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
#[derive(Debug, PartialEq, Eq)]
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

/// Bit depth of image per pixel
///
/// # Examples
/// ```
/// # use rimage::BitDepth;
/// # use std::str::FromStr;
/// let bit_depth = BitDepth::from_str("8").unwrap();
/// println!("Bit Depth: {}", bit_depth);
/// ```
///
/// # Errors
///
/// - [`BitDepth::from_str`] if bit depth is not supported
///
/// [`BitDepth::from_str`]: enum.BitDepth.html#method.from_str
#[derive(Debug)]
#[repr(u8)]
pub enum BitDepth {
    /// One bit per pixel
    One = 1,
    /// Two bits per pixel
    Two = 2,
    /// Four bits per pixel
    Four = 4,
    /// Eight bits per pixel
    Eight = 8,
    /// Sixteen bits per pixel
    Sixteen = 16,
}

impl From<png::BitDepth> for BitDepth {
    fn from(bit_depth: png::BitDepth) -> Self {
        match bit_depth {
            png::BitDepth::One => BitDepth::One,
            png::BitDepth::Two => BitDepth::Two,
            png::BitDepth::Four => BitDepth::Four,
            png::BitDepth::Eight => BitDepth::Eight,
            png::BitDepth::Sixteen => BitDepth::Sixteen,
        }
    }
}

impl std::str::FromStr for BitDepth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(BitDepth::One),
            "2" => Ok(BitDepth::Two),
            "4" => Ok(BitDepth::Four),
            "8" => Ok(BitDepth::Eight),
            "16" => Ok(BitDepth::Sixteen),
            _ => Err(format!("{} is not a valid bit depth", s)),
        }
    }
}

impl fmt::Display for BitDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitDepth::One => write!(f, "1"),
            BitDepth::Two => write!(f, "2"),
            BitDepth::Four => write!(f, "4"),
            BitDepth::Eight => write!(f, "8"),
            BitDepth::Sixteen => write!(f, "16"),
        }
    }
}

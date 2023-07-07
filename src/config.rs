use crate::{
    error::ConfigError,
    image::{Codec, ResizeType},
};

/// Config for image encoding
///
/// # Example
/// ```
/// use rimage::{Config, image::{Codec, ResizeType}};
///
/// // Without resize
/// let config = Config::builder(Codec::MozJpeg).build().unwrap();
///
/// // With resize
/// let mut config_resize = Config::builder(Codec::MozJpeg)
///     .target_width(200)
///     .target_height(200)
///     .resize_type(ResizeType::Lanczos3)
///     .build()
///     .unwrap();
/// ```
///
/// # Default
/// ```
/// use rimage::{Config, image::Codec};
///
/// let config = Config::default();
/// assert_eq!(config.quality(), 75.0);
/// assert_eq!(config.output_format(), Codec::MozJpeg);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Quality of output image
    quality: f32,
    /// Output format of image
    output_format: Codec,
    /// Target width for output image
    target_width: Option<usize>,
    /// Target height for output image
    target_height: Option<usize>,
    /// Resize type for output image
    resize_type: Option<ResizeType>,
    /// Quantization quality of output image
    quantization_quality: Option<u8>,
    /// Dithering level for output image
    dithering_level: Option<f32>,
}

pub struct ConfigBuilder {
    /// Quality of output image
    quality: f32,
    /// Output format of image
    output_format: Codec,
    /// Target width for output image
    target_width: Option<usize>,
    /// Target height for output image
    target_height: Option<usize>,
    /// Resize type for output image
    resize_type: Option<ResizeType>,
    /// Quantization quality of output image
    quantization_quality: Option<u8>,
    /// Dithering level for output image
    dithering_level: Option<f32>,
}

impl Config {
    /// Create new config
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::{Codec, ResizeType}};
    ///
    /// // Without resize
    /// let config = Config::builder(Codec::MozJpeg).build().unwrap();
    ///
    /// // With resize
    /// let mut config_resize = Config::builder(Codec::MozJpeg)
    ///     .target_width(200)
    ///     .target_height(200)
    ///     .resize_type(ResizeType::Lanczos3)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder(output_format: Codec) -> ConfigBuilder {
        ConfigBuilder {
            quality: 75.0,
            output_format,
            target_width: None,
            target_height: None,
            resize_type: None,
            quantization_quality: None,
            dithering_level: None,
        }
    }

    /// Get quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::Codec};
    ///
    /// let config = Config::builder(Codec::MozJpeg).build().unwrap();
    /// assert_eq!(config.quality(), 75.0);
    /// ```    
    #[inline]
    pub fn quality(&self) -> f32 {
        self.quality
    }

    /// Get output format
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::Codec};
    ///
    /// let config = Config::builder(Codec::MozJpeg).build().unwrap();
    /// assert_eq!(config.output_format(), Codec::MozJpeg);
    /// ```
    #[inline]
    pub fn output_format(&self) -> Codec {
        self.output_format
    }

    /// Get target width
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::Codec};
    ///
    /// let mut config = Config::builder(Codec::MozJpeg)
    ///     .target_width(175)
    ///     .target_height(175)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(config.target_width(), Some(175));
    /// ```
    #[inline]
    pub fn target_width(&self) -> Option<usize> {
        self.target_width
    }

    /// Get target height
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::Codec};
    ///
    /// let mut config = Config::builder(Codec::MozJpeg)
    ///     .target_width(175)
    ///     .target_height(175)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(config.target_height(), Some(175));
    /// ```
    #[inline]
    pub fn target_height(&self) -> Option<usize> {
        self.target_height
    }

    /// Get target height
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::Codec, image::ResizeType};
    ///
    /// let mut config = Config::builder(Codec::MozJpeg)
    ///     .target_width(175)
    ///     .target_height(175)
    ///     .resize_type(ResizeType::Triangle)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(config.resize_type(), Some(ResizeType::Triangle));
    /// ```
    #[inline]
    pub fn resize_type(&self) -> Option<ResizeType> {
        self.resize_type
    }

    /// Get target quantization quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::Codec};
    ///
    /// let config = Config::builder(Codec::MozJpeg).build().unwrap();
    /// assert_eq!(config.quantization_quality(), None);
    /// ```
    #[inline]
    pub fn quantization_quality(&self) -> Option<u8> {
        self.quantization_quality
    }

    /// Get target dithering level
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::Codec};
    ///
    /// let config = Config::builder(Codec::MozJpeg).build().unwrap();
    /// assert_eq!(config.dithering_level(), None);
    /// ```
    #[inline]
    pub fn dithering_level(&self) -> Option<f32> {
        self.dithering_level
    }
}

impl ConfigBuilder {
    pub fn build(&mut self) -> Result<Config, ConfigError> {
        if !(0.0..=100.0).contains(&self.quality) {
            return Err(ConfigError::QualityOutOfBounds(self.quality));
        }

        if let Some(width) = self.target_width {
            if width == 0 {
                return Err(ConfigError::WidthIsZero);
            }
        }

        if let Some(height) = self.target_height {
            if height == 0 {
                return Err(ConfigError::HeightIsZero);
            }
        }

        if let Some(quality) = self.quantization_quality {
            if !(0..=100).contains(&quality) {
                return Err(ConfigError::QuantizationQualityOutOfBounds(quality));
            }
        }

        if let Some(dithering_level) = self.dithering_level {
            if !(0.0..=1.0).contains(&dithering_level) {
                return Err(ConfigError::DitheringLevelOutOfBounds(dithering_level));
            }
        }

        Ok(Config {
            quality: self.quality,
            output_format: self.output_format,
            target_width: self.target_width,
            target_height: self.target_height,
            resize_type: self.resize_type,
            quantization_quality: self.quantization_quality,
            dithering_level: self.dithering_level,
        })
    }

    #[inline]
    pub fn quality(&mut self, quality: f32) -> &mut Self {
        self.quality = quality;
        self
    }

    #[inline]
    pub fn target_width(&mut self, width: usize) -> &mut Self {
        self.target_width = Some(width);
        self
    }

    #[inline]
    pub fn target_height(&mut self, height: usize) -> &mut Self {
        self.target_height = Some(height);
        self
    }

    #[inline]
    pub fn resize_type(&mut self, resize_type: ResizeType) -> &mut Self {
        self.resize_type = Some(resize_type);
        self
    }

    #[inline]
    pub fn quantization_quality(&mut self, quality: u8) -> &mut Self {
        self.quantization_quality = Some(quality);
        self
    }

    #[inline]
    pub fn dithering_level(&mut self, dithering_level: f32) -> &mut Self {
        self.dithering_level = Some(dithering_level);
        self
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            quality: 75.0,
            output_format: Codec::MozJpeg,
            target_width: None,
            target_height: None,
            resize_type: Some(ResizeType::Lanczos3),
            quantization_quality: None,
            dithering_level: None,
        }
    }
}

#[cfg(test)]
mod tests;

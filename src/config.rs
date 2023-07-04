use crate::{
    error::ConfigError,
    image::{OutputFormat, ResizeType},
};

/// Config for image encoding
///
/// # Example
/// ```
/// use rimage::{Config, image::{OutputFormat, ResizeType}};
///
/// // Without resize
/// let config = Config::new(OutputFormat::MozJpeg).build();
///
/// // With resize
/// let mut config_resize = Config::new(OutputFormat::MozJpeg)
///     .target_width(Some(200))
///     .unwrap()
///     .target_height(Some(200))
///     .unwrap()
///     .resize_type(Some(ResizeType::Lanczos3))
///     .build();
/// ```
///
/// # Default
/// ```
/// use rimage::{Config, image::OutputFormat};
///
/// let config = Config::default();
/// assert_eq!(config.quality(), 75.0);
/// assert_eq!(config.output_format, OutputFormat::MozJpeg);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Quality of output image
    quality: f32,
    /// Output format of image
    pub output_format: OutputFormat,
    /// Target width for output image
    target_width: Option<usize>,
    /// Target height for output image
    target_height: Option<usize>,
    /// Resize type for output image
    pub resize_type: Option<ResizeType>,
    /// Quantization quality of output image
    quantization_quality: Option<u8>,
    /// Dithering level for output image
    dithering_level: Option<f32>,
}

pub struct ConfigBuilder {
    /// Quality of output image
    quality: f32,
    /// Output format of image
    output_format: OutputFormat,
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
    /// use rimage::{Config, image::{OutputFormat, ResizeType}};
    ///
    /// // Without resize
    /// let config = Config::new(OutputFormat::MozJpeg).build();
    ///
    /// // With resize
    /// let mut config_resize = Config::new(OutputFormat::MozJpeg)
    ///     .target_width(Some(200))
    ///     .unwrap()
    ///     .target_height(Some(200))
    ///     .unwrap()
    ///     .resize_type(Some(ResizeType::Lanczos3))
    ///     .build();
    /// ```
    ///
    /// # Errors
    ///
    /// If quality is not in range 0.0..=100.0
    ///
    /// ```
    /// use rimage::{Config, image::OutputFormat};
    ///
    /// let mut config = Config::new(OutputFormat::MozJpeg);
    /// assert!(config.quality(200.0).is_err());
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new(output_format: OutputFormat) -> ConfigBuilder {
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
    /// use rimage::{Config, image::OutputFormat};
    ///
    /// let config = Config::new(OutputFormat::MozJpeg).build();
    /// assert_eq!(config.quality(), 75.0);
    /// ```    
    #[inline]
    pub fn quality(&self) -> f32 {
        self.quality
    }

    /// Get target width
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::OutputFormat};
    ///
    /// let mut config = Config::new(OutputFormat::MozJpeg)
    ///     .target_width(Some(175))
    ///     .unwrap()
    ///     .target_height(Some(175))
    ///     .unwrap()
    ///     .build();
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
    /// use rimage::{Config, image::OutputFormat};
    ///
    /// let mut config = Config::new(OutputFormat::MozJpeg)
    ///     .target_width(Some(175))
    ///     .unwrap()
    ///     .target_height(Some(175))
    ///     .unwrap()
    ///     .build();
    ///
    /// assert_eq!(config.target_height(), Some(175));
    /// ```
    #[inline]
    pub fn target_height(&self) -> Option<usize> {
        self.target_height
    }

    /// Get target quantization quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, image::OutputFormat};
    ///
    /// let config = Config::new(OutputFormat::MozJpeg).build();
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
    /// use rimage::{Config, image::OutputFormat};
    ///
    /// let config = Config::new(OutputFormat::MozJpeg).build();
    /// assert_eq!(config.dithering_level(), None);
    /// ```
    #[inline]
    pub fn dithering_level(&self) -> Option<f32> {
        self.dithering_level
    }
}

impl ConfigBuilder {
    pub fn build(&mut self) -> Config {
        Config {
            quality: self.quality,
            output_format: self.output_format,
            target_width: self.target_width,
            target_height: self.target_height,
            resize_type: self.resize_type,
            quantization_quality: self.quantization_quality,
            dithering_level: self.dithering_level,
        }
    }

    #[inline]
    pub fn quality(&mut self, quality: f32) -> Result<&mut Self, ConfigError> {
        if !(0.0..=100.0).contains(&quality) {
            return Err(ConfigError::QualityOutOfBounds(quality));
        }

        self.quality = quality;
        Ok(self)
    }

    #[inline]
    pub fn target_width(&mut self, width: Option<usize>) -> Result<&mut Self, ConfigError> {
        if let Some(width) = width {
            if width == 0 {
                return Err(ConfigError::WidthIsZero);
            }
        }

        self.target_width = width;
        Ok(self)
    }

    #[inline]
    pub fn target_height(&mut self, height: Option<usize>) -> Result<&mut Self, ConfigError> {
        if let Some(height) = height {
            if height == 0 {
                return Err(ConfigError::HeightIsZero);
            }
        }

        self.target_height = height;
        Ok(self)
    }

    #[inline]
    pub fn resize_type(&mut self, resize_type: Option<ResizeType>) -> &mut Self {
        self.resize_type = resize_type;
        self
    }

    #[inline]
    pub fn quantization_quality(&mut self, quality: Option<u8>) -> Result<&mut Self, ConfigError> {
        if let Some(quality) = quality {
            if !(0..=100).contains(&quality) {
                return Err(ConfigError::QuantizationQualityOutOfBounds(quality));
            }
        }

        self.quantization_quality = quality;
        Ok(self)
    }

    #[inline]
    pub fn dithering_level(
        &mut self,
        dithering_level: Option<f32>,
    ) -> Result<&mut Self, ConfigError> {
        if let Some(dithering_level) = dithering_level {
            if !(0.0..=1.0).contains(&dithering_level) {
                return Err(ConfigError::DitheringLevelOutOfBounds(dithering_level));
            }
        }

        self.dithering_level = dithering_level;
        Ok(self)
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            quality: 75.0,
            output_format: OutputFormat::MozJpeg,
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

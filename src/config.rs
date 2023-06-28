use crate::{error::ConfigError, OutputFormat, ResizeType};

/// Config for image encoding
///
/// # Example
/// ```
/// use rimage::{Config, OutputFormat, ResizeType};
///
/// // Without resize
/// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
///
/// // With resize
/// let mut config_resize = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
/// config_resize.set_target_width(Some(200)).unwrap();
/// config_resize.set_target_height(Some(200)).unwrap();
/// config_resize.resize_type = Some(ResizeType::Lanczos3);
/// ```
///
/// # Default
/// ```
/// use rimage::{Config, OutputFormat};
///
/// let config = Config::default();
/// assert_eq!(config.quality(), 75.0);
/// assert_eq!(config.output_format(), &OutputFormat::MozJpeg);
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

impl Config {
    /// Create new config
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat, ResizeType};
    ///
    /// // Without resize
    /// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    ///
    /// // With resize
    /// let mut config_resize = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// config_resize.set_target_width(Some(200)).unwrap();
    /// config_resize.set_target_height(Some(200)).unwrap();
    /// config_resize.resize_type = Some(ResizeType::Lanczos3);
    /// ```
    ///
    /// # Errors
    ///
    /// If quality is not in range 0.0..=100.0
    ///
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(200.0, OutputFormat::MozJpeg);
    /// assert!(config.is_err());
    /// ```
    pub fn build(quality: f32, output_format: OutputFormat) -> Result<Self, ConfigError> {
        if !(0.0..=100.0).contains(&quality) {
            return Err(ConfigError::QualityOutOfBounds);
        }

        Ok(Config {
            quality,
            output_format,
            target_width: None,
            target_height: None,
            resize_type: None,
            quantization_quality: None,
            dithering_level: None,
        })
    }
    /// Get quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert_eq!(config.quality(), 75.0);
    /// ```    
    #[inline]
    pub fn quality(&self) -> f32 {
        self.quality
    }
    /// Set quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    ///
    /// config.set_quality(80.0);
    ///
    /// assert_eq!(config.quality(), 80.0);
    /// ```
    ///
    /// # Errors
    ///
    /// If quality is not in range 0.0..=100.0
    ///
    /// ```
    /// # use rimage::{Config, OutputFormat};
    /// # let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert!(config.set_quality(200.0).is_err());
    /// ```
    #[inline]
    pub fn set_quality(&mut self, quality: f32) -> Result<(), ConfigError> {
        if !(0.0..=100.0).contains(&quality) {
            return Err(ConfigError::QualityOutOfBounds);
        }

        self.quality = quality;
        Ok(())
    }
    /// Get output format
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert_eq!(config.output_format(), &OutputFormat::MozJpeg);
    /// ```
    #[inline]
    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
    /// Get target width
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// config.set_target_width(Some(175)).unwrap();
    /// config.set_target_height(Some(175)).unwrap();
    ///
    /// assert_eq!(config.target_width(), Some(175));
    /// ```
    #[inline]
    pub fn target_width(&self) -> Option<usize> {
        self.target_width
    }
    /// Set target width
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    ///
    /// config.set_target_width(Some(150));
    ///
    /// assert_eq!(config.target_width(), Some(150));
    /// ```
    ///
    /// # Errors
    ///
    /// If width is equals to 0
    ///
    /// ```
    /// # use rimage::{Config, OutputFormat};
    /// # let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert!(config.set_target_width(Some(0)).is_err());
    /// ```
    #[inline]
    pub fn set_target_width(&mut self, width: Option<usize>) -> Result<(), ConfigError> {
        if let Some(width) = width {
            if width == 0 {
                return Err(ConfigError::WidthIsZero);
            }
        }

        self.target_width = width;
        Ok(())
    }
    /// Get target height
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// config.set_target_width(Some(175)).unwrap();
    /// config.set_target_height(Some(175)).unwrap();
    ///
    /// assert_eq!(config.target_height(), Some(175));
    /// ```
    #[inline]
    pub fn target_height(&self) -> Option<usize> {
        self.target_height
    }
    /// Set target width
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    ///
    /// config.set_target_height(Some(150));
    ///
    /// assert_eq!(config.target_height(), Some(150));
    /// ```
    ///
    /// # Errors
    ///
    /// If height is equals to 0
    ///
    /// ```
    /// # use rimage::{Config, OutputFormat};
    /// # let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert!(config.set_target_height(Some(0)).is_err());
    /// ```
    #[inline]
    pub fn set_target_height(&mut self, height: Option<usize>) -> Result<(), ConfigError> {
        if let Some(height) = height {
            if height == 0 {
                return Err(ConfigError::HeightIsZero);
            }
        }

        self.target_height = height;
        Ok(())
    }
    /// Get target quantization quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert_eq!(config.quantization_quality(), None);
    /// ```
    #[inline]
    pub fn quantization_quality(&self) -> Option<u8> {
        self.quantization_quality
    }
    /// Set quantization quality
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    ///
    /// config.set_quantization_quality(Some(80));
    ///
    /// assert_eq!(config.quantization_quality(), Some(80));
    /// ```
    ///
    /// # Errors
    ///
    /// If quantization quality is not in range 0..=100
    ///
    /// ```
    /// # use rimage::{Config, OutputFormat};
    /// # let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert!(config.set_quantization_quality(Some(220)).is_err());
    /// ```
    #[inline]
    pub fn set_quantization_quality(&mut self, quality: Option<u8>) -> Result<(), ConfigError> {
        if let Some(quality) = quality {
            if !(0..=100).contains(&quality) {
                return Err(ConfigError::QuantizationQualityOutOfBounds);
            }
        }

        self.quantization_quality = quality;
        Ok(())
    }
    /// Get target dithering level
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert_eq!(config.dithering_level(), None);
    /// ```
    #[inline]
    pub fn dithering_level(&self) -> Option<f32> {
        self.dithering_level
    }
    /// Set dithering level
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    ///
    /// config.set_quality(80.0);
    ///
    /// assert_eq!(config.quality(), 80.0);
    /// ```
    ///
    /// # Errors
    ///
    /// If dithering level is not in range 0.0..=1.0
    ///
    /// ```
    /// # use rimage::{Config, OutputFormat};
    /// # let mut config = Config::build(75.0, OutputFormat::MozJpeg).unwrap();
    /// assert!(config.set_dithering_level(Some(30.0)).is_err());
    /// ```
    #[inline]
    pub fn set_dithering_level(&mut self, dithering_level: Option<f32>) -> Result<(), ConfigError> {
        if let Some(dithering_level) = dithering_level {
            if !(0.0..=1.0).contains(&dithering_level) {
                return Err(ConfigError::DitheringLevelOutOfBounds);
            }
        }

        self.dithering_level = dithering_level;
        Ok(())
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

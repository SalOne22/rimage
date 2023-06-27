use crate::{error::ConfigError, OutputFormat, ResizeType};

/// Config for image encoding
///
/// # Example
/// ```
/// use rimage::{Config, OutputFormat, ResizeType};
///
/// // Without resize
/// let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
///
/// // With resize
/// let config_resize = Config::build(75.0, OutputFormat::MozJpeg, Some(200), Some(200), Some(ResizeType::Lanczos3)).unwrap();
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
    pub quality: f32,
    /// Output format of image
    pub output_format: OutputFormat,
    /// Target width for output image
    pub target_width: Option<usize>,
    /// Target height for output image
    pub target_height: Option<usize>,
    /// Resize type for output image
    pub resize_type: Option<ResizeType>,
    /// Quantization quality of output image
    pub quantization_quality: Option<u8>,
    /// Dithering level for output image
    pub dithering_level: Option<f32>,
}

impl Config {
    /// Create new config
    ///
    /// # Example
    /// ```
    /// use rimage::{Config, OutputFormat, ResizeType};
    ///
    /// // Without resize
    /// let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
    ///
    /// // With resize
    /// let config_resize = Config::build(75.0, OutputFormat::MozJpeg, Some(200), Some(200), Some(ResizeType::Lanczos3)).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If quality is not in range 0.0..=100.0
    ///
    /// ```
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(200.0, OutputFormat::MozJpeg, None, None, None);
    /// assert!(config.is_err());
    /// ```
    pub fn build(
        quality: f32,
        output_format: OutputFormat,
        width: Option<usize>,
        height: Option<usize>,
        resize_type: Option<ResizeType>,
    ) -> Result<Self, ConfigError> {
        if !(0.0..=100.0).contains(&quality) {
            return Err(ConfigError::QualityOutOfBounds);
        }

        if let Some(width) = width {
            if width == 0 {
                return Err(ConfigError::WidthIsZero);
            }
        }
        if let Some(height) = height {
            if height == 0 {
                return Err(ConfigError::HeightIsZero);
            }
        }

        Ok(Config {
            quality,
            output_format,
            target_width: width,
            target_height: height,
            resize_type,
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
    /// let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
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
    /// use rimage::{Config, OutputFormat};
    ///
    /// let config = Config::build(75.0, OutputFormat::MozJpeg, None, None, None).unwrap();
    /// assert_eq!(config.output_format(), &OutputFormat::MozJpeg);
    /// ```
    #[inline]
    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
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

use crate::error::InvalidQuantizationConfig;

/// Configuration struct for image quantization.
///
/// The [`QuantizationConfig`] struct allows you to configure settings related to image quantization,
/// which is a process used in image compression. It includes parameters for controlling the quality
/// of quantization and the level of dithering applied to the image during the process.
///
/// # Examples
///
/// Creating a basic [`QuantizationConfig`] with default settings:
///
/// ```
/// use rimage::config::QuantizationConfig;
///
/// let config = QuantizationConfig::default();
/// ```
///
/// Creating a custom [`QuantizationConfig`] with specific settings:
///
/// ```
/// use rimage::config::QuantizationConfig;
///
/// let config = QuantizationConfig::new()
///     .with_quality(90).unwrap()
///     .with_dithering(0.75).unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantizationConfig {
    /// The quality level for image quantization, ranging from 0 to 100.
    quality: u8,

    /// The level of dithering applied during quantization, ranging from 0.0 to 1.0.
    dithering_level: f32,
}

impl QuantizationConfig {
    /// Creates a new [`QuantizationConfig`]. (alias for default)
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::QuantizationConfig;
    ///
    /// let config = QuantizationConfig::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the quality level for image quantization.
    ///
    /// # Parameters
    ///
    /// - `quality`: The quality level for image quantization, ranging from 0 to 100.
    ///
    /// # Returns
    ///
    /// Returns a modified [`QuantizationConfig`] with the specified quality level if it's valid.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidQuantizationConfig`] error if the quality level is out of bounds (not in the range 0-100).
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::QuantizationConfig;
    ///
    /// let config = QuantizationConfig::new().with_quality(90).unwrap();
    /// ```
    pub fn with_quality(mut self, quality: u8) -> Result<Self, InvalidQuantizationConfig> {
        if quality > 100 {
            return Err(InvalidQuantizationConfig::QualityOutOfBounds(quality));
        }

        self.quality = quality;
        Ok(self)
    }

    /// Sets the level of dithering for image quantization.
    ///
    /// # Parameters
    ///
    /// - `dithering`: The level of dithering applied during quantization, typically ranging from 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// Returns a modified [`QuantizationConfig`] with the specified dithering level if it's valid.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidQuantizationConfig`] error if the dithering level is out of bounds (not in the range 0.0-1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::QuantizationConfig;
    ///
    /// let config = QuantizationConfig::new().with_dithering(0.5).unwrap();
    /// ```
    pub fn with_dithering(mut self, dithering: f32) -> Result<Self, InvalidQuantizationConfig> {
        if !(0.0..=1.0).contains(&dithering) {
            return Err(InvalidQuantizationConfig::DitheringOutOfBounds(dithering));
        }

        self.dithering_level = dithering;
        Ok(self)
    }

    /// Gets the quality setting for quantization.
    ///
    /// # Returns
    ///
    /// Returns the quality level as a unsigned byte value in the range [0, 100].
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::QuantizationConfig;
    ///
    /// let config = QuantizationConfig::default();
    /// let quality = config.quality();
    ///
    /// assert_eq!(quality, 100);
    /// ```
    pub fn quality(&self) -> u8 {
        self.quality
    }

    /// Gets the dithering level for quantization.
    ///
    /// # Returns
    ///
    /// Returns the dithering level as a floating-point value in the range [0.0, 1.0].
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::QuantizationConfig;
    ///
    /// let config = QuantizationConfig::default();
    /// let dithering_level = config.dithering_level();
    ///
    /// assert_eq!(dithering_level, 1.0);
    /// ```
    pub fn dithering_level(&self) -> f32 {
        self.dithering_level
    }
}

impl Default for QuantizationConfig {
    /// Creates a default [`QuantizationConfig`] with a quality of 100 and a dithering level of 1.0.
    fn default() -> Self {
        Self {
            quality: 100,
            dithering_level: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    // Import the necessary dependencies from the code
    use super::*;

    #[test]
    fn new_quantization_config() {
        // Test creating a new QuantizationConfig with default settings
        let config = QuantizationConfig::new();

        assert_eq!(config.quality(), 100);
        assert_eq!(config.dithering_level(), 1.0);
    }

    #[test]
    fn configure_quality() {
        // Test configuring quality within bounds
        let config = QuantizationConfig::new().with_quality(90).unwrap();
        assert_eq!(config.quality(), 90);

        // Test configuring quality out of bounds
        let result = QuantizationConfig::new().with_quality(120);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Quality value 120 is out of bounds (0-100)."
        )
    }

    #[test]
    fn configure_dithering() {
        // Test configuring dithering within bounds
        let config = QuantizationConfig::new().with_dithering(0.5).unwrap();
        assert_eq!(config.dithering_level(), 0.5);

        // Test configuring dithering out of bounds
        let result = QuantizationConfig::new().with_dithering(1.5);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Dithering level 1.5 is out of bounds (0.0-1.0)."
        )
    }

    #[test]
    fn default_quantization_config() {
        // Test the default QuantizationConfig
        let config = QuantizationConfig::default();

        assert_eq!(config.quality(), 100);
        assert_eq!(config.dithering_level(), 1.0);
    }
}

use crate::error::InvalidEncoderConfig;

use super::codec::Codec;

#[cfg(feature = "quantization")]
use super::quantization_config::QuantizationConfig;
#[cfg(feature = "resizing")]
use super::resize_config::ResizeConfig;

/// Configuration struct for image encoding.
///
/// The [`EncoderConfig`] struct allows you to configure various settings for image encoding,
/// such as quality, codec, quantization, and resizing. You can create an [`EncoderConfig`] instance
/// using the provided methods, customize it with the desired settings, and use it to control
/// the image encoding process.
///
/// # Examples
///
/// Creating a basic [`EncoderConfig`] with default settings:
///
/// ```
/// # use rimage::config::Codec;
/// use rimage::config::EncoderConfig;
///
/// let config = EncoderConfig::default();
///
/// ```
///
/// Creating a custom [`EncoderConfig`] with specific settings:
///
/// ```
/// use rimage::config::{EncoderConfig, Codec};
///
/// let config = EncoderConfig::new(Codec::Png).with_quality(90.0).unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct EncoderConfig {
    /// The quality level for image encoding, ranging from 0.0 to 100.0.
    quality: f32,

    /// The codec used for image encoding.
    codec: Codec,

    /// Optional quantization configuration for fine-tuning image compression.
    #[cfg(feature = "quantization")]
    quantization: Option<QuantizationConfig>,

    /// Optional resizing configuration for adjusting image dimensions.
    #[cfg(feature = "resizing")]
    resize: Option<ResizeConfig>,
}

impl EncoderConfig {
    /// Creates a new [`EncoderConfig`] with the specified codec and quality level of 75.0.
    ///
    /// # Parameters
    ///
    /// - `codec`: The codec used for image encoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{EncoderConfig, Codec};
    ///
    /// let config = EncoderConfig::new(Codec::Png);
    /// ```
    #[inline]
    pub fn new(codec: Codec) -> Self {
        Self {
            quality: 75.0,
            codec,
            #[cfg(feature = "quantization")]
            quantization: None,
            #[cfg(feature = "resizing")]
            resize: None,
        }
    }

    /// Sets the quality level for image encoding.
    ///
    /// # Parameters
    ///
    /// - `quality`: The quality level for image encoding, ranging from 0.0 to 100.0.
    ///
    /// # Returns
    ///
    /// Returns a modified [`EncoderConfig`] with the specified quality level if it's valid.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidEncoderConfig`] error if the quality level is out of bounds (not in the range 0-100).
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{EncoderConfig, Codec};
    ///
    /// let config = EncoderConfig::new(Codec::MozJpeg).with_quality(90.0).unwrap();
    /// ```
    #[inline]
    pub fn with_quality(mut self, quality: f32) -> Result<Self, InvalidEncoderConfig> {
        if !(0.0..=100.0).contains(&quality) {
            return Err(InvalidEncoderConfig::QualityOutOfBounds(quality));
        }

        self.quality = quality;
        Ok(self)
    }

    /// Sets the quantization configuration for image encoding.
    ///
    /// Quantization is an optional step in image compression that can affect the image's visual quality.
    /// Use this method to specify the quantization parameters to fine-tune the compression process.
    ///
    /// # Parameters
    ///
    /// - `quantization`: A [`QuantizationConfig`] struct containing quantization settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{EncoderConfig, Codec, QuantizationConfig};
    ///
    /// let quantization_config = QuantizationConfig::new()
    ///     .with_quality(50).unwrap()
    ///     .with_dithering(0.75).unwrap();
    ///
    /// let config = EncoderConfig::new(Codec::Png)
    ///     .with_quantization(quantization_config);
    /// ```
    #[inline]
    #[cfg(feature = "quantization")]
    pub fn with_quantization(mut self, quantization: QuantizationConfig) -> Self {
        self.quantization = Some(quantization);
        self
    }

    /// Sets the resizing configuration for image encoding.
    ///
    /// Resizing is an optional step that allows you to adjust the dimensions of the image before encoding.
    /// Use this method to specify the resizing parameters to customize the image dimensions.
    ///
    /// # Parameters
    ///
    /// - `resize`: A [`ResizeConfig`] struct containing resizing settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::resize;
    /// use rimage::config::{EncoderConfig, Codec, ResizeConfig};
    ///
    /// let resize_config = ResizeConfig::new(resize::Type::Lanczos3)
    ///     .with_width(800)
    ///     .with_height(600);
    ///
    /// let config = EncoderConfig::new(Codec::Png)
    ///     .with_resize(resize_config);
    /// ```
    #[inline]
    #[cfg(feature = "resizing")]
    pub fn with_resize(mut self, resize: ResizeConfig) -> Self {
        self.resize = Some(resize);
        self
    }

    /// Gets the quality setting for image encoding.
    ///
    /// # Returns
    ///
    /// Returns the quality level as a floating-point value in the range [0.0, 100.0].
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::EncoderConfig;
    ///
    /// let config = EncoderConfig::default();
    /// let quality = config.quality();
    ///
    /// assert_eq!(quality, 75.0);
    /// ```
    #[inline]
    pub fn quality(&self) -> f32 {
        self.quality
    }

    /// Gets the codec used for image encoding.
    ///
    /// # Returns
    ///
    /// Returns a reference to the codec enum that specifies the image encoding format.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{EncoderConfig, Codec};
    ///
    /// let config = EncoderConfig::new(Codec::MozJpeg);
    /// let codec = config.codec();
    ///
    /// assert_eq!(codec, &Codec::MozJpeg);
    /// ```
    #[inline]
    pub fn codec(&self) -> &Codec {
        &self.codec
    }

    /// Gets the quantization configuration for image encoding, if specified.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a reference to the `QuantizationConfig` if quantization
    /// settings are configured. Returns `None` if quantization is not configured.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{EncoderConfig, QuantizationConfig};
    ///
    /// let quantization = QuantizationConfig::new().with_quality(90).unwrap();
    /// let config = EncoderConfig::default().with_quantization(quantization);
    ///
    /// if let Some(quantization_config) = config.quantization_config() {
    ///     assert_eq!(quantization_config.quality(), 90);
    /// }
    /// ```
    #[inline]
    #[cfg(feature = "quantization")]
    pub fn quantization_config(&self) -> Option<&QuantizationConfig> {
        self.quantization.as_ref()
    }

    /// Gets the resize configuration for image encoding, if specified.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a reference to the `ResizeConfig` if resize settings are
    /// configured. Returns `None` if resizing is not configured.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::resize;
    /// use rimage::config::{EncoderConfig, ResizeConfig};
    ///
    /// let resize = ResizeConfig::new(resize::Type::Lanczos3).with_width(800);
    /// let config = EncoderConfig::default().with_resize(resize);
    ///
    /// if let Some(resize_config) = config.resize_config() {
    ///     assert_eq!(resize_config.width(), Some(800));
    /// }
    /// ```
    #[inline]
    #[cfg(feature = "resizing")]
    pub fn resize_config(&self) -> Option<&ResizeConfig> {
        self.resize.as_ref()
    }
}

impl Default for EncoderConfig {
    /// Creates a default [`EncoderConfig`] with a quality of 75.0 and the MozJPEG codec.
    fn default() -> Self {
        Self::new(Codec::MozJpeg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_encoder_config() {
        let config = EncoderConfig::new(Codec::MozJpeg);

        assert_eq!(config.quality(), 75.0);
        assert_eq!(config.codec(), &Codec::MozJpeg);
        #[cfg(feature = "quantization")]
        assert!(config.quantization_config().is_none());
        #[cfg(feature = "resizing")]
        assert!(config.resize_config().is_none());
    }

    #[test]
    fn configure_quality() {
        let config = EncoderConfig::new(Codec::MozJpeg)
            .with_quality(90.0)
            .unwrap();

        assert_eq!(config.quality(), 90.0);

        // Test invalid quality level
        let result = EncoderConfig::new(Codec::MozJpeg).with_quality(120.0);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Quality value 120 is out of bounds (0.0-100.0)."
        )
    }

    #[test]
    #[cfg(feature = "quantization")]
    fn configure_quantization() {
        let quantization_config = QuantizationConfig::default();

        let config = EncoderConfig::new(Codec::Png).with_quantization(quantization_config);

        assert_eq!(config.quantization_config(), Some(&quantization_config));
    }

    #[test]
    #[cfg(feature = "resizing")]
    fn configure_resize() {
        let resize_config = ResizeConfig::default();

        let config = EncoderConfig::new(Codec::Png).with_resize(resize_config);

        assert!(config.resize_config().is_some());
    }

    #[test]
    fn default_encoder_config() {
        let config = EncoderConfig::default();

        assert_eq!(config.quality(), 75.0);
        assert_eq!(config.codec(), &Codec::MozJpeg);
        #[cfg(feature = "quantization")]
        assert!(config.quantization_config().is_none());
        #[cfg(feature = "resizing")]
        assert!(config.resize_config().is_none());
    }
}

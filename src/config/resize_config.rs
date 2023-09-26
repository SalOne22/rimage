use super::ResizeType;

/// Configuration struct for image resizing.
///
/// The [`ResizeConfig`] struct allows you to configure settings for image resizing, including
/// options for specifying the width, height, and the type of resizing filter to be applied.
/// Use this struct to customize how images are resized to meet your application's needs.
///
/// # Examples
///
/// Creating a basic [`ResizeConfig`] with default settings:
///
/// ```
/// use rimage::config::ResizeConfig;
///
/// let config = ResizeConfig::default();
/// ```
///
/// Creating a custom [`ResizeConfig`] with specific settings:
///
/// ```
/// use rimage::config::{ResizeConfig, ResizeType};
///
/// let config = ResizeConfig::new(ResizeType::Lanczos3)
///     .with_width(800)
///     .with_height(600);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResizeConfig {
    /// The target width for image resizing. `None` if not specified.
    width: Option<usize>,
    /// The target height for image resizing. `None` if not specified.
    height: Option<usize>,
    /// The type of resizing filter to be used.
    filter_type: ResizeType,
}

impl ResizeConfig {
    /// Creates a new [`ResizeConfig`] with the specified resizing filter type.
    ///
    /// # Parameters
    ///
    /// - `filter_type`: The type of resizing filter to be used (e.g., Lanczos3).
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{ResizeConfig, ResizeType};
    ///
    /// let config = ResizeConfig::new(ResizeType::Lanczos3);
    /// ```
    #[inline]
    pub fn new(filter_type: ResizeType) -> Self {
        Self {
            width: None,
            height: None,
            filter_type,
        }
    }

    /// Specifies the target width for image resizing.
    ///
    /// # Parameters
    ///
    /// - `width`: The target width for resizing.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{ResizeConfig, ResizeType};
    ///
    /// let config = ResizeConfig::new(ResizeType::Lanczos3)
    ///     .with_width(800);
    /// ```
    #[inline]
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Specifies the target height for image resizing.
    ///
    /// # Parameters
    ///
    /// - `height`: The target height for resizing.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{ResizeConfig, ResizeType};
    ///
    /// let config = ResizeConfig::new(ResizeType::Lanczos3)
    ///     .with_height(600);
    /// ```
    #[inline]
    pub fn with_height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Gets the width setting for image resizing, if specified.
    ///
    /// # Returns
    ///
    /// Returns an [`Option`] containing the width as a [`usize`] if width is configured.
    /// Returns [`None`] if the width is not configured, indicating that the image width should
    /// remain unchanged during resizing.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{ResizeConfig, ResizeType};
    ///
    /// let config = ResizeConfig::new(ResizeType::Lanczos3).with_width(800);
    ///
    /// if let Some(width) = config.width() {
    ///     assert_eq!(width, 800);
    /// }
    /// ```
    #[inline]
    pub fn width(&self) -> Option<usize> {
        self.width
    }

    /// Gets the height setting for image resizing, if specified.
    ///
    /// # Returns
    ///
    /// Returns an [`Option`] containing the height as a [`usize`] if height is configured.
    /// Returns [`None`] if the height is not configured, indicating that the image height should
    /// remain unchanged during resizing.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{ResizeConfig, ResizeType};
    ///
    /// let config = ResizeConfig::new(ResizeType::Lanczos3).with_height(600);
    ///
    /// if let Some(height) = config.height() {
    ///     assert_eq!(height, 600);
    /// }
    /// ```
    #[inline]
    pub fn height(&self) -> Option<usize> {
        self.height
    }

    /// Gets the type of image resizing algorithm used, if specified.
    ///
    /// # Returns
    ///
    /// Returns a reference to the [`ResizeType`] enum that represents the image resizing algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::config::{ResizeConfig, ResizeType};
    ///
    /// let config = ResizeConfig::new(ResizeType::Lanczos3);
    ///
    /// assert_eq!(config.filter_type(), ResizeType::Lanczos3);
    /// ```
    pub fn filter_type(&self) -> ResizeType {
        self.filter_type
    }
}

impl Default for ResizeConfig {
    /// Creates a default [`ResizeConfig`] with a default resizing filter type (Lanczos3).
    fn default() -> Self {
        Self::new(ResizeType::Lanczos3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_resize_config() {
        let resize_config = ResizeConfig::new(ResizeType::Lanczos3);
        assert_eq!(resize_config.width(), None);
        assert_eq!(resize_config.height(), None);
        assert_eq!(resize_config.filter_type(), ResizeType::Lanczos3);
    }

    #[test]
    fn default_resize_config() {
        let resize_config = ResizeConfig::default();
        assert_eq!(resize_config.width(), None);
        assert_eq!(resize_config.height(), None);
        assert_eq!(resize_config.filter_type(), ResizeType::Lanczos3);
    }

    #[test]
    fn resize_config_with_width() {
        let resize_config = ResizeConfig::new(ResizeType::Lanczos3).with_width(120);
        assert_eq!(resize_config.width(), Some(120));
        assert_eq!(resize_config.height(), None);
        assert_eq!(resize_config.filter_type(), ResizeType::Lanczos3);
    }

    #[test]
    fn resize_config_with_height() {
        let resize_config = ResizeConfig::new(ResizeType::Lanczos3).with_height(120);
        assert_eq!(resize_config.width(), None);
        assert_eq!(resize_config.height(), Some(120));
        assert_eq!(resize_config.filter_type(), ResizeType::Lanczos3);
    }
}

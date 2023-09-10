use std::fmt::Debug;

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
/// use rimage::resize;
/// use rimage::config::ResizeConfig;
///
/// let config = ResizeConfig::new(resize::Type::Lanczos3)
///     .with_width(800)
///     .with_height(600);
/// ```
pub struct ResizeConfig {
    /// The target width for image resizing. `None` if not specified.
    width: Option<usize>,
    /// The target height for image resizing. `None` if not specified.
    height: Option<usize>,
    /// The type of resizing filter to be used.
    filter_type: resize::Type,
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
    /// use rimage::resize;
    /// use rimage::config::ResizeConfig;
    ///
    /// let config = ResizeConfig::new(resize::Type::Lanczos3);
    /// ```
    #[inline]
    pub fn new(filter_type: resize::Type) -> Self {
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
    /// use rimage::resize;
    /// use rimage::config::ResizeConfig;
    ///
    /// let config = ResizeConfig::new(resize::Type::Lanczos3)
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
    /// use rimage::resize;
    /// use rimage::config::ResizeConfig;
    ///
    /// let config = ResizeConfig::new(resize::Type::Lanczos3)
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
    /// use rimage::resize;
    /// use rimage::config::ResizeConfig;
    ///
    /// let config = ResizeConfig::new(resize::Type::Lanczos3).with_width(800);
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
    /// use rimage::resize;
    /// use rimage::config::ResizeConfig;
    ///
    /// let config = ResizeConfig::new(resize::Type::Lanczos3).with_height(600);
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
    /// Returns a reference to the [`resize::Type`] enum that represents the image resizing algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::resize;
    /// use rimage::config::ResizeConfig;
    ///
    /// let config = ResizeConfig::new(resize::Type::Lanczos3);
    /// let resize_type = config.filter_type();
    /// ```
    pub fn filter_type(&self) -> resize::Type {
        match &self.filter_type {
            resize::Type::Point => resize::Type::Point,
            resize::Type::Triangle => resize::Type::Triangle,
            resize::Type::Catrom => resize::Type::Catrom,
            resize::Type::Mitchell => resize::Type::Mitchell,
            resize::Type::Lanczos3 => resize::Type::Lanczos3,
            //FIXME: because resize::Type does not implement Copy or Clone we use fallback Lanczos3
            resize::Type::Custom(_) => resize::Type::Lanczos3,
        }
    }
}

impl Clone for ResizeConfig {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            filter_type: self.filter_type(),
        }
    }
}

impl Debug for ResizeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResizeConfig")
            .field("width", &self.width)
            .field("height", &self.height)
            .field(
                "filter_type",
                &match self.filter_type {
                    resize::Type::Point => "point",
                    resize::Type::Triangle => "triangle",
                    resize::Type::Catrom => "catrom",
                    resize::Type::Mitchell => "mitchell",
                    resize::Type::Lanczos3 => "lanczos3",
                    resize::Type::Custom(_) => "custom",
                },
            )
            .finish()
    }
}

impl Default for ResizeConfig {
    /// Creates a default [`ResizeConfig`] with a default resizing filter type (Lanczos3).
    fn default() -> Self {
        Self::new(resize::Type::Lanczos3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_resize_config() {
        let resize_config = ResizeConfig::new(resize::Type::Lanczos3);
        assert_eq!(resize_config.width(), None);
        assert_eq!(resize_config.height(), None);
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: None, height: None, filter_type: \"lanczos3\" }"
        );
    }

    #[test]
    fn default_resize_config() {
        let resize_config = ResizeConfig::default();
        assert_eq!(resize_config.width(), None);
        assert_eq!(resize_config.height(), None);
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: None, height: None, filter_type: \"lanczos3\" }"
        );
    }

    #[test]
    fn resize_config_with_width() {
        let resize_config = ResizeConfig::new(resize::Type::Lanczos3).with_width(120);
        assert_eq!(resize_config.width(), Some(120));
        assert_eq!(resize_config.height(), None);
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: Some(120), height: None, filter_type: \"lanczos3\" }"
        );
    }

    #[test]
    fn resize_config_with_height() {
        let resize_config = ResizeConfig::new(resize::Type::Lanczos3).with_height(120);
        assert_eq!(resize_config.width(), None);
        assert_eq!(resize_config.height(), Some(120));
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: None, height: Some(120), filter_type: \"lanczos3\" }"
        );
    }

    #[test]
    fn clone_resize_config() {
        let resize_config = ResizeConfig::new(resize::Type::Lanczos3)
            .with_width(120)
            .with_height(120);

        let cloned_resize_config = resize_config.clone();

        assert_eq!(resize_config.width(), cloned_resize_config.width());
        assert_eq!(resize_config.height(), cloned_resize_config.height());
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: Some(120), height: Some(120), filter_type: \"lanczos3\" }"
        );
        assert_eq!(
            format!("{cloned_resize_config:?}"),
            "ResizeConfig { width: Some(120), height: Some(120), filter_type: \"lanczos3\" }"
        );
    }

    #[test]
    fn debug_resize_config() {
        let resize_config = ResizeConfig::new(resize::Type::Lanczos3)
            .with_width(120)
            .with_height(120);

        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: Some(120), height: Some(120), filter_type: \"lanczos3\" }"
        );

        let resize_config = ResizeConfig::new(resize::Type::Catrom);
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: None, height: None, filter_type: \"catrom\" }"
        );

        let resize_config = ResizeConfig::new(resize::Type::Mitchell);
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: None, height: None, filter_type: \"mitchell\" }"
        );

        let resize_config = ResizeConfig::new(resize::Type::Point);
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: None, height: None, filter_type: \"point\" }"
        );

        let resize_config = ResizeConfig::new(resize::Type::Triangle);
        assert_eq!(
            format!("{resize_config:?}"),
            "ResizeConfig { width: None, height: None, filter_type: \"triangle\" }"
        );
    }
}

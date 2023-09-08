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
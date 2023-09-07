use rgb::RGBA8;

use crate::config::{QuantizationConfig, ResizeConfig};

/// Struct representing an image with RGBA8 pixel data.
pub struct Image {
    data: Vec<RGBA8>,
    width: usize,
    height: usize,
}

impl Image {
    /// Creates a new [`Image`] instance with the given pixel data, width, and height.
    ///
    /// # Parameters
    ///
    /// - `data`: A vector containing RGBA8 pixel data.
    /// - `width`: The width of the image in pixels.
    /// - `height`: The height of the image in pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use rimage::Image;
    /// use rgb::RGBA8;
    ///
    /// let pixel_data: Vec<RGBA8> = vec![/* pixel data */];
    /// let image = Image::new(pixel_data, 800, 600);
    /// ```
    #[inline]
    pub fn new(data: Vec<RGBA8>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    /// Resizes the image using the specified [`ResizeConfig`].
    ///
    /// # Parameters
    ///
    /// - `resize_config`: The configuration for resizing the image.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or a [`resize::Error`] on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rimage::rgb::RGBA8;
    /// use rimage::{Image, config::ResizeConfig};
    ///
    /// let image_data = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    /// let mut image = Image::new(image_data, 800, 600);
    ///
    /// let resize_config = ResizeConfig::default()
    ///     .with_width(400);
    ///
    /// image.resize(&resize_config)?;
    /// # Ok::<(), rimage::resize::Error>(())
    /// ```
    pub fn resize(&mut self, resize_config: &ResizeConfig) -> Result<(), resize::Error> {
        let aspect_ratio = self.width as f64 / self.height as f64;

        let width = resize_config.width().unwrap_or(
            resize_config
                .height()
                .map(|h| (h as f64 * aspect_ratio) as usize)
                .unwrap_or(self.width),
        );
        let height = resize_config.height().unwrap_or(
            resize_config
                .width()
                .map(|w| (w as f64 / aspect_ratio) as usize)
                .unwrap_or(self.height),
        );

        let mut buf: Vec<RGBA8> = vec![RGBA8::new(0, 0, 0, 0); width * height];

        let mut resizer = resize::new(
            self.width,
            self.height,
            width,
            height,
            resize::Pixel::RGBA8,
            resize_config.filter_type(),
        )?;

        resizer.resize(&self.data, &mut buf)?;

        self.data = buf;
        self.width = width;
        self.height = height;

        Ok(())
    }

    /// Quantizes the image using the specified [`QuantizationConfig`].
    ///
    /// # Parameters
    ///
    /// - `quantization_config`: The configuration for quantizing the image.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an [`imagequant::Error`] on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rimage::rgb::RGBA8;
    /// use rimage::{Image, config::QuantizationConfig};
    ///
    /// let image_data = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    /// let mut image = Image::new(image_data, 800, 600);
    ///
    /// let quantization_config = QuantizationConfig::default();
    /// image.quantize(&quantization_config).unwrap();
    /// ```
    pub fn quantize(
        &mut self,
        quantization_config: &QuantizationConfig,
    ) -> Result<(), imagequant::Error> {
        self.data = {
            let mut liq = imagequant::new();

            liq.set_speed(5)?;
            liq.set_quality(0, quantization_config.quality())?;

            let mut img = liq.new_image_borrowed(&self.data, self.width, self.height, 0.0)?;

            let mut res = liq.quantize(&mut img)?;

            res.set_dithering_level(quantization_config.dithering_level())?;

            let (palette, pixels) = res.remapped(&mut img)?;

            pixels.iter().map(|pix| palette[*pix as usize]).collect()
        };

        Ok(())
    }

    /// Fixes the orientation of the image based on the given orientation value.
    ///
    /// This method applies various transformations to correct the orientation of the image.
    ///
    /// # Parameters
    ///
    /// - `orientation`: An integer value representing the image orientation. It should be a value
    ///   between 1 and 8, inclusive.
    ///
    /// # Example
    ///
    /// ```no run
    /// use rimage::Image;
    ///
    /// let mut image = Image::new(/* ... */);
    /// image.fix_orientation(3); // Fix the orientation of the image
    /// ```
    pub fn fix_orientation(&mut self, orientation: u32) {
        if orientation > 8 {
            return;
        }

        let orientation = orientation - 1;

        if orientation & 0b100 != 0 {
            self.flip_diagonally();
        }

        if orientation & 0b010 != 0 {
            self.rotate_180();
        }

        if orientation & 0b001 != 0 {
            self.flip_horizontally();
        }
    }

    /// Flips the image diagonally.
    ///
    /// This method performs a diagonal flip (transpose) of the image data and updates the image dimensions accordingly.
    ///
    /// # Example
    ///
    /// ```no run
    /// use rimage::Image;
    ///
    /// let mut image = Image::new(/* ... */);
    /// image.flip_diagonally(); // Flip the image diagonally
    /// ```
    fn flip_diagonally(&mut self) {
        let mut buf = vec![RGBA8::new(0, 0, 0, 0); self.data.len()];

        transpose::transpose(&self.data, &mut buf, self.width, self.height);

        self.data = buf;
        (self.width, self.height) = (self.height, self.width);
    }

    /// Flips the image horizontally.
    ///
    /// This method performs a horizontal flip of the image data.
    ///
    /// # Example
    ///
    /// ```no run
    /// use rimage::Image;
    ///
    /// let mut image = Image::new(/* ... */);
    /// image.flip_horizontally(); // Flip the image horizontally
    /// ```
    pub fn flip_horizontally(&mut self) {
        for y in 0..self.height {
            let start = y * self.width;

            self.data[start..start + self.width].reverse();
        }
    }

    /// Rotates the image 90 degrees clockwise.
    ///
    /// This method rotates the image 90 degrees clockwise by performing a diagonal flip followed by a horizontal flip.
    ///
    /// # Example
    ///
    /// ```no run
    /// use rimage::Image;
    ///
    /// let mut image = Image::new(/* ... */);
    /// image.rotate_90(); // Rotate the image 90 degrees clockwise
    /// ```
    pub fn rotate_90(&mut self) {
        self.flip_diagonally();
        self.flip_horizontally();
    }

    /// Rotates the image 180 degrees clockwise.
    ///
    /// This method rotates the image 180 degrees clockwise by performing two consecutive 90-degree clockwise rotations.
    ///
    /// # Example
    ///
    /// ```no run
    /// use rimage::Image;
    ///
    /// let mut image = Image::new(/* ... */);
    /// image.rotate_180(); // Rotate the image 180 degrees clockwise
    /// ```
    pub fn rotate_180(&mut self) {
        self.rotate_90();
        self.rotate_90();
    }

    /// Gets a reference to the pixel data of the image.
    ///
    /// # Returns
    ///
    /// Returns a reference to the RGBA8 pixel data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rimage::rgb::RGBA8;
    /// use rimage::Image;
    ///
    /// let image_data = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    /// let image = Image::new(image_data, 800, 600);
    /// let data_reference = image.data();
    /// ```
    #[inline]
    pub fn data(&self) -> &[RGBA8] {
        &self.data
    }

    /// Gets the width of the image in pixels.
    ///
    /// # Returns
    ///
    /// Returns the width of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rimage::rgb::RGBA8;
    /// use rimage::Image;
    ///
    /// let image_data = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    /// let image = Image::new(image_data, 800, 600);
    /// let width = image.width();
    ///
    /// assert_eq!(width, 800)
    /// ```
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height of the image in pixels.
    ///
    /// # Returns
    ///
    /// Returns the height of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rimage::rgb::RGBA8;
    /// use rimage::Image;
    ///
    /// let image_data = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    /// let image = Image::new(image_data, 800, 600);
    /// let height = image.height();
    ///
    /// assert_eq!(height, 600)
    /// ```
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests;

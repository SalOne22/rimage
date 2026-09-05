//! SVG decoder rendering vector images into raster pixels through `resvg`.

use std::io::Read;
use std::path::PathBuf;

use resvg::tiny_skia;
use resvg::usvg;
use zune_core::colorspace::ColorSpace;
use zune_image::errors::ImageErrors;
use zune_image::image::Image;
use zune_image::traits::DecoderTrait;

use super::fonts;

/// Options controlling how an SVG image is rendered into pixels.
#[derive(Clone, Debug)]
pub struct SvgOptions {
    /// Directory used to resolve relative paths inside the SVG, such as the
    /// `href` of an `<image>` element.
    ///
    /// Should be set to the directory containing the SVG file.
    pub resources_dir: Option<PathBuf>,
    /// Uniform scale factor applied to the SVG intrinsic size.
    ///
    /// Ignored when [`SvgOptions::width`] or [`SvgOptions::height`] is set.
    /// Must be positive and finite. Defaults to `1.0`.
    pub scale: f32,
    /// Target width in pixels. When set without [`SvgOptions::height`], the
    /// height is derived while keeping the aspect ratio of the SVG.
    pub width: Option<u32>,
    /// Target height in pixels. When set without [`SvgOptions::width`], the
    /// width is derived while keeping the aspect ratio of the SVG.
    pub height: Option<u32>,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self {
            resources_dir: None,
            scale: 1.0,
            width: None,
            height: None,
        }
    }
}

/// A decoder that renders SVG images into raster pixels using `resvg`.
///
/// Scaling happens while rendering, so any target size keeps the vector
/// quality of the source instead of resampling a rasterized image.
pub struct SvgDecoder {
    tree: usvg::Tree,
    intrinsic: (f32, f32),
    target: (usize, usize),
}

impl SvgDecoder {
    /// Create a new SVG decoder with default render options.
    pub fn try_new<R: Read>(source: R) -> Result<Self, ImageErrors> {
        Self::try_new_with_options(source, SvgOptions::default())
    }

    /// Create a new SVG decoder with custom render options.
    pub fn try_new_with_options<R: Read>(
        mut source: R,
        options: SvgOptions,
    ) -> Result<Self, ImageErrors> {
        let mut data = Vec::new();
        source.read_to_end(&mut data).map_err(|e| {
            ImageErrors::ImageDecodeErrors(format!("Unable to read SVG data - {e}"))
        })?;

        let mut usvg_options = usvg::Options {
            resources_dir: options.resources_dir.clone(),
            font_resolver: fonts::font_resolver(),
            ..usvg::Options::default()
        };
        usvg_options.fontdb = fonts::system_fontdb();

        // `Tree::from_data` detects and decompresses gzip (SVGZ) automatically.
        let tree = usvg::Tree::from_data(&data, &usvg_options)
            .map_err(|e| ImageErrors::ImageDecodeErrors(format!("Unable to parse SVG - {e}")))?;

        let size = tree.size();
        let target = resolve_target_size(&options, size)?;

        Ok(Self {
            tree,
            intrinsic: (size.width(), size.height()),
            target,
        })
    }
}

/// Resolves the pixel size the SVG should be rendered at from the requested
/// options and the intrinsic SVG size.
fn resolve_target_size(
    options: &SvgOptions,
    size: usvg::Size,
) -> Result<(usize, usize), ImageErrors> {
    if !options.scale.is_finite() || options.scale <= 0.0 {
        return Err(ImageErrors::ImageDecodeErrors(format!(
            "Invalid SVG scale factor {}",
            options.scale
        )));
    }

    let scaled = match (options.width, options.height) {
        (Some(width), Some(height)) => usvg::Size::from_wh(width as f32, height as f32),
        (Some(width), None) => size.scale_to_width(width as f32),
        (None, Some(height)) => size.scale_to_height(height as f32),
        (None, None) => size.scale_by(options.scale),
    }
    .ok_or_else(|| ImageErrors::ImageDecodeErrors("Invalid SVG target size".to_string()))?;

    // Rounds and clamps to at least 1x1.
    let target = scaled.to_int_size();

    Ok((target.width() as usize, target.height() as usize))
}

impl DecoderTrait for SvgDecoder {
    fn decode(&mut self) -> Result<Image, ImageErrors> {
        let (width, height) = self.target;

        let mut pixmap = tiny_skia::Pixmap::new(width as u32, height as u32).ok_or_else(|| {
            ImageErrors::ImageDecodeErrors(format!(
                "Unable to allocate a {width}x{height} pixmap for SVG rendering"
            ))
        })?;

        // Scaling happens here, so the vectors are rasterized directly at the
        // target resolution instead of resampling a smaller image.
        let transform = tiny_skia::Transform::from_scale(
            width as f32 / self.intrinsic.0,
            height as f32 / self.intrinsic.1,
        );

        resvg::render(&self.tree, transform, &mut pixmap.as_mut());

        // tiny-skia stores premultiplied alpha while zune_image expects
        // straight alpha.
        let mut pixels = Vec::with_capacity(width * height * 4);
        for pixel in pixmap.pixels() {
            let color = pixel.demultiply();
            pixels.extend_from_slice(&[color.red(), color.green(), color.blue(), color.alpha()]);
        }

        Ok(Image::from_u8(&pixels, width, height, ColorSpace::RGBA))
    }

    fn dimensions(&self) -> Option<(usize, usize)> {
        Some(self.target)
    }

    fn out_colorspace(&self) -> ColorSpace {
        ColorSpace::RGBA
    }

    fn name(&self) -> &'static str {
        "svg-decoder"
    }
}

#[cfg(test)]
mod tests;

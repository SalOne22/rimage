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

/// Maximum number of pixels an SVG render target may cover (2^28, ~268 MP).
///
/// At 4 bytes per pixel this bounds the render pixmap and the straight-alpha
/// copy at ~1 GiB each, plus ~1 GiB of interleaved channels in the resulting
/// [`Image`], which keeps worst-case transient memory bounded regardless of
/// the SVG's declared size or the provided render options.
pub const MAX_TARGET_PIXELS: u64 = 1 << 28;

/// Options controlling how an SVG image is rendered into pixels.
#[derive(Clone, Debug)]
pub struct SvgOptions {
    /// Directory used to resolve relative paths inside the SVG, such as the
    /// `href` of an `<image>` element.
    ///
    /// Should be set to the directory containing the SVG file.
    pub resources_dir: Option<PathBuf>,
    /// Explicit render target in pixels. When `None`, the SVG is rendered
    /// at its intrinsic size.
    ///
    /// The resolved render target may not exceed [`MAX_TARGET_PIXELS`] pixels.
    pub target_size: Option<(u32, u32)>,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self {
            resources_dir: None,
            target_size: None,
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

/// Parses an SVG document into a `resvg` tree.
///
/// Reads the whole source, configures `resvg` with the SVG's resource
/// directory and the process-wide system font database, and parses the
/// document. `Tree::from_data` detects and decompresses gzip (SVGZ)
/// automatically.
fn parse_tree<R: Read>(
    mut source: R,
    resources_dir: Option<PathBuf>,
) -> Result<usvg::Tree, ImageErrors> {
    let mut data = Vec::new();
    source
        .read_to_end(&mut data)
        .map_err(|e| ImageErrors::ImageDecodeErrors(format!("Unable to read SVG data - {e}")))?;

    let mut usvg_options = usvg::Options {
        resources_dir,
        font_resolver: fonts::font_resolver(),
        ..usvg::Options::default()
    };
    usvg_options.fontdb = fonts::system_fontdb();

    usvg::Tree::from_data(&data, &usvg_options)
        .map_err(|e| ImageErrors::ImageDecodeErrors(format!("Unable to parse SVG - {e}")))
}

impl SvgDecoder {
    /// Create a new SVG decoder with default render options.
    pub fn try_new<R: Read>(source: R) -> Result<Self, ImageErrors> {
        Self::try_new_with_options(source, SvgOptions::default())
    }

    /// Parses the SVG once and resolves the render target through a caller
    /// supplied callback.
    ///
    /// The callback receives the intrinsic SVG size as integer pixels and returns
    /// the desired render target, or `Ok(None)` to render at the intrinsic size.
    /// This avoids parsing the input twice when callers need to compute a resize
    /// target from the intrinsic dimensions.
    pub fn try_new_with_resize<R, F>(
        source: R,
        resources_dir: Option<PathBuf>,
        target_for_size: F,
    ) -> Result<Self, ImageErrors>
    where
        R: Read,
        F: FnOnce((usize, usize)) -> Result<Option<(u32, u32)>, ImageErrors>,
    {
        let tree = parse_tree(source, resources_dir.clone())?;
        let size = tree.size();
        let intrinsic = (
            (size.width().round() as usize).max(1),
            (size.height().round() as usize).max(1),
        );
        let target_size = target_for_size(intrinsic)?;
        let target = resolve_target_size(
            &SvgOptions {
                resources_dir,
                target_size,
            },
            size,
        )?;

        Ok(Self {
            tree,
            intrinsic: (size.width(), size.height()),
            target,
        })
    }

    /// Returns the intrinsic SVG size in pixels without rendering the image.
    ///
    /// Unlike [`SvgDecoder::try_new_with_options`], this does not validate
    /// [`MAX_TARGET_PIXELS`]; callers can use it to compute a resize target
    /// before seeking back and constructing the real decoder.
    pub fn probe_size<R: Read>(
        source: R,
        resources_dir: Option<PathBuf>,
    ) -> Result<(f32, f32), ImageErrors> {
        let tree = parse_tree(source, resources_dir)?;
        let size = tree.size();
        Ok((size.width(), size.height()))
    }

    /// Create a new SVG decoder with custom render options.
    pub fn try_new_with_options<R: Read>(
        source: R,
        options: SvgOptions,
    ) -> Result<Self, ImageErrors> {
        let tree = parse_tree(source, options.resources_dir.clone())?;

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
    let target = match options.target_size {
        Some((width, height)) => {
            if width == 0 || height == 0 {
                return Err(ImageErrors::ImageDecodeErrors(format!(
                    "Invalid SVG target size {width}x{height}"
                )));
            }
            (width as u32, height as u32)
        }
        None => {
            let intrinsic = size.to_int_size();
            (intrinsic.width(), intrinsic.height())
        }
    };

    // Clamp both dimensions to at least 1x1.
    let width = target.0.max(1) as usize;
    let height = target.1.max(1) as usize;

    // The area is computed in u64 because width and height can each approach
    // u32::MAX, whose product overflows usize.
    let area = (width as u64) * (height as u64);
    if area > MAX_TARGET_PIXELS {
        return Err(ImageErrors::ImageDecodeErrors(format!(
            "SVG target size {width}x{height} ({area} pixels) exceeds the limit of {MAX_TARGET_PIXELS} pixels, reduce the --resize target or the intrinsic size",
        )));
    }

    Ok((width, height))
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

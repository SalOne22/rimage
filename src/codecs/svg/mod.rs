//! SVG input support rendering vector images through [`resvg`].

pub mod decoder;

pub(crate) mod fonts;

pub use decoder::{SvgDecoder, SvgOptions};

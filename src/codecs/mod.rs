/// AVIF encoding support
#[cfg(feature = "avif")]
pub mod avif;

/// Jpegli encoding support
#[cfg(feature = "jpegli")]
pub mod jpegli;

/// OxiPNG encoding support
#[cfg(feature = "oxipng")]
pub mod oxipng;

/// WebP encoding support
#[cfg(feature = "webp")]
pub mod webp;

/// Quantization operation
///
/// This can be used to reduce image palette by performing quantization operation.
#[cfg(feature = "quantization")]
pub mod quantize;
/// Resize an image to a new dimensions
#[cfg(feature = "resize")]
pub mod resize;

/// Operations to apply icc profiles
#[cfg(feature = "icc")]
pub mod icc;

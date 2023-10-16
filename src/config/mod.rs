mod codec;
mod encoder_config;
mod image_format;

#[cfg(feature = "quantization")]
mod quantization_config;
#[cfg(feature = "resizing")]
mod resize_config;
#[cfg(feature = "resizing")]
mod resize_type;

pub use codec::Codec;
pub use encoder_config::EncoderConfig;
pub use image_format::ImageFormat;

#[cfg(feature = "quantization")]
pub use quantization_config::QuantizationConfig;
#[cfg(feature = "resizing")]
pub use resize_config::ResizeConfig;
#[cfg(feature = "resizing")]
pub use resize_type::ResizeType;

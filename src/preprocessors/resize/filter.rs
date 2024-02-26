use anyhow::anyhow;
use clap::{builder::PossibleValue, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeFilter {
    Box,
    Bilinear,
    Hamming,
    CatmullRom,
    Mitchell,
    Lanczos3,
}

impl ValueEnum for ResizeFilter {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ResizeFilter::Box,
            ResizeFilter::Bilinear,
            ResizeFilter::Hamming,
            ResizeFilter::CatmullRom,
            ResizeFilter::Mitchell,
            ResizeFilter::Lanczos3,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            ResizeFilter::Box => PossibleValue::new("box").help("Each pixel contributes equally to destination. For upscaling, like Nearest."),
            ResizeFilter::Bilinear => PossibleValue::new("bilinear").help("Uses linear interpolation among contributing pixels for output"),
            ResizeFilter::Hamming => PossibleValue::new("hamming").help("Provides quality akin to bicubic for downscaling, sharper than Bilinear, but not optimal for upscaling"),
            ResizeFilter::CatmullRom => PossibleValue::new("catmull-rom").help("Employs cubic interpolation for output pixel calculation"),
            ResizeFilter::Mitchell => PossibleValue::new("mitchell").help("Utilizes cubic interpolation for output pixel calculation"),
            ResizeFilter::Lanczos3 => PossibleValue::new("lanczos3").help("Applies high-quality Lanczos filter for output pixel calculation"),
        })
    }
}

impl std::fmt::Display for ResizeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for ResizeFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(anyhow!("invalid variant: {s}"))
    }
}

use anyhow::anyhow;
use clap::{builder::PossibleValue, ValueEnum};

#[derive(Clone, Copy)]
pub enum ResizeFit {
    Stretch,
    Cover,
}

impl ValueEnum for ResizeFit {
    fn value_variants<'a>() -> &'a [Self] {
        &[ResizeFit::Stretch, ResizeFit::Cover]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            ResizeFit::Stretch => PossibleValue::new("stretch")
                .help("Stretches resulting image to fit new dimensions"),
            ResizeFit::Cover => PossibleValue::new("cover")
                .help("Resulting image is sized to maintain its aspect ratio. Clipping to fit"),
        })
    }
}

impl std::fmt::Display for ResizeFit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for ResizeFit {
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

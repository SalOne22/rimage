use std::str::FromStr;

/// Thin wrapper around [`resize::Type`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeType {
    /// Wrapper around [`resize::Type::Point`]
    Point,
    /// Wrapper around [`resize::Type::Triangle`]
    Triangle,
    /// Wrapper around [`resize::Type::Catrom`]
    CatmullRom,
    /// Wrapper around [`resize::Type::Mitchell`]
    Mitchell,
    /// Wrapper around [`resize::Type::Lanczos3`]
    Lanczos3,
}

impl From<ResizeType> for resize::Type {
    fn from(val: ResizeType) -> Self {
        match val {
            ResizeType::Point => resize::Type::Point,
            ResizeType::Triangle => resize::Type::Triangle,
            ResizeType::CatmullRom => resize::Type::Catrom,
            ResizeType::Mitchell => resize::Type::Mitchell,
            ResizeType::Lanczos3 => resize::Type::Lanczos3,
        }
    }
}

impl FromStr for ResizeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "point" => Self::Point,
            "triangle" => Self::Triangle,
            "catmull-rom" | "catrom" => Self::CatmullRom,
            "mitchell" => Self::Mitchell,
            "lanczos3" => Self::Lanczos3,
            filter => return Err(format!("{filter} is not valid resize filter")),
        })
    }
}

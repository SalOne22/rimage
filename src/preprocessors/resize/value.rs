use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq)]
pub enum ResizeValue {
    Multiplier(f32),
    Percentage(f32),
    Dimensions(Option<u32>, Option<u32>),
}

impl std::fmt::Display for ResizeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResizeValue::Multiplier(multiplier) => f.write_fmt(format_args!("@{multiplier}")),
            ResizeValue::Percentage(percentage) => f.write_fmt(format_args!("{percentage}%")),
            ResizeValue::Dimensions(Some(width), Some(height)) => {
                f.write_fmt(format_args!("{width}x{height}"))
            }
            ResizeValue::Dimensions(Some(width), None) => f.write_fmt(format_args!("{width}x_")),
            ResizeValue::Dimensions(None, Some(height)) => f.write_fmt(format_args!("_x{height}")),
            ResizeValue::Dimensions(None, None) => f.write_fmt(format_args!("base")),
        }
    }
}

impl std::str::FromStr for ResizeValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.starts_with('@') => Ok(Self::Multiplier(s[1..].parse()?)),
            s if s.ends_with('%') => Ok(Self::Percentage(s[..s.len() - 1].parse()?)),
            s if s.contains('x') => {
                let dimensions: Vec<&str> = s.split('x').collect();
                if dimensions.len() > 2 {
                    return Err(anyhow!("There is more that 2 dimensions"));
                }

                let width = if dimensions[0] == "_" {
                    None
                } else {
                    Some(dimensions[0].parse::<u32>()?)
                };

                let height = if dimensions[1] == "_" {
                    None
                } else {
                    Some(dimensions[1].parse::<u32>()?)
                };

                Ok(Self::Dimensions(width, height))
            }
            _ => Err(anyhow!("Invalid resize value")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(ResizeValue::Multiplier(2.).to_string(), "@2");
        assert_eq!(ResizeValue::Multiplier(1.5).to_string(), "@1.5");

        assert_eq!(ResizeValue::Percentage(200.).to_string(), "200%");
        assert_eq!(ResizeValue::Percentage(150.).to_string(), "150%");

        assert_eq!(
            ResizeValue::Dimensions(Some(200), Some(200)).to_string(),
            "200x200"
        );
        assert_eq!(
            ResizeValue::Dimensions(Some(150), Some(150)).to_string(),
            "150x150"
        );

        assert_eq!(
            ResizeValue::Dimensions(None, Some(200)).to_string(),
            "_x200"
        );
        assert_eq!(
            ResizeValue::Dimensions(None, Some(150)).to_string(),
            "_x150"
        );

        assert_eq!(
            ResizeValue::Dimensions(Some(200), None).to_string(),
            "200x_"
        );
        assert_eq!(
            ResizeValue::Dimensions(Some(150), None).to_string(),
            "150x_"
        );

        assert_eq!(ResizeValue::Dimensions(None, None).to_string(), "base");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "@2".parse::<ResizeValue>().unwrap(),
            ResizeValue::Multiplier(2.)
        );

        assert_eq!(
            "@1.5".parse::<ResizeValue>().unwrap(),
            ResizeValue::Multiplier(1.5)
        );

        assert_eq!(
            "200%".parse::<ResizeValue>().unwrap(),
            ResizeValue::Percentage(200.)
        );

        assert_eq!(
            "150%".parse::<ResizeValue>().unwrap(),
            ResizeValue::Percentage(150.)
        );

        assert_eq!(
            "200x200".parse::<ResizeValue>().unwrap(),
            ResizeValue::Dimensions(Some(200), Some(200))
        );

        assert_eq!(
            "150x150".parse::<ResizeValue>().unwrap(),
            ResizeValue::Dimensions(Some(150), Some(150))
        );

        assert_eq!(
            "200x_".parse::<ResizeValue>().unwrap(),
            ResizeValue::Dimensions(Some(200), None)
        );

        assert_eq!(
            "150x_".parse::<ResizeValue>().unwrap(),
            ResizeValue::Dimensions(Some(150), None)
        );

        assert_eq!(
            "_x200".parse::<ResizeValue>().unwrap(),
            ResizeValue::Dimensions(None, Some(200))
        );

        assert_eq!(
            "_x150".parse::<ResizeValue>().unwrap(),
            ResizeValue::Dimensions(None, Some(150))
        );

        assert_eq!(
            "_x_".parse::<ResizeValue>().unwrap(),
            ResizeValue::Dimensions(None, None)
        );
    }
}

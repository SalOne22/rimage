use std::{error::Error, fs, io, panic, path};

use mozjpeg::Decompress;
use rgb::{FromSlice, RGB8, RGBA8};

/// Decodes image to (pixels, width, height)
pub fn decode_image(path: &path::PathBuf) -> Result<(Vec<RGBA8>, usize, usize), Box<dyn Error>> {
    let input_format = path.extension().unwrap();
    let decoded = match input_format.to_str() {
        Some("jpg") => decode_jpeg(path)?,
        Some("png") => decode_png(path)?,
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "File not supported",
            )))
        }
    };

    Ok(decoded)
}

fn decode_jpeg(path: &path::PathBuf) -> Result<(Vec<RGBA8>, usize, usize), Box<dyn Error>> {
    panic::catch_unwind(|| -> Result<(Vec<RGBA8>, usize, usize), Box<dyn Error>> {
        let file_content = fs::read(path)?;
        let d = Decompress::new_mem(&file_content)?;
        let mut image = d.rgb()?;
        let width = image.width();
        let height = image.height();
        let pixels: Vec<RGB8> = image.read_scanlines().unwrap();
        let pixels = pixels.iter().map(|pixel| pixel.alpha(255)).collect();

        assert!(image.finish_decompress());
        Ok((pixels, width, height))
    })
    .unwrap_or(Err(Box::new(io::Error::new(
        io::ErrorKind::InvalidData,
        "Failed to read jpeg",
    ))))
}

fn decode_png(path: &path::PathBuf) -> Result<(Vec<RGBA8>, usize, usize), io::Error> {
    let d = png::Decoder::new(fs::File::open(path)?);
    let mut reader = d.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    let bytes = &buf[..info.buffer_size()];
    let pixels: Vec<RGBA8> = match info.color_type {
        png::ColorType::Grayscale => bytes
            .as_gray()
            .iter()
            .map(|pixel| RGBA8::new(pixel.0, pixel.0, pixel.0, 255))
            .collect(),
        png::ColorType::Rgb => bytes
            .as_rgb()
            .iter()
            .map(|pixel| pixel.alpha(255))
            .collect(),
        png::ColorType::GrayscaleAlpha => bytes
            .as_gray_alpha()
            .iter()
            .map(|pixel| RGBA8::new(pixel.0, pixel.0, pixel.0, pixel.1))
            .collect(),
        png::ColorType::Rgba => bytes.as_rgba().to_owned(),
        png::ColorType::Indexed => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File ColorScheme not supported",
            ))
        }
    };
    Ok((pixels, info.width as usize, info.height as usize))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_decode_image_jpg() {
        let file_path = PathBuf::from("test/test.jpg");
        let result = decode_image(&file_path);
        println!("{result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_image_png() {
        let file_path = PathBuf::from("test/test.png");
        let result = decode_image(&file_path);
        println!("{result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_image_unsupported() {
        let file_path = PathBuf::from("test/test.bmp");
        let result = decode_image(&file_path);
        println!("{result:?}");
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_jpeg() {
        let file_path = PathBuf::from("test/test.jpg");
        let result = decode_jpeg(&file_path);
        println!("{result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_jpeg_invalid() {
        let file_path = PathBuf::from("test/invalid.jpg");
        let result = decode_jpeg(&file_path);
        println!("{result:?}");
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_png() {
        let file_path = PathBuf::from("test/test.png");
        let result = decode_png(&file_path);
        println!("{result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_png_invalid() {
        let file_path = PathBuf::from("test/invalid.png");
        let result = decode_png(&file_path);
        println!("{result:?}");
        assert!(result.is_err());
    }
}

use std::{error::Error, fs, io, panic, path};

use mozjpeg::Decompress;
use rgb::{FromSlice, RGB8};

/// Decodes an image file to a vector of RGB8 pixels, along with the image's width and height.
///
/// Result is
/// - Ok with tuple (pixels in RGB8, width, height)
/// - Err if error occurs from decode functions
/// - Err if input_format not supported
///
/// # Panics
/// This function will panic if file has no extension
///
/// TODO: Return error if file has no extension
pub fn decode_image(path: &path::PathBuf) -> Result<(Vec<RGB8>, usize, usize), Box<dyn Error>> {
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

fn decode_jpeg(path: &path::PathBuf) -> Result<(Vec<RGB8>, usize, usize), Box<dyn Error>> {
    panic::catch_unwind(|| -> Result<(Vec<RGB8>, usize, usize), Box<dyn Error>> {
        let file_content = fs::read(path)?;
        let d = Decompress::new_mem(&file_content)?;
        let mut image = d.rgb()?;
        let width = image.width();
        let height = image.height();
        let pixels = image.read_scanlines().unwrap();

        assert!(image.finish_decompress());
        Ok((pixels, width, height))
    })
    .unwrap_or(Err(Box::new(io::Error::new(
        io::ErrorKind::InvalidData,
        "Failed to read jpeg",
    ))))
}

fn decode_png(path: &path::PathBuf) -> Result<(Vec<RGB8>, usize, usize), io::Error> {
    let d = png::Decoder::new(fs::File::open(path)?);
    let mut reader = d.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    let bytes = &buf[..info.buffer_size()];
    let pixels: Vec<RGB8> = bytes.as_rgba().iter().map(|color| color.rgb()).collect();
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

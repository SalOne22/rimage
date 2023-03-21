use std::{
    error, fs,
    io::{self, BufWriter, Cursor},
    panic, path,
};

use mozjpeg::Compress;
use rgb::{ComponentBytes, FromSlice, RGB8};

/// Encodes image with pixels, width, height, quality and saves it to path
pub fn encode_image(
    path: &path::PathBuf,
    pixels: &[RGB8],
    output_format: &str,
    width: usize,
    height: usize,
    quality: f32,
) -> Result<(), io::Error> {
    let file_bytes = match output_format {
        "jpg" => encode_jpeg(pixels, width, height, quality),
        "png" => encode_png(pixels, width, height),
        _ => return Err(io::Error::new(io::ErrorKind::Other, "File not supported")),
    }
    .unwrap();

    let mut path = path.to_owned();
    path.set_extension(output_format);
    fs::write(path, file_bytes)
}

fn encode_jpeg(
    pixels: &[RGB8],
    width: usize,
    height: usize,
    quality: f32,
) -> Result<Vec<u8>, Box<dyn error::Error>> {
    panic::catch_unwind(|| -> Result<Vec<u8>, Box<dyn error::Error>> {
        let mut comp = Compress::new(mozjpeg::ColorSpace::JCS_RGB);

        comp.set_size(width, height);
        comp.set_quality(quality * 100.0);
        comp.set_color_space(mozjpeg::ColorSpace::JCS_YCbCr);
        comp.set_mem_dest();
        comp.start_compress();

        assert!(comp.write_scanlines(&pixels[..].as_bytes()));

        comp.finish_compress();
        Ok(comp.data_to_vec().unwrap())
    })
    .unwrap_or(Err(Box::new(io::Error::new(
        io::ErrorKind::InvalidData,
        "Failed to read jpeg",
    ))))
}

fn encode_png(
    pixels: &[RGB8],
    width: usize,
    height: usize,
) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut buf = Cursor::new(Vec::with_capacity(width * height * 4));
    {
        let ref mut w = BufWriter::new(&mut buf);
        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut w = encoder.write_header()?;
        w.write_image_data(pixels.as_rgba().as_bytes())?;
        w.finish()?;
    }
    let opts = oxipng::Options::from_preset(6);
    Ok(oxipng::optimize_from_memory(buf.get_ref(), &opts)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoders;
    use std::path::PathBuf;

    #[test]
    fn test_encode_jpeg() {
        let (pixels, width, height) =
            decoders::decode_image(&PathBuf::from("test/encode_test.png")).unwrap();
        let quality = 0.8;

        let result = encode_jpeg(&pixels, width, height, quality);
        assert!(result.is_ok());

        let encoded_bytes = result.unwrap();
        assert!(!encoded_bytes.is_empty());
    }

    #[test]
    fn test_encode_png() {
        let (pixels, width, height) =
            decoders::decode_image(&PathBuf::from("test/encode_test.png")).unwrap();

        let result = encode_png(&pixels, width, height);
        println!("{result:?}");
        assert!(result.is_ok());

        let encoded_bytes = result.unwrap();
        assert!(!encoded_bytes.is_empty());
    }

    #[test]
    fn test_encode_image() {
        let (pixels, width, height) =
            decoders::decode_image(&PathBuf::from("test/encode_test.png")).unwrap();
        let quality = 0.8;

        let path = PathBuf::from("test.jpg");
        let result = encode_image(&path, &pixels, "jpg", width, height, quality);
        println!("{result:?}");
        assert!(result.is_ok());
        assert!(fs::remove_file(path).is_ok());

        let path = PathBuf::from("test.png");
        let result = encode_image(&path, &pixels, "png", width, height, quality);
        println!("{result:?}");
        assert!(result.is_ok());
        assert!(fs::remove_file(path).is_ok());
    }
}

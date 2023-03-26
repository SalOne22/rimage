/*!
This crate provides a cli tool and library for image processing.
Similar to [squoosh!](https://squoosh.app/) using same codecs,
but fully written on rust and with bulk processing support.

Current features:
- Decoding jpeg and png
- Encoding with optimizations
- Get image information

# Usage

First add this crate to your dependencies:
```text
cargo add rimage
```

or add this to Cargo.toml:
```toml
[dependencies]
rimage = "0.2"
```

Next import Decoder:
```text
use rimage::Decoder;
```

After that you can use this crate:

## Decoding

```
# use rimage::Decoder;
# let path = std::path::PathBuf::from("tests/files/basi0g01.jpg");
// Build decoder from file path
let decoder = match Decoder::build(&path) {
    Ok(d) => d,
    Err(e) => {
        eprintln!("Oh no there is error! {e}");
        std::process::exit(1);
    }
};

// Decode image to image data
let image = match decoder.decode() {
    Ok(img) => img,
    Err(e) => {
        eprintln!("Oh no there is another error! {e}");
        std::process::exit(1);
    }
};

// Get image data
println!("Color Space: {:?}", image.color_space());
println!("Bit Depth: {:?}", image.bit_depth());
println!("Size: {:?}", image.size());
println!("Data length: {:?}", image.data().len());

// Do something with image...
```
*/
#![warn(missing_docs)]

use error::{ConfigError, DecodingError, EncodingError};
use std::{fs, panic, path};

use image::BitDepth;
pub use image::{ImageData, OutputFormat};

/// Decoders for images
#[deprecated(since = "0.2.0", note = "use the Decoder struct instead")]
pub mod decoders;
/// Encoders for images
#[deprecated(since = "0.2.0", note = "use the Encoder struct instead")]
pub mod encoders;

/// All errors that can occur
pub mod error;

/// Image data
pub mod image;

/// Config for encoder
#[derive(Debug)]
pub struct Config {
    output: path::PathBuf,
    quality: f32,
    output_format: OutputFormat,
}

impl Config {
    /// Builds config from parameters
    ///
    /// # Examples
    ///
    /// ```
    /// # use rimage::{Config, OutputFormat};
    /// # use std::path;
    /// let input = &[path::PathBuf::from("tests/files/basi0g01.jpg")];
    /// let quality = 100.0;
    /// let output_format = OutputFormat::MozJpeg;
    /// let config = Config::build(input, quality, output_format).unwrap();
    /// ```
    ///
    /// # Errors
    /// - [`ConfigError::QualityOutOfBounds`] if quality is not in range 0.0 - 100.0
    /// - [`ConfigError::InputIsEmpty`] if input is empty
    pub fn build(
        output: path::PathBuf,
        quality: f32,
        output_format: OutputFormat,
    ) -> Result<Self, ConfigError> {
        if quality < 0.0 || quality > 100.0 {
            return Err(ConfigError::QualityOutOfBounds);
        }

        if output.to_str().unwrap().is_empty() {
            return Err(ConfigError::InputIsEmpty);
        }

        Ok(Config {
            output,
            quality,
            output_format,
        })
    }
    #[inline]
    /// Gets input array of paths from config
    pub fn input(&self) -> &path::PathBuf {
        &self.output
    }
    #[inline]
    /// Gets quality of output images from config
    pub fn quality(&self) -> f32 {
        self.quality
    }
    #[inline]
    /// Gets format of output images from config
    pub fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            output: path::PathBuf::from(""),
            quality: 75.0,
            output_format: OutputFormat::MozJpeg,
        }
    }
}

/// Decoder used to get image data from file
pub struct Decoder<'a> {
    path: &'a path::PathBuf,
    raw_data: Vec<u8>,
}

impl<'a> Decoder<'a> {
    /// Builds decoder from path
    ///
    /// # Examples
    /// ```
    /// # use rimage::Decoder;
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// let d = Decoder::build(&path);
    /// ```
    ///
    /// # Errors
    ///
    /// - [`io::Error`] if file failed to read, find, etc.
    ///
    /// [`Decoder`]: struct.Decoder.html
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    #[inline]
    pub fn build(path: &'a path::PathBuf) -> Result<Self, DecodingError> {
        let raw_data = fs::read(path)?;

        Ok(Decoder { path, raw_data })
    }

    /// Decodes file to ImageData
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Decoder, ImageData};
    /// # use std::path;
    /// # let path = path::PathBuf::from("tests/files/basi0g01.jpg");
    /// # let d = Decoder::build(&path).unwrap();
    /// let image = d.decode();
    /// match image {
    ///   Ok(_) => println!("Image decoded"),
    ///  Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`DecodingError::Format`] if format is not supported
    /// - [`DecodingError::Parsing`] if file is not a image, unsupported color space, etc.
    ///
    /// [`DecodingError::Format`]: enum.DecodingError.html#variant.Format
    /// [`DecodingError::Parsing`]: enum.DecodingError.html#variant.Parsing
    pub fn decode(&self) -> Result<ImageData, DecodingError> {
        let extension = match self.path.extension() {
            Some(ext) => ext,
            None => return Err(DecodingError::Format("None".to_string())),
        };

        match extension.to_str() {
            Some("jpg") | Some("jpeg") => self.decode_jpeg(),
            Some("png") => self.decode_png(),
            Some(ext) => Err(DecodingError::Format(ext.to_string())),
            None => Err(DecodingError::Parsing(
                "Extension is not valid unicode".to_string(),
            )),
        }
    }

    fn decode_jpeg(&self) -> Result<ImageData, DecodingError> {
        panic::catch_unwind(|| -> Result<ImageData, DecodingError> {
            let d = mozjpeg::Decompress::new_mem(&self.raw_data)?;
            let color_space = d.color_space().into();
            let mut image = match d.image()? {
                mozjpeg::Format::RGB(img) => img,
                mozjpeg::Format::Gray(img) => img,
                mozjpeg::Format::CMYK(img) => img,
            };

            let data = image.read_scanlines_flat().ok_or(DecodingError::Parsing(
                "Cannot read jpeg scanlines".to_string(),
            ))?;

            Ok(ImageData::new(
                color_space,
                BitDepth::Eight,
                image.width() as usize,
                image.height() as usize,
                data,
            ))
        })
        .unwrap_or(Err(DecodingError::Parsing(
            "Failed to decode jpeg".to_string(),
        )))
    }

    fn decode_png(&self) -> Result<ImageData, DecodingError> {
        let d = png::Decoder::new(fs::File::open(self.path)?);
        let mut reader = d.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let data = buf[..info.buffer_size()].to_vec();
        Ok(ImageData::new(
            info.color_type.into(),
            info.bit_depth.into(),
            info.width as usize,
            info.height as usize,
            data,
        ))
    }
}

/// Encoder used to encode image data to file
pub struct Encoder<'a> {
    image_data: ImageData,
    config: &'a Config,
}

// Write Encoder encode method to encode image data to file in different formats from config
// Write Encoder build method to build encoder from Config and ImageData
// Write Encoder encode_png method to encode image data to png file
// Write Encoder encode_jpeg method to encode image data to jpeg file
impl<'a> Encoder<'a> {
    /// Builds encoder from config and image data
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Encoder, Config, ImageData, image};
    /// let config = Config::default();
    /// let image_data = ImageData::new(image::ColorSpace::Rgb, image::BitDepth::Eight, 8, 8, vec![0; 192]);
    /// let encoder = Encoder::new(&config, image_data);
    /// ```
    pub fn new(conf: &'a Config, image_data: ImageData) -> Self {
        Encoder {
            image_data,
            config: conf,
        }
    }

    /// Encodes image data to file
    ///
    /// # Examples
    /// ```
    /// # use rimage::{Encoder, Config, ImageData, image};
    /// # let config = Config::default();
    /// # let image_data = ImageData::new(image::ColorSpace::Rgb, image::BitDepth::Eight, 8, 8, vec![0; 192]);
    /// # let encoder = Encoder::new(&config, image_data);
    /// encoder.encode();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`EncodingError::Encoding`] if encoding failed
    /// - [`EncodingError::IoError`] if IO error occurred
    pub fn encode(self) -> Result<(), EncodingError> {
        match self.config.output_format {
            OutputFormat::Png => self.encode_png(),
            OutputFormat::Oxipng => self.encode_oxipng(),
            OutputFormat::MozJpeg => self.encode_mozjpeg(),
        }
    }

    fn encode_mozjpeg(self) -> Result<(), EncodingError> {
        panic::catch_unwind(|| -> Result<(), EncodingError> {
            let mut encoder = mozjpeg::Compress::new((*self.image_data.color_space()).into());
            println!("ImgData: {:?}", self.image_data);
            println!("Config: {:?}", self.config);
            println!("Data len: {}", self.image_data.data().len());

            encoder.set_size(self.image_data.size().0, self.image_data.size().1);
            encoder.set_quality(self.config.quality);
            encoder.set_progressive_mode();
            encoder.set_mem_dest();
            encoder.start_compress();
            encoder.write_scanlines(&self.image_data.data());
            encoder.finish_compress();

            let data = encoder.data_as_mut_slice().unwrap();

            fs::write(&self.config.output, data)?;

            Ok(())
        })
        .unwrap_or(Err(EncodingError::Encoding(
            "Failed to encode jpeg".to_string(),
        )))
    }

    fn encode_png(&self) -> Result<(), EncodingError> {
        let mut encoder = png::Encoder::new(
            fs::File::create(&self.config.output)?,
            self.image_data.size().0 as u32,
            self.image_data.size().1 as u32,
        );

        encoder.set_color((*self.image_data.color_space()).into());
        encoder.set_depth((*self.image_data.bit_depth()).into());
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.image_data.data())?;

        Ok(())
    }

    fn encode_oxipng(&self) -> Result<(), EncodingError> {
        let mut file = fs::File::create(&self.config.output)?;
        let mut encoder = png::Encoder::new(
            &mut file,
            self.image_data.size().0 as u32,
            self.image_data.size().1 as u32,
        );
        encoder.set_color((*self.image_data.color_space()).into());
        encoder.set_depth((*self.image_data.bit_depth()).into());
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.image_data.data())?;
        writer.finish()?;

        oxipng::optimize(
            &oxipng::InFile::from(&self.config.output),
            &oxipng::OutFile::Path(Some(self.config.output.clone())),
            &oxipng::Options::default(),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::image::ColorSpace;

    use super::*;

    #[test]
    fn decode_grayscale() {
        // let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        //     .unwrap()
        //     .map(|entry| {
        //         let entry = entry.unwrap();
        //         entry.path()
        //     })
        //     .filter(|path| {
        //         let re = Regex::new(r"^tests/files/[^x].+0g\d\d((\.png)|(\.jpg))").unwrap();
        //         re.is_match(path.to_str().unwrap_or(""))
        //     })
        //     .collect();
        let files = [path::PathBuf::from("tests/files/basi0g08.png")];

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            println!("{path:?}");

            assert_eq!(image.color_space(), &ColorSpace::Gray);
            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_grayscale_alpha() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+4a\d\d\.png").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorSpace::GrayAlpha);
            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_rgb() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^^tests/files/[^x].+2c\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorSpace::Rgb);
            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_rgba() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+6a\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorSpace::Rgba);
            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_indexed() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+3p\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            assert_eq!(image.color_space(), &ColorSpace::Indexed);
            assert_ne!(image.data().len(), 0);
            assert_ne!(image.size(), (0, 0));
        })
    }

    #[test]
    fn decode_corrupted() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/x.+\d\d\.png$").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let d = Decoder::build(path);
            assert!(d.is_ok());

            let img = d.unwrap().decode();
            assert!(img.is_err());
        })
    }

    #[test]
    fn encode_grayscale_jpeg() {
        let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                entry.path()
            })
            .filter(|path| {
                let re = Regex::new(r"^tests/files/[^x].+0g\d\d((\.png)|(\.jpg))").unwrap();
                re.is_match(path.to_str().unwrap_or(""))
            })
            .collect();

        files.iter().for_each(|path| {
            let image = Decoder::build(path).unwrap().decode().unwrap();

            let out_path = path.with_extension("out.jpg");

            let conf = Config::build(out_path.clone(), 75.0, OutputFormat::MozJpeg).unwrap();

            let encoder = Encoder::new(&conf, image);
            let result = encoder.encode();
            println!("{path:?}: {result:?}");

            assert!(result.is_ok());
            assert!(out_path.exists());
            assert!(out_path.is_file());
            assert!(out_path.metadata().unwrap().len() > 0);
            // assert!(fs::remove_file(out_path).is_ok());
        })
    }
}

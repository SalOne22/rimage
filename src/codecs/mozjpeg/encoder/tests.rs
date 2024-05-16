use std::io::Cursor;

use zune_core::colorspace::ColorSpace;

use crate::test_utils::*;

use super::*;

#[test]
fn encode_colorspaces_u8() {
    let mut results = vec![];

    let encoder = MozJpegEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u8(200, 200, *colorspace);

                let mut encoder = MozJpegEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                if result.is_err() {
                    dbg!(&result);
                }

                assert!(result.is_ok());
            })
            .unwrap();

        results.push(handler.join())
    }

    results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
}

#[test]
fn encode_colorspaces_u16() {
    let mut results = vec![];

    let encoder = MozJpegEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u16(200, 200, *colorspace);

                let mut encoder = MozJpegEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                if result.is_err() {
                    dbg!(&result);
                }

                assert!(result.is_ok());
            })
            .unwrap();

        results.push(handler.join())
    }

    results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
}

#[test]
fn encode_colorspaces_f32() {
    let mut results = vec![];

    let encoder = MozJpegEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_f32(200, 200, *colorspace);

                let mut encoder = MozJpegEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                if result.is_err() {
                    dbg!(&result);
                }

                assert!(result.is_ok());
            })
            .unwrap();

        results.push(handler.join())
    }

    results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
}

#[test]
fn encode_u8() {
    let image = create_test_image_u8(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_u16() {
    let image = create_test_image_u16(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_f32() {
    let image = create_test_image_f32(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_animated() {
    let image = create_test_image_animated(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

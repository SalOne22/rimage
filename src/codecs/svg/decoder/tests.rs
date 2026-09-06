use std::fs::File;

use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

use super::{SvgDecoder, SvgOptions};

#[test]
fn decode_simple_rect() {
    let file = File::open("tests/files/svg/rect.svg").unwrap();

    let decoder = SvgDecoder::try_new(file).unwrap();
    let img = Image::from_decoder(decoder).unwrap();

    assert_eq!(img.dimensions(), (100, 50));
    assert_eq!(img.colorspace(), ColorSpace::RGBA);

    let red = img.channels_ref(false)[0].reinterpret_as::<u8>().unwrap();
    assert_eq!(red[0], 255);

    let green = img.channels_ref(false)[1].reinterpret_as::<u8>().unwrap();
    assert_eq!(green[0], 0);

    let alpha = img.channels_ref(false)[3].reinterpret_as::<u8>().unwrap();
    assert_eq!(alpha[0], 255);
}

#[test]
fn probe_size_returns_intrinsic_size() {
    let file = File::open("tests/files/svg/rect.svg").unwrap();

    let size = SvgDecoder::probe_size(file, None).unwrap();

    assert_eq!(size, (100.0, 50.0));
}

#[test]
fn decode_through_resize_callback() {
    let file = File::open("tests/files/svg/rect.svg").unwrap();

    let decoder = SvgDecoder::try_new_with_resize(file, None, |size| {
        assert_eq!(size, (100, 50));
        Ok(Some((200, 100)))
    })
    .unwrap();
    let img = Image::from_decoder(decoder).unwrap();

    assert_eq!(img.dimensions(), (200, 100));
}

#[test]
fn decode_with_target_size_renders_at_vector_quality() {
    let file = File::open("tests/files/svg/rect.svg").unwrap();
    let options = SvgOptions {
        target_size: Some((200, 100)),
        ..Default::default()
    };

    let decoder = SvgDecoder::try_new_with_options(file, options).unwrap();
    let img = Image::from_decoder(decoder).unwrap();

    assert_eq!(img.dimensions(), (200, 100));

    // The target-size render keeps full opacity and crisp vector edges, just
    // like the old --svg-scale path did.
    let red = img.channels_ref(false)[0].reinterpret_as::<u8>().unwrap();
    assert_eq!(red[0], 255);
}

#[test]
fn decode_with_target_aspect_ratio() {
    let file = File::open("tests/files/svg/rect.svg").unwrap();
    let options = SvgOptions {
        target_size: Some((50, 25)),
        ..Default::default()
    };

    let decoder = SvgDecoder::try_new_with_options(file, options).unwrap();
    let img = Image::from_decoder(decoder).unwrap();

    assert_eq!(img.dimensions(), (50, 25));
}

#[test]
fn decode_with_exact_target_size() {
    let file = File::open("tests/files/svg/rect.svg").unwrap();
    let options = SvgOptions {
        target_size: Some((120, 40)),
        ..Default::default()
    };

    let decoder = SvgDecoder::try_new_with_options(file, options).unwrap();
    let img = Image::from_decoder(decoder).unwrap();

    assert_eq!(img.dimensions(), (120, 40));
}

#[test]
fn decode_without_viewbox_uses_content_bbox() {
    let file = File::open("tests/files/svg/no-viewbox.svg").unwrap();

    let decoder = SvgDecoder::try_new(file).unwrap();
    let img = Image::from_decoder(decoder).unwrap();

    // The circle spans x 80..160 and y 40..120.
    assert_eq!(img.dimensions(), (160, 120));
}

#[test]
fn decode_text_renders_without_error() {
    let file = File::open("tests/files/svg/text-cjk.svg").unwrap();

    let decoder = SvgDecoder::try_new(file).unwrap();
    let img = Image::from_decoder(decoder).unwrap();

    // the canvas comes from the SVG's width/height (200x100), the text
    // mixes simplified and traditional Han with kana and latin
    assert_eq!(img.dimensions(), (200, 100));
}

#[test]
fn decode_invalid_svg_errors() {
    let file = File::open("tests/files/svg/invalid.svg").unwrap();

    // Parsing happens eagerly when creating the decoder.
    let decoder = SvgDecoder::try_new(file);

    assert!(decoder.is_err());
}

#[test]
fn decode_with_zero_target_size_errors() {
    let file = File::open("tests/files/svg/rect.svg").unwrap();
    let options = SvgOptions {
        target_size: Some((0, 100)),
        ..Default::default()
    };

    let decoder = SvgDecoder::try_new_with_options(file, options);

    assert!(decoder.is_err());
}

#[test]
fn decode_with_huge_intrinsic_size_errors() {
    let file = File::open("tests/files/svg/huge-canvas.svg").unwrap();

    // Parsing succeeds, the oversized render target is rejected instead of
    // attempting a multi-gigabyte allocation.
    let decoder = SvgDecoder::try_new(file);

    assert!(decoder.is_err());
}

#[test]
fn decode_with_oversized_target_size_errors() {
    let file = File::open("tests/files/svg/rect.svg").unwrap();
    let options = SvgOptions {
        target_size: Some((u32::MAX, u32::MAX)),
        ..Default::default()
    };

    let decoder = SvgDecoder::try_new_with_options(file, options);

    assert!(decoder.is_err());
}

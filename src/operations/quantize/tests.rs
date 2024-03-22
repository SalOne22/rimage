use crate::test_utils::*;

use super::*;

#[test]
fn quantize_u8() {
    let quantize = Quantize::new(75, None);
    let mut image = create_test_image_u8(200, 200, ColorSpace::RGBA);

    let result = quantize.execute(&mut image);

    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn dither_u8() {
    let quantize = Quantize::new(75, Some(0.75));
    let mut image = create_test_image_u8(200, 200, ColorSpace::RGBA);

    let result = quantize.execute(&mut image);

    dbg!(&result);

    assert!(result.is_ok());
}

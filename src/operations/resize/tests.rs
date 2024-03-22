use zune_core::colorspace::ColorSpace;

use crate::test_utils::*;

use super::*;

#[test]
fn resize_u8() {
    let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
    let mut image = create_test_image_u8(200, 200, ColorSpace::RGB);

    let result = resize.execute(&mut image);
    dbg!(&result);

    assert!(result.is_ok());
    assert_eq!(image.dimensions(), (100, 100));
}

#[test]
fn resize_u16() {
    let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
    let mut image = create_test_image_u16(200, 200, ColorSpace::RGB);

    let result = resize.execute(&mut image);
    dbg!(&result);

    assert!(result.is_ok());
    assert_eq!(image.dimensions(), (100, 100));
}

#[test]
fn resize_f32() {
    let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
    let mut image = create_test_image_f32(200, 200, ColorSpace::RGB);

    let result = resize.execute(&mut image);
    dbg!(&result);

    assert!(result.is_ok());
    assert_eq!(image.dimensions(), (100, 100));
}

#[test]
fn resize_animated() {
    let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
    let mut image = create_test_image_animated(200, 200, ColorSpace::RGB);

    let result = resize.execute(&mut image);
    dbg!(&result);

    assert!(result.is_ok());
    assert_eq!(image.dimensions(), (100, 100));
}

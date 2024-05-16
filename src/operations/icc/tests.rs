use crate::test_utils::*;

use super::*;

#[test]
fn apply_icc_profile() {
    let mut image = create_test_image_u8(100, 100, ColorSpace::RGB);

    let src_profile = Profile::new_srgb().icc().unwrap();
    image.metadata_mut().set_icc_chunk(src_profile);

    let target_profile = Profile::new_file("tests/files/icc/tinysrgb.icc").unwrap();

    let icc = target_profile.icc().unwrap();

    // Apply ICC profile
    let apply_icc = ApplyICC::new(target_profile);
    apply_icc.execute_impl(&mut image).unwrap();

    // Assert ICC profile is set correctly
    assert_eq!(*image.metadata().icc_chunk().unwrap(), icc);
}

#[test]
fn skip_icc_profile() {
    let mut image = create_test_image_u8(100, 100, ColorSpace::RGB);

    let apply_icc = ApplySRGB;
    let result = apply_icc.execute_impl(&mut image);

    assert!(result.is_ok());
    assert_eq!(image.metadata().icc_chunk(), None);
}

#[test]
fn apply_srgb_profile() {
    let mut image = create_test_image_u8(100, 100, ColorSpace::RGB);

    let src_profile = Profile::new_file("tests/files/icc/tinysrgb.icc")
        .unwrap()
        .icc()
        .unwrap();
    image.metadata_mut().set_icc_chunk(src_profile);

    let icc = Profile::new_srgb().icc().unwrap();

    // Apply ICC profile
    let apply_icc = ApplySRGB;
    apply_icc.execute_impl(&mut image).unwrap();

    // Assert ICC profile is set correctly
    assert_eq!(*image.metadata().icc_chunk().unwrap(), icc);
}

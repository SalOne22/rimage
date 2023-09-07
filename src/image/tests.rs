use super::*;

#[test]
fn new_image() {
    let pixel_data: Vec<RGBA8> = vec![RGBA8::new(0, 0, 0, 0); 100 * 50];
    let image = Image::new(pixel_data.clone(), 100, 50);

    assert_eq!(image.data(), &pixel_data);
    assert_eq!(image.width(), 100);
    assert_eq!(image.height(), 50);
}

#[test]
fn resize_image_smaller() {
    let image_data: Vec<RGBA8> = vec![RGBA8::new(0, 0, 0, 0); 100 * 50];
    let mut image = Image::new(image_data, 100, 50);

    let resize_config = ResizeConfig::default().with_width(50);
    image.resize(&resize_config).unwrap();

    assert_eq!(image.data(), &[RGBA8::new(0, 0, 0, 0); 50 * 25]);
    assert_eq!(image.width(), 50);
    assert_eq!(image.height(), 25);
}

#[test]
fn resize_image_bigger() {
    let image_data: Vec<RGBA8> = vec![RGBA8::new(0, 0, 0, 0); 100 * 50];
    let mut image = Image::new(image_data, 100, 50);

    let resize_config = ResizeConfig::default().with_width(200);
    image.resize(&resize_config).unwrap();

    assert_eq!(image.data(), &vec![RGBA8::new(0, 0, 0, 0); 200 * 100]);
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 100);
}

#[test]
fn resize_image_width_and_height() {
    let image_data: Vec<RGBA8> = vec![RGBA8::new(0, 0, 0, 0); 100 * 50];
    let mut image = Image::new(image_data, 100, 50);

    let resize_config = ResizeConfig::default().with_width(200).with_height(150);
    image.resize(&resize_config).unwrap();

    assert_eq!(image.data(), &[RGBA8::new(0, 0, 0, 0); 200 * 150]);
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 150);
}

#[test]
fn quantize_image() {
    let image_data: Vec<RGBA8> = vec![RGBA8::new(0, 0, 0, 0); 800 * 600];
    let mut image = Image::new(image_data.clone(), 800, 600);

    let quantization_config = QuantizationConfig::default();
    assert!(image.quantize(&quantization_config).is_ok());

    // Test quantization with quality
    let quantization_config = QuantizationConfig::default().with_quality(50).unwrap();
    assert!(image.quantize(&quantization_config).is_ok());
}

#[test]
fn flip_diagonally() {
    #[rustfmt::skip]
    let image_data = vec![
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
    ];

    #[rustfmt::skip]
    let test_image_data = vec![
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
    ];

    let mut image = Image::new(image_data.clone(), 5, 3);

    image.flip_diagonally();

    assert_ne!(image.data, image_data);

    assert_eq!(image.width, 3);
    assert_eq!(image.height, 5);

    assert_eq!(image.data, test_image_data);
}

#[test]
fn flip_horizontally() {
    #[rustfmt::skip]
    let image_data = vec![
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255),
    ];

    #[rustfmt::skip]
    let test_image_data = vec![
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
    ];

    let mut image = Image::new(image_data.clone(), 3, 5);

    image.flip_horizontally();

    assert_ne!(image.data, image_data);

    assert_eq!(image.width, 3);
    assert_eq!(image.height, 5);

    assert_eq!(image.data, test_image_data);
}

#[test]
fn rotate_180() {
    #[rustfmt::skip]
    let image_data = vec![
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
    ];

    #[rustfmt::skip]
    let test_image_data = vec![
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
        RGBA8::new(0, 0, 0, 255), RGBA8::new(0, 0, 0, 0), RGBA8::new(0, 0, 0, 0),
    ];

    let mut image = Image::new(image_data.clone(), 3, 5);

    image.rotate_180();

    assert_ne!(image.data, image_data);

    assert_eq!(image.width, 3);
    assert_eq!(image.height, 5);

    assert_eq!(image.data, test_image_data);
}

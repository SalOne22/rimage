use std::fs::File;

use super::*;

#[test]
fn decode() {
    let file_content = File::open("tests/files/webp/f1t.webp").unwrap();

    let decoder = WebPDecoder::try_new(file_content).unwrap();

    let img = Image::from_decoder(decoder).unwrap();

    assert_eq!(img.dimensions(), (48, 80));
    assert_eq!(img.colorspace(), ColorSpace::RGBA);
}

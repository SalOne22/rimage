use std::fs::read;

use zune_core::bytestream::ZByteReader;

use super::*;

#[test]
fn decode() {
    let file_content = read("tests/files/webp/f1t.webp").unwrap();

    let reader = ZByteReader::new(file_content);

    let mut decoder = WebPDecoder::try_new(reader).unwrap();

    let img =
        <WebPDecoder<ZByteReader<Vec<u8>>> as DecoderTrait<Vec<u8>>>::decode(&mut decoder).unwrap();

    assert_eq!(img.dimensions(), (48, 80));
    assert_eq!(img.colorspace(), ColorSpace::RGBA);
}

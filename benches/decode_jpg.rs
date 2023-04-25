use std::{
    fs,
    path::{Path, PathBuf},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[allow(deprecated)]
use rimage::decoders::decode_image;
use rimage::Decoder;

#[allow(deprecated)]
fn bench_decode_jpg(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_jpg");
    group.bench_function("decoders", |b| {
        b.iter(|| decode_image(black_box(&PathBuf::from("tests/files/basi6a08.jpg"))))
    });
    group.bench_function("Decoder", |b| {
        b.iter(|| {
            let file = fs::File::open(&Path::new("tests/files/basi6a08.jpg")).unwrap();
            Decoder::new(black_box(&Path::new("tests/files/basi6a08.jpg")), file).decode()
        })
    });
    group.finish();
}

criterion_group!(benches, bench_decode_jpg);
criterion_main!(benches);

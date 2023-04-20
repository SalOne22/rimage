use std::{
    fs,
    path::{Path, PathBuf},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[allow(deprecated)]
use rimage::decoders::decode_image;
use rimage::Decoder;

#[allow(deprecated)]
fn bench_decode_png(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_png");
    group.bench_function("decoders", |b| {
        b.iter(|| decode_image(black_box(&PathBuf::from("tests/files/basi6a08.png"))))
    });
    group.bench_function("Decoder", |b| {
        b.iter(|| {
            let file = fs::File::open(&Path::new("tests/files/basi6a08.png")).unwrap();
            Decoder::new(black_box(&Path::new("tests/files/basi6a08.png")), file).decode()
        })
    });
    group.finish();
}

criterion_group!(benches, bench_decode_png);
criterion_main!(benches);

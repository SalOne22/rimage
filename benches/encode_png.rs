use std::{fs, path::PathBuf};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[allow(deprecated)]
use rimage::{decoders::decode_image, encoders::encode_image};

#[allow(deprecated)]
fn bench_encode_png(c: &mut Criterion) {
    let (pixels, width, height) = decode_image(&PathBuf::from("tests/files/basi6a08.png")).unwrap();
    c.bench_function("en png", |b| {
        b.iter(|| {
            encode_image(
                black_box(&PathBuf::from("en")),
                black_box(&pixels),
                black_box("png"),
                black_box(width),
                black_box(height),
                black_box(0.75),
            )
        })
    });
    fs::remove_file("en.png").unwrap();
}

criterion_group!(benches, bench_encode_png);
criterion_main!(benches);

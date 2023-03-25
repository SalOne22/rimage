use std::{path::PathBuf, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rimage::decoders::decode_image;

fn bench_decode_png_1(c: &mut Criterion) {
    c.bench_function("di lt1png", |b| {
        b.iter(|| decode_image(black_box(&PathBuf::from("test/large_test1.png"))))
    });
}

fn bench_decode_png_2(c: &mut Criterion) {
    c.bench_function("di lt2png", |b| {
        b.iter(|| decode_image(black_box(&PathBuf::from("test/large_test2.png"))))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(20).measurement_time(Duration::from_secs(25));
    targets = bench_decode_png_1, bench_decode_png_2
);
criterion_main!(benches);
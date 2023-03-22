use std::{fs, path::PathBuf, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rimage::{decoders::decode_image, encoders::encode_image};

fn bench_encode_jpg_1(c: &mut Criterion) {
    let (pixels, width, height) = decode_image(&PathBuf::from("test/large_test1.jpg")).unwrap();
    c.bench_function("en lt1jpg", |b| {
        b.iter(|| {
            encode_image(
                black_box(&PathBuf::from("en_lt1")),
                black_box(&pixels),
                black_box("jpg"),
                black_box(width),
                black_box(height),
                black_box(0.75),
            )
        })
    });
    fs::remove_file("en_lt1.jpg").unwrap();
}

fn bench_encode_jpg_2(c: &mut Criterion) {
    let (pixels, width, height) = decode_image(&PathBuf::from("test/large_test2.jpg")).unwrap();
    c.bench_function("en lt2jpg", |b| {
        b.iter(|| {
            encode_image(
                black_box(&PathBuf::from("en_lt2")),
                black_box(&pixels),
                black_box("jpg"),
                black_box(width),
                black_box(height),
                black_box(0.75),
            )
        })
    });
    fs::remove_file("en_lt2.jpg").unwrap();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(30));
    targets = bench_encode_jpg_1, bench_encode_jpg_2
);
criterion_main!(benches);

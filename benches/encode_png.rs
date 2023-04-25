use std::{
    fs,
    path::{Path, PathBuf},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rimage::Decoder;
#[allow(deprecated)]
use rimage::{decoders::decode_image, encoders::encode_image};

#[allow(deprecated)]
fn bench_encode_png(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_png");

    let (pixels, width, height) = decode_image(&PathBuf::from("tests/files/basi6a08.png")).unwrap();

    let file = fs::File::open(&Path::new("tests/files/basi6a08.png")).unwrap();
    let image = Decoder::new(black_box(&Path::new("tests/files/basi6a08.png")), file)
        .decode()
        .unwrap();

    group.bench_function("encoders", |b| {
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
    group.bench_function("Encoder", |b| {
        b.iter(|| {
            let data = rimage::Encoder::new(
                black_box(
                    &rimage::Config::build(75.0, rimage::OutputFormat::Oxipng, None, None, None)
                        .unwrap(),
                ),
                black_box(image.clone()),
            )
            .encode()
            .unwrap();
            fs::write("en_u.png", data).unwrap();
        })
    });
    fs::remove_file("en.png").unwrap();
    fs::remove_file("en_u.png").unwrap();
}

criterion_group!(benches, bench_encode_png);
criterion_main!(benches);

use std::{fs, path::Path};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[allow(deprecated)]
use rimage::{image, Decoder};

#[allow(deprecated)]
fn bench_encode_png(c: &mut Criterion) {
    let image = Decoder::from_path(black_box(Path::new("tests/files/basi6a08.png")))
        .unwrap()
        .decode()
        .unwrap();

    c.bench_function("Encoder", |b| {
        b.iter(|| {
            let data = rimage::Encoder::new(
                black_box(
                    &rimage::Config::builder(image::Codec::Oxipng)
                        .build()
                        .unwrap(),
                ),
                black_box(image.clone()),
            )
            .encode()
            .unwrap();
            fs::write("en.png", data).unwrap();
        })
    });

    fs::remove_file("en.png").unwrap();
}

criterion_group!(benches, bench_encode_png);
criterion_main!(benches);

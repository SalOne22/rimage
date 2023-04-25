use std::{fs, path::Path};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rimage::Decoder;

fn bench_encode_webp(c: &mut Criterion) {
    let file = fs::File::open(&Path::new("tests/files/basi6a08.webp")).unwrap();
    let image = Decoder::new(black_box(&Path::new("tests/files/basi6a08.webp")), file)
        .decode()
        .unwrap();

    c.bench_function("encode_webp", |b| {
        b.iter(|| {
            let data = rimage::Encoder::new(
                black_box(
                    &rimage::Config::build(75.0, rimage::OutputFormat::WebP, None, None, None)
                        .unwrap(),
                ),
                black_box(image.clone()),
            )
            .encode()
            .unwrap();
            fs::write("en.webp", data).unwrap();
        })
    });
    fs::remove_file("en.webp").unwrap();
}

criterion_group!(benches, bench_encode_webp);
criterion_main!(benches);

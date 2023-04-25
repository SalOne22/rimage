use std::{fs, path::Path};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rimage::Decoder;

fn bench_decode_webp(c: &mut Criterion) {
    c.bench_function("decode_webp", |b| {
        b.iter(|| {
            let file = fs::File::open(&Path::new("tests/files/basi6a08.webp")).unwrap();
            Decoder::new(black_box(&Path::new("tests/files/basi6a08.webp")), file).decode()
        })
    });
}

criterion_group!(benches, bench_decode_webp);
criterion_main!(benches);

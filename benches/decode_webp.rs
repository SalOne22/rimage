use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rimage::Decoder;

fn bench_decode_webp(c: &mut Criterion) {
    c.bench_function("decode_webp", |b| {
        b.iter(|| {
            Decoder::from_path(black_box(Path::new("tests/files/basi6a08.webp")))
                .unwrap()
                .decode()
        })
    });
}

criterion_group!(benches, bench_decode_webp);
criterion_main!(benches);

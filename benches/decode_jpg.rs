use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[allow(deprecated)]
use rimage::Decoder;

#[allow(deprecated)]
fn bench_decode_jpg(c: &mut Criterion) {
    c.bench_function("Decoder", |b| {
        b.iter(|| {
            Decoder::from_path(black_box(Path::new("tests/files/basi6a08.jpg")))
                .unwrap()
                .decode()
        })
    });
}

criterion_group!(benches, bench_decode_jpg);
criterion_main!(benches);

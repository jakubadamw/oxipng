use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxipng::{internal_tests::*, *};

fn strategies_benchmarks(c: &mut Criterion) {
    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("filters_minsum", |b| {
        b.iter(|| png.raw.filter_image(RowFilter::MinSum, false))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("filters_entropy", |b| {
        b.iter(|| png.raw.filter_image(RowFilter::Entropy, false))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("filters_bigrams", |b| {
        b.iter(|| png.raw.filter_image(RowFilter::Bigrams, false))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("filters_bigent", |b| {
        b.iter(|| png.raw.filter_image(RowFilter::BigEnt, false))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("filters_brute", |b| {
        b.iter(|| png.raw.filter_image(RowFilter::Brute, false))
    });
}

criterion_group!(benches, strategies_benchmarks);
criterion_main!(benches);

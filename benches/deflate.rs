use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxipng::{internal_tests::*, *};

fn deflate_benchmarks(c: &mut Criterion) {
    let input = black_box(PathBuf::from("tests/files/rgb_16_should_be_rgb_16.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("deflate_16_bits", |b| {
        b.iter(|| deflate(png.raw.data.as_ref(), 12, None))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("deflate_8_bits", |b| {
        b.iter(|| deflate(png.raw.data.as_ref(), 12, None))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_4_should_be_palette_4.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("deflate_4_bits", |b| {
        b.iter(|| deflate(png.raw.data.as_ref(), 12, None))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_2_should_be_palette_2.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("deflate_2_bits", |b| {
        b.iter(|| deflate(png.raw.data.as_ref(), 12, None))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_1_should_be_palette_1.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("deflate_1_bits", |b| {
        b.iter(|| deflate(png.raw.data.as_ref(), 12, None))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_16_should_be_rgb_16.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("inflate_generic", |b| {
        b.iter(|| inflate(png.idat_data.as_ref(), png.raw.ihdr.raw_data_size()))
    });
}

criterion_group!(benches, deflate_benchmarks);
criterion_main!(benches);

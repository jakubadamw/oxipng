use std::{num::NonZeroU8, path::PathBuf};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxipng::{internal_tests::*, *};

// SAFETY: trivially safe. Stopgap solution until const unwrap is stabilized.
const DEFAULT_ZOPFLI_ITERATIONS: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(15) };

fn zopfli_benchmarks(c: &mut Criterion) {
    let input = black_box(PathBuf::from("tests/files/rgb_16_should_be_rgb_16.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("zopfli_16_bits_strategy_0", |b| {
        b.iter(|| {
            zopfli_deflate(png.raw.data.as_ref(), DEFAULT_ZOPFLI_ITERATIONS).ok();
        })
    });

    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("zopfli_8_bits_strategy_0", |b| {
        b.iter(|| {
            zopfli_deflate(png.raw.data.as_ref(), DEFAULT_ZOPFLI_ITERATIONS).ok();
        })
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_4_should_be_palette_4.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("zopfli_4_bits_strategy_0", |b| {
        b.iter(|| {
            zopfli_deflate(png.raw.data.as_ref(), DEFAULT_ZOPFLI_ITERATIONS).ok();
        })
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_2_should_be_palette_2.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("zopfli_2_bits_strategy_0", |b| {
        b.iter(|| {
            zopfli_deflate(png.raw.data.as_ref(), DEFAULT_ZOPFLI_ITERATIONS).ok();
        })
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_1_should_be_palette_1.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("zopfli_1_bits_strategy_0", |b| {
        b.iter(|| {
            zopfli_deflate(png.raw.data.as_ref(), DEFAULT_ZOPFLI_ITERATIONS).ok();
        })
    });
}

criterion_group!(benches, zopfli_benchmarks);
criterion_main!(benches);

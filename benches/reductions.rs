use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxipng::{internal_tests::*, *};

fn reductions_benchmarks(c: &mut Criterion) {
    let input = black_box(PathBuf::from("tests/files/rgb_16_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_16_to_8_bits", |b| {
        b.iter(|| bit_depth::reduced_bit_depth_16_to_8(&png.raw, false))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_16_should_be_rgb_16.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_16_to_8_bits_scaled", |b| {
        b.iter(|| bit_depth::reduced_bit_depth_16_to_8(&png.raw, true))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_8_should_be_palette_4.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_8_to_4_bits", |b| {
        b.iter(|| bit_depth::reduced_bit_depth_8_or_less(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_8_should_be_palette_2.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_8_to_2_bits", |b| {
        b.iter(|| bit_depth::reduced_bit_depth_8_or_less(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_8_should_be_palette_1.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_8_to_1_bits", |b| {
        b.iter(|| bit_depth::reduced_bit_depth_8_or_less(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/grayscale_8_should_be_grayscale_4.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_grayscale_8_to_4_bits", |b| {
        b.iter(|| bit_depth::reduced_bit_depth_8_or_less(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/grayscale_8_should_be_grayscale_2.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_grayscale_8_to_2_bits", |b| {
        b.iter(|| bit_depth::reduced_bit_depth_8_or_less(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/grayscale_8_should_be_grayscale_1.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_grayscale_8_to_1_bits", |b| {
        b.iter(|| bit_depth::reduced_bit_depth_8_or_less(&png.raw))
    });

    let input = black_box(PathBuf::from("tests/files/rgba_16_should_be_rgb_16.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgba_to_rgb_16", |b| {
        b.iter(|| alpha::reduced_alpha_channel(&png.raw, true))
    });

    let input = black_box(PathBuf::from("tests/files/rgba_8_should_be_rgb_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgba_to_rgb_8", |b| {
        b.iter(|| alpha::reduced_alpha_channel(&png.raw, true))
    });

    let input = black_box(PathBuf::from("tests/files/rgba_8_should_be_rgb_trns_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgba_to_rgb_trns_8", |b| {
        b.iter(|| alpha::reduced_alpha_channel(&png.raw, true))
    });

    let input = black_box(PathBuf::from(
        "tests/files/rgba_16_should_be_grayscale_alpha_16.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgba_to_grayscale_alpha_16", |b| {
        b.iter(|| color::reduced_rgb_to_grayscale(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/rgba_8_should_be_grayscale_alpha_8.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgba_to_grayscale_alpha_8", |b| {
        b.iter(|| color::reduced_rgb_to_grayscale(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/rgb_16_should_be_grayscale_16.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgb_to_grayscale_16", |b| {
        b.iter(|| color::reduced_rgb_to_grayscale(&png.raw))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_grayscale_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgb_to_grayscale_8", |b| {
        b.iter(|| color::reduced_rgb_to_grayscale(&png.raw))
    });

    let input = black_box(PathBuf::from("tests/files/rgba_8_should_be_palette_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgba_to_palette_8", |b| {
        b.iter(|| color::reduced_to_indexed(&png.raw, true))
    });

    let input = black_box(PathBuf::from("tests/files/rgb_8_should_be_palette_8.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_rgb_to_palette_8", |b| {
        b.iter(|| color::reduced_to_indexed(&png.raw, true))
    });

    let input = black_box(PathBuf::from(
        "tests/files/grayscale_alpha_16_should_be_grayscale_16.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_grayscale_alpha_to_grayscale_16", |b| {
        b.iter(|| alpha::reduced_alpha_channel(&png.raw, true))
    });

    let input = black_box(PathBuf::from(
        "tests/files/grayscale_alpha_8_should_be_grayscale_8.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_grayscale_alpha_to_grayscale_8", |b| {
        b.iter(|| alpha::reduced_alpha_channel(&png.raw, true))
    });

    let input = black_box(PathBuf::from(
        "tests/files/grayscale_alpha_8_should_be_grayscale_trns_8.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_grayscale_alpha_to_grayscale_trns_8", |b| {
        b.iter(|| alpha::reduced_alpha_channel(&png.raw, true))
    });

    let input = black_box(PathBuf::from(
        "tests/files/grayscale_8_should_be_palette_8.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_grayscale_8_to_palette_8", |b| {
        b.iter(|| color::reduced_to_indexed(&png.raw, true))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_8_should_be_grayscale_8.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_palette_8_to_grayscale_8", |b| {
        b.iter(|| color::indexed_to_channels(&png.raw, true, false))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_should_be_reduced_with_dupes.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_palette_duplicate_reduction", |b| {
        b.iter(|| palette::reduced_palette(&png.raw, false))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_should_be_reduced_with_unused.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_palette_unused_reduction", |b| {
        b.iter(|| palette::reduced_palette(&png.raw, false))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_should_be_reduced_with_both.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_palette_full_reduction", |b| {
        b.iter(|| palette::reduced_palette(&png.raw, false))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_8_should_be_palette_8.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_palette_sort", |b| {
        b.iter(|| palette::sorted_palette(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_8_should_be_palette_8.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_palette_sort_mzeng", |b| {
        b.iter(|| palette::sorted_palette_mzeng(&png.raw))
    });

    let input = black_box(PathBuf::from(
        "tests/files/palette_8_should_be_palette_8.png",
    ));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_palette_sort_battiato", |b| {
        b.iter(|| palette::sorted_palette_battiato(&png.raw))
    });

    let input = black_box(PathBuf::from("tests/files/rgba_8_reduce_alpha.png"));
    let png = PngData::new(&input, &Options::default()).unwrap();
    c.bench_function("reductions_alpha", |b| {
        b.iter(|| alpha::cleaned_alpha_channel(&png.raw))
    });
}

criterion_group!(benches, reductions_benchmarks);
criterion_main!(benches);

use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxipng::{internal_tests::*, *};

fn interlacing_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("interlacing");

    let encode = [
        ("16_bits", "tests/files/rgb_16_should_be_rgb_16.png"),
        ("8_bits", "tests/files/rgb_8_should_be_rgb_8.png"),
        ("4_bits", "tests/files/palette_4_should_be_palette_4.png"),
        ("2_bits", "tests/files/palette_2_should_be_palette_2.png"),
        ("1_bits", "tests/files/palette_1_should_be_palette_1.png"),
    ];

    for (label, path) in encode {
        let input = black_box(PathBuf::from(path));
        let png = PngData::new(&input, &Options::default()).unwrap();
        group.bench_function(format!("interlacing_{}", label), move |b| {
            b.iter(|| png.raw.change_interlacing(Interlacing::Adam7))
        });
    }

    let decode = [
        (
            "16_bits",
            "tests/files/interlaced_rgb_16_should_be_rgb_16.png",
        ),
        ("8_bits", "tests/files/interlaced_rgb_8_should_be_rgb_8.png"),
        (
            "4_bits",
            "tests/files/interlaced_palette_4_should_be_palette_4.png",
        ),
        (
            "2_bits",
            "tests/files/interlaced_palette_2_should_be_palette_2.png",
        ),
        (
            "1_bits",
            "tests/files/interlaced_palette_1_should_be_palette_1.png",
        ),
    ];

    for (label, path) in decode {
        let input = black_box(PathBuf::from(path));
        let png = PngData::new(&input, &Options::default()).unwrap();
        group.bench_function(format!("deinterlacing_{}", label), move |b| {
            b.iter(|| png.raw.change_interlacing(Interlacing::None))
        });
    }

    group.finish();
}

criterion_group!(benches, interlacing_benchmarks);
criterion_main!(benches);

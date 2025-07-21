use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxipng::{internal_tests::*, *};

fn filters_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("filters");

    let cases = [
        ("16_bits", "tests/files/rgb_16_should_be_rgb_16.png"),
        ("8_bits", "tests/files/rgb_8_should_be_rgb_8.png"),
        ("4_bits", "tests/files/palette_4_should_be_palette_4.png"),
        ("2_bits", "tests/files/palette_2_should_be_palette_2.png"),
        ("1_bits", "tests/files/palette_1_should_be_palette_1.png"),
    ];
    let filters = [
        ("filter_0", RowFilter::None),
        ("filter_1", RowFilter::Sub),
        ("filter_2", RowFilter::Up),
        ("filter_3", RowFilter::Average),
        ("filter_4", RowFilter::Paeth),
        ("filter_5", RowFilter::MinSum),
    ];

    for (bit_label, path) in cases {
        for (filter_label, filter) in filters {
            let name = format!("filters_{}_{}", bit_label, filter_label);
            let input = black_box(PathBuf::from(path));
            let png = PngData::new(&input, &Options::default()).unwrap();
            group.bench_function(name, move |b| {
                b.iter(|| png.raw.filter_image(filter, false))
            });
        }
    }

    group.finish();
}

criterion_group!(benches, filters_benchmarks);
criterion_main!(benches);

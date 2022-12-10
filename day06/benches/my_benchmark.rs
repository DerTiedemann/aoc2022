use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("human", |b| {
        b.iter(|| {
            day06::find_first_n_distict_char_offset(
                black_box(include_str!("../input.txt")),
                black_box(4),
            );
            day06::find_first_n_distict_char_offset(
                black_box(include_str!("../input.txt")),
                black_box(14),
            );
        })
    });

    c.bench_function("machine", |b| {
        b.iter(|| {
            day06::find_marker(black_box(include_str!("../input.txt")), black_box(4));
            day06::find_marker(black_box(include_str!("../input.txt")), black_box(14));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

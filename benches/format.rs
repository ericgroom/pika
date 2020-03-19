use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pika::formatter::{format, FormatOption};

fn sponge_call(text: &str) -> String {
    format(text, FormatOption::Sponge)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sponge short", |b| {
        b.iter(|| sponge_call(black_box("A short test")))
    });
}

criterion_group!(sponge, criterion_benchmark);
criterion_main!(sponge);

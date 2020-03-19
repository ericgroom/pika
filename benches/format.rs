use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pika::formatter::{format, FormatOption};

#[inline]
fn sponge_call(text: &str) -> String {
    format(text, FormatOption::Sponge)
}

#[inline]
fn usa_call(text: &str) -> String {
    format(text, FormatOption::Usa)
}

fn short_benchmark(c: &mut Criterion) {
    c.bench_function("sponge short", |b| {
        b.iter(|| sponge_call(black_box("A short test")))
    });
    c.bench_function("usa short", |b| {
        b.iter(|| usa_call(black_box("A short test")))
    });
}

fn lorem_benchmark(c: &mut Criterion) {
    c.bench_function("sponge lorem", |b| {
        b.iter(|| sponge_call(black_box(include_str!("./lorem.txt"))))
    });
    c.bench_function("usa lorem", |b| {
        b.iter(|| usa_call(black_box(include_str!("./lorem.txt"))))
    });
}

fn war_and_peace_benchmark(c: &mut Criterion) {
    c.bench_function("sponge war and peace", |b| {
        b.iter(|| sponge_call(black_box(include_str!("./war_and_peace.txt"))))
    });
    c.bench_function("usa war and peace", |b| {
        b.iter(|| usa_call(black_box(include_str!("./war_and_peace.txt"))))
    });
}

criterion_group!(short, short_benchmark);
criterion_group!(lorem, lorem_benchmark);
criterion_group!(war_and_peace, war_and_peace_benchmark);
criterion_main!(short, lorem, war_and_peace);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use technical_analysis::indicators;
use technical_analysis::Indicator;

fn benchmark_sma(c: &mut Criterion) {
    let mut sma = indicators::SMA::factory().build().unwrap();
    c.bench_function("bench_sma", |b| b.iter(|| sma.next(black_box(20.0))));
}

criterion_group!(benches, benchmark_sma);
criterion_main!(benches);


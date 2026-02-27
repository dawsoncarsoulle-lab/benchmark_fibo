use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fibo::{fibo1, fibo2, fibo3, fibo4};
use std::{collections::HashMap, hint::black_box};

fn benchmark_fibo(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    group.plot_config(
        criterion::PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic),
    );
    for n in (6..=186).step_by(5) {
        // group.bench_with_input(BenchmarkId::new("fibo1", n), &n, |b, &n| {
        //     b.iter(|| fibo1(black_box(n)))
        // });
        group.bench_with_input(BenchmarkId::new("fibo2", n), &n, |b, &n| {
            b.iter(|| fibo2(black_box(n)))
        });
        group.bench_with_input(BenchmarkId::new("fibo3", n), &n, |b, &n| {
            b.iter(|| fibo3(black_box(n)))
        });
        group.bench_with_input(BenchmarkId::new("fibo4", n), &n, |b, &n| {
            b.iter_with_setup(|| HashMap::new(), |mut memo| fibo4(black_box(n), &mut memo))
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_fibo);
criterion_main!(benches);

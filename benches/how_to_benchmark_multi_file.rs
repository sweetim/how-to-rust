use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_how_to_benchmark_fn(c: &mut Criterion) {
    let mut group = c.benchmark_group("how_to_benchmark_fn");

    for i in [10, 100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(BenchmarkId::new("fast_function", i), i, |b, &i| {
            b.iter(|| {
                std::hint::black_box({
                    let _sum: u64 = (1..=i).sum();
                });
            });
        });

        group.bench_with_input(BenchmarkId::new("slow_function", i), i, |b, &i| {
            b.iter(|| {
                std::hint::black_box({
                    let mut _sum = 0;
                    for i in 1..=i {
                        _sum += i;
                    }
                });
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_how_to_benchmark_fn);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_recursive_memoized(n: u64) -> u64 {
    fn inner(n: u64, penultimate: u64, last: u64) -> u64 {
        match n {
            0 => penultimate,
            1 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }

    inner(n, 0, 1)
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn bench_how_to_benchmark_multi_file(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    for i in [10, 20, 30, 45].iter() {
        group.bench_with_input(BenchmarkId::new("recursive", i), i, |b, &i| {
            b.iter(|| {
                std::hint::black_box({
                    fibonacci_recursive(i);
                });
            });
        });

        group.bench_with_input(BenchmarkId::new("recursive-memoized", i), i, |b, &i| {
            b.iter(|| {
                std::hint::black_box({
                    fibonacci_recursive_memoized(i);
                });
            });
        });

        group.bench_with_input(BenchmarkId::new("iterative", i), i, |b, &i| {
            b.iter(|| {
                std::hint::black_box({
                    fibonacci_iterative(i);
                });
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_how_to_benchmark_multi_file);
criterion_main!(benches);

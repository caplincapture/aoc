// in `benches/my_benchmark.rs`
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

static INPUT: &str = include_str!("../input.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("run");
    macro_rules! measure {
        ($name:ident) => {
            group.bench_function(stringify!($name), |b| {
                b.iter(|| day17test::$name(black_box(INPUT)))
            });
        };
    }
    measure!(naive);
    measure!(state_structs);
    measure!(arrays);
    measure!(small_vecs);

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
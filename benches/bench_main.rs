
use criterion::criterion_main;
mod benchmarks;

criterion_main! {
    benchmarks::bench_vec::vector_backed,
    benchmarks::bench_arr::array_backed,
}

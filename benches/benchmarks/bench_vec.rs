
use criterion::{Criterion, criterion_group};
use sliding_window::sliding_window::{new_with_vector_storage, SlidingWindow};
use sliding_window::storage_vec::VectorStorage;
use crate::benchmarks::fields::{MULT, SIZE};

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}
fn get_sliding_window() -> SlidingWindow<VectorStorage<Data>, Data> {
    new_with_vector_storage(SIZE, MULT)
}

fn vector_backed_benchmark(criterion: &mut Criterion)
{
    let d1 = Data{dats:0};
    let mut w = get_sliding_window();

    criterion.bench_function("vector_push", |bencher| {
        bencher.iter(||
            w.push(d1)
        )
    });
}

criterion_group! {
    name = vector_backed;
    config = Criterion::default().sample_size(100);
    targets =
    vector_backed_benchmark,
}
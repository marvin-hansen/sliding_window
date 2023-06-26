use criterion::{Criterion, criterion_group};

use sliding_window::sliding_window::{new_with_array_storage, SlidingWindow};
use sliding_window::storage_array::ArrayStorage;
use crate::benchmarks::fields::{MULT, SIZE};


#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

fn get_sliding_window() -> SlidingWindow<ArrayStorage<Data, SIZE, MULT>, Data> {
    new_with_array_storage()
}

fn array_backed_benchmark(criterion: &mut Criterion)
{
    let d1 = Data { dats: 0 };
    let mut w = get_sliding_window();

    criterion.bench_function("array_push", |bencher| {
        bencher.iter(||
            w.push(d1)
        )
    });
}


criterion_group! {
    name = array_backed;
    config = Criterion::default().sample_size(100);
    targets =
    array_backed_benchmark,
}
use criterion::{Criterion, criterion_group};

use generic_array::typenum::U1000;
use sliding_window::sliding_window::{new_with_generic_array_storage, SlidingWindow};
use sliding_window::storage_gen_arr::GenericArrayStorage;

use crate::benchmarks::fields::{SIZE};

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

fn get_sliding_window() -> SlidingWindow<GenericArrayStorage<Data, U1000>, Data> {
    new_with_generic_array_storage(SIZE)
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
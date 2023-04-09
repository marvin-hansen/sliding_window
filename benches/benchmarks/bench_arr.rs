use criterion::{Criterion, criterion_group};

use sliding_window::array_backed::SlidingWindow;
use generic_array::typenum::U10000;

use crate::benchmarks::fields::{SIZE};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

impl Default for &Data
{
    fn default() -> Self {
        &Data { dats: 0 }
    }
}


fn array_backed_benchmark(criterion: &mut Criterion)
{
    let d1 = Data{dats:0};
    let mut w: SlidingWindow<Data, U10000> = SlidingWindow::new(SIZE);

    criterion.bench_function("vector_push", |bencher| {
        bencher.iter(||
            w.push(&d1)
        )
    });
}


criterion_group! {
    name = array_backed;
    config = Criterion::default().sample_size(100);
    targets =
    array_backed_benchmark,
}
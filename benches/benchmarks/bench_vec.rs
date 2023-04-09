
use criterion::{Criterion, criterion_group};
use sliding_window::vector_backed::SlidingWindow;
use crate::benchmarks::fields::{MULT, SIZE};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}


fn vector_backed_benchmark(criterion: &mut Criterion)
{
    let d1 = Data{dats:0};
    let mut w: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);

    criterion.bench_function("vector_push", |bencher| {
        bencher.iter(||
            w.push(&d1)
        )
    });
}


criterion_group! {
    name = vector_backed;
    config = Criterion::default().sample_size(100);
    targets =
    vector_backed_benchmark,
}
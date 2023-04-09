use sliding_window::vector_backed::SlidingWindow;

fn main() {

    let d1 = Data{dats:0};

    let mut window: SlidingWindow<Data> = SlidingWindow::new(2,4);

    window.push(&d1);

    let first = window.first().unwrap();
    assert_eq!(first.dats, 0);
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}
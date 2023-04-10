use sliding_window::array_backed::SlidingWindow;
use generic_array::typenum::U8;

const SIZE: usize = 4;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

impl Default for &Data {
    fn default() -> Self {
        &Data { dats: 0 }
    }
}

fn main() {

    let mut window: SlidingWindow<Data, U8> = SlidingWindow::new(SIZE);
    assert_eq!(window.empty(), true);
    assert_eq!(window.filled(), false);
    println!("Filled {:?}", window.filled());

    window.push(&Data{dats:0});
    assert_eq!(window.filled(), false);

    window.push(& Data{dats:1});
    assert_eq!(window.filled(), false);

    window.push(& Data{dats:2});
    assert_eq!(window.filled(), false);

    // Filled
    window.push(& Data{dats:3});
    assert_eq!(window.filled(), true);
    println!("Filled {:?}", window.filled());

    let res = window.first();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    assert_eq!(data.dats,0);
    println!("First {:?}", data.dats);

    let res = window.last();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    assert_eq!(data.dats,3);
    println!("Last {:?}", data.dats);

    // Filled
    window.push(& Data{dats:4});
    assert_eq!(window.filled(), true);

    window.push(& Data{dats:5});
    assert_eq!(window.filled(), true);

    window.push(& Data{dats:6});
    assert_eq!(window.filled(), true);

    window.push(& Data{dats:7});
    assert_eq!(window.filled(), true);

    window.push(& Data{dats:8});
    assert_eq!(window.filled(), true);

    let res = window.first();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    println!("First {:?}", data.dats);

    let res = window.last();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    println!("Last {:?}", data.dats);
}
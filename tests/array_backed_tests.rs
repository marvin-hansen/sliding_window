use sliding_window::array_backed::SlidingWindow;

const SIZE: usize = 4;
// const MULT: usize = 8;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

impl Default for &Data {
    fn default() -> Self {
        &Data { dats: 0 }
    }
}

#[test]
fn test_new() {
    let  window: SlidingWindow<Data> = SlidingWindow::new(SIZE);
    assert_eq!(window.empty(), true);
    assert_eq!(window.len(), SIZE);
}


#[test]
fn test_empty() {
    let d1 = Data{dats:0};
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE);
    assert_eq!(window.empty(), true);

    window.push(&d1);    assert_eq!(window.len(), SIZE);
    assert_eq!(window.empty(), false);
}



#[test]
fn test_push() {
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE);
    assert_eq!(window.len(), SIZE);
    assert_eq!(window.filled(), false);
    assert_eq!(window.empty(), true);

    let d1 = Data{dats:0};
    window.push(&d1);
    assert_eq!(window.filled(), false);
    assert_eq!(window.empty(), false);
}


#[test]
fn test_filled() {
    let d = Data{dats:0};
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE);
    assert_eq!(window.len(), SIZE);

    assert_eq!(window.len(), SIZE);
    assert_eq!(window.filled(), false);

    window.push(&d);
    assert_eq!(window.filled(), false);

    window.push(& d);
    assert_eq!(window.filled(), false);

    window.push(& d);
    assert_eq!(window.filled(), false);

    // Filled
    window.push(& d);
    assert_eq!(window.filled(), true);

    window.push(& d);
    assert_eq!(window.filled(), true);

    window.push(& d);
    assert_eq!(window.filled(), true);

    // window.push(& d);
    // assert_eq!(window.filled(), true);
    //
    // window.push(& d);
    // assert_eq!(window.filled(), true);
    //
    // // Rewinds b/c max capacity of 8 was reached
    // window.push(& d);
    // assert_eq!(window.filled(), false);
    //
    // window.push(& d);
    // assert_eq!(window.filled(), false);
}


#[test]
fn test_first() {
    let d = Data{dats:0};
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE);    assert_eq!(window.len(), SIZE);
    assert_eq!(window.len(), SIZE);
    assert_eq!(window.filled(), false);

    let res = window.first();
    assert_eq!(res.is_err(), true);

    window.push(&d);
    assert_eq!(window.filled(), false);

    let res = window.first();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    assert_eq!(data.dats,0);
}


#[test]
fn test_last() {
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE);    assert_eq!(window.len(), SIZE);
    assert_eq!(window.len(), SIZE);
    assert_eq!(window.filled(), false);

    let res = window.last();
    assert_eq!(res.is_err(), true);

    let d = Data{dats:0};
    window.push(&d);
    window.push(&d);
    window.push(&d);
    window.push(&d);
    assert_eq!(window.filled(), true);

    let res = window.first();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    assert_eq!(data.dats,0);

    let d = Data{dats:42};
    window.push(&d);

    let res = window.last();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    assert_eq!(data.dats,42);
}



#[test]
fn test_slice() {
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE);
    assert_eq!(window.len(), SIZE);
    assert_eq!(window.len(), SIZE);
    assert_eq!(window.filled(), false);

    let d = Data{dats:0};
    window.push(&d);
    window.push(&d);
    window.push(&d);
    window.push(&d);
    assert_eq!(window.filled(), true);

    let d = Data{dats:42};
    window.push(&d);

    let slice = window.slice();
    assert_eq!(slice.len(), SIZE);

    assert_eq!(slice[0].dats, 0);
    assert_eq!(slice[1].dats, 0);
    assert_eq!(slice[2].dats, 0);
    assert_eq!(slice[3].dats, 42);
}


#[test]
fn test_reverse_slice() {
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE);    assert_eq!(window.len(), SIZE);
    assert_eq!(window.len(), SIZE);
    assert_eq!(window.filled(), false);

    let d = Data{dats:0};
    window.push(&d);
    window.push(&d);
    window.push(&d);
    window.push(&d);
    assert_eq!(window.filled(), true);

    let d = Data{dats:42};
    window.push(&d);

    let slice = window.reverse_slice();
    assert_eq!(slice.len(), SIZE);

    assert_eq!(slice[0].dats, 42);
    assert_eq!(slice[1].dats, 0);
    assert_eq!(slice[2].dats, 0);
    assert_eq!(slice[3].dats, 0);
}
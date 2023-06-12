use sliding_window::vector_backed::SlidingWindow;

const SIZE: usize = 4;
const MULT: usize = 2;

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

#[test]
fn test_new() {
    let  window: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);
    assert_eq!(window.empty(), true);
    assert_eq!(window.len(), SIZE);
}

#[test]
fn test_empty() {
    let d1 = Data{dats:0};
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);
    assert_eq!(window.empty(), true);

    window.push(&d1);    assert_eq!(window.len(), SIZE);
    assert_eq!(window.empty(), false);
}

#[test]
fn test_push() {
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);    assert_eq!(window.len(), SIZE);
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
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);    assert_eq!(window.len(), SIZE);
    
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

    window.push(& d);
    assert_eq!(window.filled(), true);

    window.push(& d);
    assert_eq!(window.filled(), true);

    // Rewinds b/c max capacity of 8 was reached
    window.push(& d);
    assert_eq!(window.filled(), true);

    window.push(& d);
    assert_eq!(window.filled(), true);
}

#[test]
fn test_first() {
    let d = Data{dats:0};
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);    assert_eq!(window.len(), SIZE);
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
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);    assert_eq!(window.len(), SIZE);
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
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);    assert_eq!(window.len(), SIZE);
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
fn test_vec() {
    let d1 = &Data{dats:0};
    let mut window: SlidingWindow<Data> = SlidingWindow::new(SIZE,MULT);    assert_eq!(window.len(), SIZE);

    assert_eq!(window.len(), SIZE);
    assert_eq!(window.filled(), false);

    window.push(d1);
    window.push(d1);
    window.push(d1);
    window.push(d1);
    assert_eq!(window.filled(), true);

    let d2 = &Data { dats: 42 };
    window.push(d2);

    let e1 = window.first().unwrap();
    assert_eq!(e1.dats, d1.dats);

    let e2 = window.last().unwrap();
    assert_eq!(e2.dats, d2.dats);

    let v = window.vec();
    assert_eq!(v.len(), SIZE);

    let e1 = window.first().unwrap();
    let v1 = v.get(0).unwrap();
    assert_eq!(e1.dats, v1.dats);

    let e2 = window.last().unwrap();
    let v2 = v.get(SIZE - 1).unwrap();
    assert_eq!(e2.dats, v2.dats);
}
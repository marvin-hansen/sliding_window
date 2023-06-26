/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */

use generic_array::typenum::U8;

use sliding_window::sliding_window::{new_with_generic_array_storage, SlidingWindow};
use sliding_window::storage_gen_arr::GenericArrayStorage;

const SIZE: usize = 4;

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Data {
    dats: i32,
}

fn get_sliding_window() -> SlidingWindow<GenericArrayStorage<Data, U8>, Data> {
    new_with_generic_array_storage(SIZE)
}

#[test]
fn test_new() {
    let window = get_sliding_window();
    assert_eq!(window.empty(), true);
    assert_eq!(window.size(), SIZE);
}

#[test]
fn test_empty() {
    let d1 = Data { dats: 0 };
    let mut window = get_sliding_window();
    assert_eq!(window.empty(), true);

    window.push(d1);
    assert_eq!(window.size(), SIZE);
    assert_eq!(window.empty(), false);
}

#[test]
fn test_push() {
    let mut window = get_sliding_window();
    assert_eq!(window.size(), SIZE);
    assert_eq!(window.filled(), false);
    assert_eq!(window.empty(), true);

    let d1 = Data { dats: 0 };
    window.push(d1);
    assert_eq!(window.filled(), false);
    assert_eq!(window.empty(), false);
}

#[test]
fn test_filled() {
    let d = Data { dats: 0 };
    let mut window = get_sliding_window();

    assert_eq!(window.size(), SIZE);
    assert_eq!(window.filled(), false);

    window.push(d);
    assert_eq!(window.filled(), false);

    window.push(d);
    assert_eq!(window.filled(), false);

    window.push(d);
    assert_eq!(window.filled(), false);

    // Filled
    window.push(d);
    assert_eq!(window.filled(), true);

    window.push(d);
    assert_eq!(window.filled(), true);

    window.push(d);
    assert_eq!(window.filled(), true);

    window.push(d);
    assert_eq!(window.filled(), true);

    window.push(d);
    assert_eq!(window.filled(), true);

    // Rewinds b/c max capacity of 8 was reached
    window.push(d);
    assert_eq!(window.filled(), true);

    window.push(d);
    assert_eq!(window.filled(), true);
}

#[test]
fn test_first() {
    let d = Data { dats: 0 };
    let mut window = get_sliding_window();
    assert_eq!(window.size(), SIZE);
    assert_eq!(window.filled(), false);

    let res = window.first();
    assert_eq!(res.is_err(), true);

    window.push(d);
    assert_eq!(window.filled(), false);

    let res = window.first();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    assert_eq!(data.dats, 0);
}

#[test]
fn test_last() {
    let mut window = get_sliding_window();
    assert_eq!(window.size(), SIZE);
    assert_eq!(window.filled(), false);

    let res = window.last();
    assert_eq!(res.is_err(), true);

    let d = Data { dats: 0 };
    window.push(d);
    window.push(d);
    window.push(d);
    window.push(d);
    assert_eq!(window.filled(), true);

    let res = window.first();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    assert_eq!(data.dats, 0);

    let d = Data { dats: 42 };
    window.push(d);

    let res = window.last();
    assert_eq!(res.is_ok(), true);

    let data = res.unwrap();
    assert_eq!(data.dats, 42);
}

#[test]
fn test_slice() {
    let mut window = get_sliding_window();
    assert_eq!(window.size(), SIZE);
    assert_eq!(window.filled(), false);

    let d = Data { dats: 0 };
    window.push(d);
    window.push(d);
    window.push(d);
    window.push(d);
    assert_eq!(window.filled(), true);

    let d = Data { dats: 42 };
    window.push(d);

    let slice = window.slice().expect("Failed to get slice");
    assert_eq!(slice.len(), SIZE);
    assert_eq!(slice[0].dats, 0);
    assert_eq!(slice[1].dats, 0);
    assert_eq!(slice[2].dats, 0);
    assert_eq!(slice[3].dats, 42);

    let d = Data { dats: 0 };
    window.push(d);
    assert_eq!(window.filled(), true);

    let slice = window.slice().expect("Failed to get slice");
    assert_eq!(slice.len(), SIZE);
    assert_eq!(slice[0].dats, 0);
    assert_eq!(slice[1].dats, 0);
    assert_eq!(slice[2].dats, 42);
    assert_eq!(slice[3].dats, 0);
}

#[test]
fn test_vec() {
    let d1 = Data { dats: 0 };
    let mut window = get_sliding_window();

    assert_eq!(window.size(), SIZE);
    assert_eq!(window.filled(), false);

    window.push(d1);
    window.push(d1);
    window.push(d1);
    window.push(d1);
    assert_eq!(window.filled(), true);

    let d2 = Data { dats: 42 };
    window.push(d2);

    let e1 = window.first().unwrap();
    assert_eq!(e1.dats, d1.dats);

    let e2 = window.last().unwrap();
    assert_eq!(e2.dats, d2.dats);

    let v = window.vec().expect("Failed to get vec");
    assert_eq!(v.len(), SIZE);

    let e1 = window.first().unwrap();
    let v1 = v.get(0).unwrap();
    assert_eq!(e1.dats, v1.dats);

    let e2 = window.last().unwrap();
    let v2 = v.get(SIZE - 1).unwrap();
    assert_eq!(e2.dats, v2.dats);
}

#[test]
fn test_arr() {
    let d1 = Data { dats: 0 };
    let mut window = get_sliding_window();
    assert_eq!(window.size(), SIZE);
    assert_eq!(window.filled(), false);

    window.push(d1);
    window.push(d1);
    window.push(d1);
    window.push(d1);
    assert_eq!(window.filled(), true);

    let slice = window.slice().expect("Failed to get slice");
    assert_eq!(slice.len(), SIZE);

    // Filled
    let d2 = Data { dats: 42 };
    window.push(d2);
    assert_eq!(window.filled(), true);

    let e1 = window.first().unwrap();
    assert_eq!(e1.dats, d1.dats);

    let e2 = window.last().unwrap();
    assert_eq!(e2.dats, d2.dats);

    let slice = window.slice().expect("Failed to get slice");
    assert_eq!(slice.len(), SIZE);

    let arr: [Data; SIZE] = window.arr().unwrap();
    assert_eq!(arr.len(), SIZE);

    let e1 = window.first().unwrap();
    let a1 = arr.get(0).unwrap();
    assert_eq!(e1.dats, a1.dats);

    let e2 = window.last().unwrap();
    let a2 = arr.get(SIZE - 1).unwrap();
    assert_eq!(e2.dats, a2.dats);

    assert_eq!(arr[0].dats, 0);
    assert_eq!(arr[1].dats, 0);
    assert_eq!(arr[2].dats, 0);
    assert_eq!(arr[3].dats, 42);

    let d = Data { dats: 0 };
    window.push(d);

    let arr: [Data; SIZE] = window.arr().unwrap();
    assert_eq!(arr.len(), SIZE);

    assert_eq!(arr[0].dats, 0);
    assert_eq!(arr[1].dats, 0);
    assert_eq!(arr[2].dats, 42);
    assert_eq!(arr[3].dats, 0);
}
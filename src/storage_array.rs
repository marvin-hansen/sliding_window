/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */

use crate::storage::Storage;

pub struct ArrayStorage<T, const SIZE: usize, const CAPACITY: usize>
    where
        T: PartialEq + Copy + Default,
        [T; CAPACITY]: Sized,
{
    arr: [T; CAPACITY],
    size: usize,
    head: usize,
    tail: usize,
}

impl<T, const SIZE: usize, const CAPACITY: usize> ArrayStorage<T, SIZE, CAPACITY>
    where
        T: PartialEq + Copy + Default,
        [T; CAPACITY]: Sized,
{
    pub fn new() -> Self
    {
        Self {
            arr: [T::default(); CAPACITY],
            size: SIZE,
            head: 0,
            tail: 0,
        }
    }
}


impl<T, const SIZE: usize, const CAPACITY: usize> Storage<T> for ArrayStorage<T, SIZE, CAPACITY>
    where
        T: PartialEq + Copy + Default,
        [T; SIZE]: Sized,
{
    fn push(&mut self, value: T) {
        // if the array is full, rewind
        if self.tail > 0 && self.tail == self.arr.len()
        {
            // rewind
            for i in 0..self.size - 1
            {
                self.arr[i] = self.arr[self.head + i];
            }
            self.head = 0;
            self.tail = self.size;
        }

        // push the value
        self.arr[self.tail] = value;

        // check if the window is full,
        if self.tail - self.head > self.size
        {
            // move head cursor one position forward
            self.head += 1;
        }

        //increase tail cursor to next position
        self.tail += 1;
    }

    fn first(&self) -> Result<T, String> {
        return if self.tail != 0 {
            Ok(self.arr[self.head])
        } else {
            Err(format!("Array is empty. Add some elements to the array first"))
        };
    }

    fn last(&self) -> Result<T, String> {
        return if self.filled() {
            Ok(self.arr[self.tail - 1])
        } else {
            Err(format!("Array is not yet filled. Add some elements to the array first"))
        };
    }

    fn tail(&self) -> usize {
        self.tail
    }

    fn size(&self) -> usize {
        self.size
    }

    fn get_slice(&self) -> &[T] {
        if self.tail > self.size
        {
            // Adjust offset in case the window is larger than the slice.
            &self.arr[self.head + 1..self.tail]
        } else {
            &self.arr[self.head..self.tail]
        }
    }
}
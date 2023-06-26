/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */

use generic_array::{ArrayLength, GenericArray};
use crate::storage::{Storage};

pub struct GenericArrayStorage<T, N>
    where
        T: PartialEq + Copy + Default ,
        N: ArrayLength<T>,
{
    arr: GenericArray<T, N>,
    size: usize,
    head: usize,
    tail: usize,
}

impl<T, N> GenericArrayStorage<T, N>
    where
        T: PartialEq + Copy + Default,
        N: ArrayLength<T>,
{
    pub fn new(size: usize) -> Self
    {
        Self
        {
            arr: GenericArray::default(),
            size,
            head: 0,
            tail: 0,
        }
    }
}

impl<T, N> Storage<T> for GenericArrayStorage<T, N>
    where
        T: PartialEq + Copy + Default,
        N: ArrayLength<T>,

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
        }
    }

    fn last(&self) -> Result<T, String> {
        return if self.filled() {
            Ok(self.arr[self.tail - 1])
        } else {
            Err(format!("Array is not yet filled. Add some elements to the array first"))
        };
    }

    #[inline(always)]
    fn tail(&self) -> usize{
        self.tail
    }

    #[inline(always)]
    fn size(&self) -> usize{
        self.size
    }

    #[inline(always)]
    fn get_slice(&self) -> &[T]
    {
        if self.tail > self.size
        {
            // Adjust offset in case the window is larger than the slice.
            &self.arr[self.head + 1..self.tail]
        } else {
            &self.arr[self.head..self.tail]
        }
    }
}

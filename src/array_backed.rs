/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */

use generic_array::{ArrayLength, GenericArray};

pub struct SlidingWindow<T, N>
    where
        T: PartialEq + Copy,
        T: Default,
        N: ArrayLength<T>,
{
    arr: GenericArray<T, N>,
    size: usize,
    head: usize,
    tail: usize,
}

impl<T, N> SlidingWindow<T, N>
    where
        T: PartialEq + Copy,
        T: Default,
        N: ArrayLength<T>,
{
    /// Create a new sliding window with the size N as argument and max capacity C specified
    /// as constant generic parameter.
    /// This data structures trades off space and copying complexity;
    /// the number of elements a sing windows can hold without array copying
    /// is approx M-1, where M is the multiple. For example, if the window size N is 7,
    /// and the max capacity C is 49 (7*7), then the sliding window can hold up to 48 elements
    /// before rewind performs an array copy. After a rewinds, the window needs ro refill before
    /// the entire window can be accessed hence call filled() to check if the window is ready.
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

impl<T, N> SlidingWindow<T, N>
    where
        T: PartialEq + Copy,
        T: Default,
        N: ArrayLength<T>,
{
    pub fn push(&mut self, value: T)
    {
        // if the array is full, rewind
        if self.tail > 0 && self.tail == self.arr.len()
        {
            self.rewind()
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

    /// Returns true if the window is empty
    #[inline(always)]
    pub fn empty(&self) -> bool
    {
        return if self.tail == 0 {
            true
        } else {
            false
        }
    }

    /// Returns the first element in the sliding window
    #[inline(always)]
    pub fn first(&self) -> Result<T, String>
    {
        return if self.tail != 0 {
            Ok(self.arr[self.head])
        } else {
            Err(format!("Array is empty. Add some elements to the array first"))
        }
    }

    /// Returns the last element in the sliding window
    #[inline(always)]
    pub fn last(&self) -> Result<T, String>
    {
        return if self.filled() {
            Ok(self.arr[self.tail - 1])
        } else {
            Err(format!("Array is not yet filled. Add some elements to the array first"))
        };
    }

    #[inline(always)]
    pub fn filled(&self) -> bool
    {
        return if self.tail < self.size {
            false
        } else {
            true
        };
    }

    /// Returns the size of the moving window,
    #[inline(always)]
    pub fn len(&self) -> usize
    {
        self.size
    }

    #[inline(always)]
    fn rewind(&mut self)
    {
        for i in 0..self.size - 1
        {
            self.arr[i] = self.arr[self.head + i];
        }
        self.head = 0;
        self.tail = self.size;
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

impl<T, N> SlidingWindow<T, N>
    where
        T: PartialEq + Copy,
        T: Default,
        N: ArrayLength<T>,
{
    /// Returns the sliding window as a fixed size static array.
    pub fn arr<const S: usize>(&self) -> Result<[T; S], String> {
        if !self.filled() {
            return Err(format!("Sliding window is not yet filled. Add some elements to the array first"))
        }

        let mut arr: [T; S] = [T::default(); S];
        let slice = self.get_slice();
        for i in 0..arr.len() {
            arr[i] = slice[i];
        }

        Ok(arr)
    }

    /// Returns the sliding window as a slice.
    pub fn slice(&self) -> Result<&[T], String> {
        return if !self.filled() {
            Err(format!("Sliding window is not yet filled. Add some elements to the array first"))
        } else {
            Ok(self.get_slice())
        }
    }


    /// Returns the sliding window as a vector.
    pub fn vec(&self) -> Result<Vec<T>, String> {
        return if !self.filled() {
            Err(format!("Sliding window is not yet filled. Add some elements to the array first"))
        } else {
            Ok(self.get_slice().to_vec())
        }
    }
}

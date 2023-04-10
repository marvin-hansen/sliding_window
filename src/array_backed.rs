use generic_array::{ArrayLength, GenericArray};

pub struct SlidingWindow<'l, T, N>
    where
       T: PartialEq + Copy,
        &'l T: Default,
        N: ArrayLength<&'l T>,
{
    arr: GenericArray<&'l T,N>,
    size: usize,
    head: usize,
    tail: usize,
}


impl<'l, T,N > SlidingWindow<'l, T, N>
    where
        T: PartialEq + Copy,
        &'l T: Default,
        N: ArrayLength<&'l T>,
{
    pub fn new(size:usize) -> Self {
        Self {
            arr: GenericArray::default(),
            size,
            head: 0,
            tail: 0,
        }
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
    pub fn first(&self) -> Result<T, String>
    {
        return if self.tail != 0 {
            Ok(*self.arr[self.head])
        } else {
            Err(format!("Array is empty. Add some elements to the array first"))
        }
    }

    /// Returns the last element in the sliding window
    pub fn last(&self) -> Result<T, String>
    {
        return if self.filled() {
            Ok(*self.arr[self.tail-1])
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

    pub fn push(&mut self, value: &'l T)
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

    /// Returns the sliding window as a slice.
    pub fn slice(&self) -> &[&T]
    {
        &self.arr[self.head+1..self.tail]
    }

    /// Returns the sliding window with its items in reverse order.
    pub fn reverse_slice(&mut self) -> &[&T]
    {
        let s = &mut self.arr[self.head+1..self.tail];
        s.reverse();
        s
    }

    /// Returns the size of the moving window,
    pub fn len(&self) -> usize
    {
        self.size
    }

    pub fn vec(&self) -> Vec<&T>{
        let mut vec = Vec::with_capacity(self.size);

        for i in self.head+1..self.tail {
            vec.push(self.arr[i]);
        }

        vec
}

    pub fn reverse_vec(&self) -> Vec<&T>{
        let mut vec = Vec::with_capacity(self.size);

        for i in self.head+1..self.tail {
            vec.push(self.arr[i]);
        }

        vec
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
}

pub struct SlidingWindow<'l, T>
    where T: PartialEq + Copy
{
    arr: Vec<&'l T>,
    size: usize,
    head: usize,
    tail: usize,
}

impl<'l,T> SlidingWindow<'l,T>
    where T: PartialEq + Copy
{
    pub fn new(size: usize, multiple: usize) -> Self {
        let capacity = size * multiple;
        Self {
            arr: Vec::with_capacity(capacity),
            size,
            head: 0,
            tail: 0,
        }
    }

    /// Adds a new element to the sliding window.
    pub fn push(&mut self, value: &'l T)
    {
        // if the array is full, rewind
        if self.tail > 0 && self.tail == self.arr.capacity()
        {
            self.rewind()
        }

        // push the value
        // self.arr[self.tail] = value;
        self.arr.push(value);

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
            Ok(*self.arr[self.head])
        } else {
            Err(format!("Array is empty. Add some elements to the array first"))
        }
    }

    /// Returns the last element in the sliding window
    #[inline(always)]
    pub fn last(&self) -> Result<T, String>
    {
        return if self.filled() {
            Ok(*self.arr[self.tail-1])
        } else {
            Err(format!("Array is not yet filled. Add some elements to the array first"))
        };
    }

    /// Returns true if the window has reached its size
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

    /// Returns the sliding window as a slice.
    #[inline(always)]
    pub fn slice(&self) -> &[&T]
    {
        &self.get_slice()
    }

    /// Returns the sliding window as a vector.
    #[inline(always)]
    pub fn vec(&self) -> Vec<&T> {
        self.get_slice().to_vec()
    }

    #[inline(always)]
    fn get_slice(&self) -> &[&T]
    {
        if self.tail > self.size
        {
            // Adjust offset in case the window is larger than the slice.
            &self.arr[self.head + 1..self.tail]
        } else {
            &self.arr[self.head..self.tail]
        }
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
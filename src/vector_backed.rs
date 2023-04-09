
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
    pub fn new(size:usize, multiple:usize) -> Self {
        let capacity = size * multiple;
        Self {
            arr: Vec::with_capacity(capacity),
            size,
            head: 0,
            tail: 0,
        }
    }

    /// Returns true if the window is empty
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

    /// Returns true if the window has reached its size
    pub fn filled(&self) -> bool
    {
        return if self.tail < self.size {
            false
        } else {
            true
        };
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

    fn rewind(&mut self)
    {
        let l = self.arr.len();

        for i in 0..self.size - 1
        {
            self.arr[i] = self.arr[l - (self.size + i)];
        }

        self.head = 0;
        self.tail = 0;
    }
}
use std::marker::PhantomData;
use generic_array::ArrayLength;
use crate::storage::{Storage};
use crate::storage_gen_arr::GenericArrayStorage;
use crate::storage_vec::VectorStorage;

pub fn new_with_vector_storage<T: PartialEq + Copy + Default >(
    size: usize,
    multiple: usize,
)
    -> SlidingWindow<VectorStorage<T>, T>
{
    SlidingWindow::with_storage(
        VectorStorage::new(size, multiple)
    )
}

pub fn new_with_generic_array_storage<T: PartialEq + Copy + Default ,  N: ArrayLength<T>,>(
    size: usize,
)
    -> SlidingWindow<GenericArrayStorage<T, N>, T>
{
    SlidingWindow::with_storage(
        GenericArrayStorage::new(size)
    )
}

pub struct SlidingWindow<S, T>
    where
        T: PartialEq + Copy + Default ,
        S: Storage<T>,
{
    storage: S,
    ty: PhantomData<T>,
}

impl<S, T> SlidingWindow<S, T>
    where
        T: PartialEq + Copy + Default ,
        S: Storage<T>,
{
    pub(crate) fn with_storage(storage: S) -> Self
    {
        Self { storage, ty: Default::default() }
    }
}


impl<S, T> SlidingWindow<S, T>
    where
        T: PartialEq + Copy + Default,
        S: Storage<T>,
{
    pub fn push(&mut self, value: T)
    {
        self.storage.push(value)
    }

    pub fn first(&self) -> Result<T, String>
    {
        self.storage.first()
    }

    pub fn last(&self) -> Result<T, String>
    {
        self.storage.last()
    }

    pub fn empty(&self) -> bool
    {
        self.storage.empty()
    }

    pub fn filled(&self) -> bool
    {
        self.storage.filled()
    }

    pub fn size(&self) -> usize
    {
        self.storage.size()
    }

    pub fn arr<const SIZE: usize>(&self) -> Result<[T; SIZE], String> {
        self.storage.arr()
    }

    pub fn slice(&self) -> Result<&[T], String> {
        self.storage.slice()
    }

    pub fn vec(&self) -> Result<Vec<T>, String> {
        self.storage.vec()
    }
}
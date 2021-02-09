use crate::{new::index::iters, *};
use std::{marker::PhantomData, ops::Range};

pub type Iter1D<M, T> = Iter<iters::Index1D<M>, T, unsafe fn(T, Point) -> <T as Grid>::Item>;
pub type Iter2D<M, T, Item> =
    Iter<iters::Index2D<M>, T, unsafe fn(T, (usize, Range<usize>)) -> Item>;
pub type ColIter<T> = Iter1D<ColMajor, T>;
pub type RowIter<T> = Iter1D<RowMajor, T>;
pub type ColsIter<T> = Iter2D<ColMajor, T, <T as Grid>::Col>;
pub type RowsIter<T> = Iter2D<RowMajor, T, <T as Grid>::Row>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Iter<I, T, F> {
    index: I,
    grid:  T,
    func:  F,
}

impl<M, T, F> Iter<iters::Index1D<M>, T, F> {
    /// SAFETY: TODO
    pub unsafe fn new<TM: Major>(grid: T, index: impl Index1D, func: F) -> Option<Self>
    where
        T: WithMSize<TM>,
    {
        let index = index.checked(grid.msize())?.into();

        Some(Self { index, grid, func })
    }

    /// SAFETY: TODO
    pub unsafe fn new_unchecked<TM: Major>(grid: T, index: impl Index1D, func: F) -> Self
    where
        T: WithMSize<TM>,
    {
        let index = index.unchecked(grid.msize()).into();

        Self { index, grid, func }
    }
}

impl<M: Major, T: WithSize, F> Iter<iters::Index2D<M>, T, F> {
    /// SAFETY: TODO
    pub unsafe fn new(grid: T, index: impl Index2D, func: F) -> Option<Self> {
        let index = index.checked(grid.size())?.into();

        Some(Self { index, grid, func })
    }

    /// SAFETY: TODO
    pub unsafe fn new_unchecked(grid: T, index: impl Index2D, func: F) -> Self {
        let index = index.unchecked(grid.size()).into();

        Self { index, grid, func }
    }
}

impl<'a, I: Iterator, T, Item> Iterator for Iter<I, &'a T, unsafe fn(&'a T, I::Item) -> Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.next()?;

        // SAFETY: TODO
        Some(unsafe { (self.func)(self.grid, index) })
    }
}

impl<'a, I: Iterator, T, Item> Iterator
    for Iter<I, &'a mut T, unsafe fn(&'a mut T, I::Item) -> Item>
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.next()?;

        // SAFETY:
        // constructors guaranty that:
        // item_unchecked returns valid, non-overlapping references.
        // Then, it is safe to extend grid's lifetime
        let grid = unsafe { std::mem::transmute::<&mut T, &mut T>(self.grid) };

        // SAFETY: TODO
        Some(unsafe { (self.func)(grid, index) })
    }
}

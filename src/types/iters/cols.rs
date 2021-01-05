use crate::*;
use std::{marker::PhantomData, ops::Range};

/// Generic columns iterator leveraging
/// [`Grid::col_unchecked`](crate::Grid::col_unchecked).
///
/// @see also [`Rows`](crate::Rows).
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Cols<I, T: Grid<I>> {
    grid:    T,
    index:   Point<Range<usize>>,
    phantom: PhantomData<I>,
}

pub type ColsRef<'a, I, T> = Cols<&'a I, &'a T>;
pub type ColsMut<'a, I, T> = Cols<&'a mut I, &'a mut T>;

impl<I, T: Grid<I>> Cols<I, T> {
    fn new_owned(grid: T, index: impl Index2D) -> Option<Self> {
        let (width, height) = grid.size().into();
        let Point { x, y } = index.checked(grid.size())?;

        // SAFETY: Index2D::checked guaranties:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { Self::new_unchecked_owned(grid, (x, y)) })
    }

    unsafe fn new_unchecked_owned(grid: T, index: impl Index2D) -> Self {
        let index = index.unchecked(grid.size());

        Self {
            grid,
            index,
            phantom: PhantomData,
        }
    }
}

/// @see [`ColsRef`](crate::ColsRef).
impl<'a, I, T> ColsRef<'a, I, T>
where
    &'a T: Grid<&'a I>,
{
    /// Returns a [`ColsRef`](crate::ColsRef), or `None` if out of bounds.
    pub fn new(grid: &'a T, index: impl Index2D) -> Option<Self> {
        Cols::new_owned(grid, index)
    }

    /// Returns a [`ColsRef`](crate::ColsRef), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    pub unsafe fn new_unchecked(grid: &'a T, index: impl Index2D) -> Self {
        Cols::new_unchecked_owned(grid, index)
    }
}

/// @see [`ColsMut`](crate::ColsMut).
impl<'a, I, T> ColsMut<'a, I, T>
where
    &'a mut T: Grid<&'a mut I>,
{
    /// Returns a [`ColsMut`](crate::ColsMut), or `None` if out of bounds.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::col_unchecked`](crate::Grid::col_unchecked) returns
    ///   non-overlapping references
    pub unsafe fn new(grid: &'a mut T, index: impl Index2D) -> Option<Self> {
        Cols::new_owned(grid, index)
    }

    /// Returns a [`ColsMut`](crate::ColsMut), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::col_unchecked`](crate::Grid::col_unchecked) returns
    ///   non-overlapping references
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    pub unsafe fn new_unchecked(grid: &'a mut T, index: impl Index2D) -> Self {
        Cols::new_unchecked_owned(grid, index)
    }
}

impl<'a, I, T> Iterator for ColsRef<'a, I, T>
where
    &'a T: Grid<&'a I>,
{
    type Item = <&'a T as Grid<&'a I>>::Col;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.index.x;

        if start < end {
            self.index.x.start += 1;

            // SAFETY: constructors guaranty this is safe
            Some(unsafe { self.grid.col_unchecked((start, self.index.y.clone())) })
        } else {
            None
        }
    }
}

impl<'a, I, T> Iterator for ColsMut<'a, I, T>
where
    &'a mut T: Grid<&'a mut I>,
{
    type Item = <&'a mut T as Grid<&'a mut I>>::Col;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.index.x;

        if start < end {
            self.index.x.start += 1;

            // SAFETY:
            // row_mut_unchecked returns valid, non-overlapping references.
            // Then, it is safe to extend iterator's items lifetime
            let grid = unsafe { std::mem::transmute::<&mut T, &mut T>(self.grid) };

            // SAFETY: constructors guaranty this is safe
            let it = unsafe { grid.col_unchecked((start, self.index.y.clone())) };

            Some(it)
        } else {
            None
        }
    }
}

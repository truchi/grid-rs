use crate::*;
use std::{marker::PhantomData, ops::Range};

/// Generic columns iterator leveraging
/// [`Grid::col_unchecked`](crate::Grid::col_unchecked).
///
/// @see also [`Cols`](crate::Cols).
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rows<I, T: Grid<I>> {
    grid:    T,
    index:   Point<Range<usize>>,
    phantom: PhantomData<I>,
}

pub type RowsRef<'a, I, T> = Rows<&'a I, &'a T>;
pub type RowsMut<'a, I, T> = Rows<&'a mut I, &'a mut T>;

impl<I, T: Grid<I>> Rows<I, T> {
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

/// @see [`RowsRef`](crate::RowsRef).
impl<'a, I, T> RowsRef<'a, I, T>
where
    &'a T: Grid<&'a I>,
{
    /// Returns a [`RowsRef`](crate::RowsRef), or `None` if out of bounds.
    pub fn new(grid: &'a T, index: impl Index2D) -> Option<Self> {
        Rows::new_owned(grid, index)
    }

    /// Returns a [`RowsRef`](crate::RowsRef), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    pub unsafe fn new_unchecked(grid: &'a T, index: impl Index2D) -> Self {
        Rows::new_unchecked_owned(grid, index)
    }
}

/// @see [`RowsMut`](crate::RowsMut).
impl<'a, I, T> RowsMut<'a, I, T>
where
    &'a mut T: Grid<&'a mut I>,
{
    /// Returns a [`RowsMut`](crate::RowsMut), or `None` if out of bounds.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::col_unchecked`](crate::Grid::col_unchecked) returns
    ///   non-overlapping references
    pub unsafe fn new(grid: &'a mut T, index: impl Index2D) -> Option<Self> {
        Rows::new_owned(grid, index)
    }

    /// Returns a [`RowsMut`](crate::RowsMut), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::col_unchecked`](crate::Grid::col_unchecked) returns
    ///   non-overlapping references
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    pub unsafe fn new_unchecked(grid: &'a mut T, index: impl Index2D) -> Self {
        Rows::new_unchecked_owned(grid, index)
    }
}

impl<'a, I, T> Iterator for RowsRef<'a, I, T>
where
    &'a T: Grid<&'a I>,
{
    type Item = <&'a T as Grid<&'a I>>::Row;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.index.y;

        if start < end {
            self.index.y.start += 1;

            // SAFETY: constructors guaranty this is safe
            Some(unsafe { self.grid.row_unchecked((start, self.index.x.clone())) })
        } else {
            None
        }
    }
}

impl<'a, I, T> Iterator for RowsMut<'a, I, T>
where
    &'a mut T: Grid<&'a mut I>,
{
    type Item = <&'a mut T as Grid<&'a mut I>>::Row;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.index.y;

        if start < end {
            self.index.y.start += 1;

            // SAFETY:
            // row_unchecked returns valid, non-overlapping references.
            // Then, it is safe to extend iterator's items lifetime
            let grid = unsafe { std::mem::transmute::<&mut T, &mut T>(self.grid) };

            // SAFETY: constructors guaranty this is safe
            let it = unsafe { grid.row_unchecked((start, self.index.x.clone())) };

            Some(it)
        } else {
            None
        }
    }
}

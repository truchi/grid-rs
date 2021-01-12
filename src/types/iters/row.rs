use crate::*;
use std::{marker::PhantomData, ops::Range};

/// Generic row iterator leveraging
/// [`Grid::item_unchecked`](crate::Grid::item_unchecked).
///
/// @see also [`Col`](crate::Col).
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Row<I, T> {
    grid:    T,
    row:     usize,
    range:   Range<usize>,
    phantom: PhantomData<I>,
}

pub type RowRef<'a, I, T> = Row<&'a I, &'a T>;
pub type RowMut<'a, I, T> = Row<&'a mut I, &'a mut T>;

impl<I, T: Grid<I>> Row<I, T> {
    /// Returns a [`Row`](crate::Row), or `None` if out of bounds.
    fn new_owned(grid: T, index: impl Index1D) -> Option<Self> {
        let index = index.row(grid.size())?;

        // SAFETY:
        // Index1D::row guaranties that:
        {
            let (width, height) = grid.size().into();
            let (row, range) = index.clone();
            debug_assert!(row < height);
            debug_assert!(range.start <= range.end);
            debug_assert!(range.end <= width);
        }
        Some(unsafe { Self::new_unchecked_owned(grid, index) })
    }

    /// Returns a [`Row`](crate::Row), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::item_unchecked`](crate::Grid::item_unchecked) returns
    ///   non-overlapping references (for [`RowMut`](crate::RowMut))
    /// - `row < height`
    /// - `start <= end`
    /// - `end <= height`
    unsafe fn new_unchecked_owned(grid: T, index: impl Index1D) -> Self {
        let (row, range) = index.row_unchecked(grid.size());

        Self {
            grid,
            row,
            range,
            phantom: PhantomData,
        }
    }
}

/// @see [`RowRef`](crate::RowRef).
impl<'a, I, T> RowRef<'a, I, T>
where
    &'a T: Grid<&'a I>,
{
    /// Returns a [`RowRef`](crate::RowRef), or `None` if out of bounds.
    pub fn new(grid: &'a T, index: impl Index1D) -> Option<Self> {
        Row::new_owned(grid, index)
    }

    /// Returns a [`RowRef`](crate::RowRef), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `row < height`
    /// - `start <= end`
    /// - `end <= height`
    pub unsafe fn new_unchecked(grid: &'a T, index: impl Index1D) -> Self {
        Row::new_unchecked_owned(grid, index)
    }
}

/// @see [`RowMut`](crate::RowMut).
impl<'a, I, T> RowMut<'a, I, T>
where
    &'a mut T: Grid<&'a mut I>,
{
    /// Returns a [`RowMut`](crate::RowMut), or `None` if out of bounds.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::item_unchecked`](crate::Grid::item_unchecked) returns
    ///   non-overlapping references
    pub unsafe fn new(grid: &'a mut T, index: impl Index1D) -> Option<Self> {
        Row::new_owned(grid, index)
    }

    /// Returns a [`RowMut`](crate::RowMut), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::item_unchecked`](crate::Grid::item_unchecked) returns
    ///   non-overlapping references
    /// - `row < height`
    /// - `start <= end`
    /// - `end <= height`
    pub unsafe fn new_unchecked(grid: &'a mut T, index: impl Index1D) -> Self {
        Row::new_unchecked_owned(grid, index)
    }
}

impl<'a, I, T> Iterator for RowRef<'a, I, T>
where
    &'a T: Grid<&'a I>,
{
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.range;

        if start < end {
            self.range.start += 1;
            let point = Point {
                x: start,
                y: self.row,
            };

            // SAFETY:
            // constructors guaranty that:
            debug_assert!(point < self.grid.size());
            Some(unsafe { self.grid.item_unchecked(point) })
        } else {
            None
        }
    }
}

impl<'a, I, T> Iterator for RowMut<'a, I, T>
where
    &'a mut T: Grid<&'a mut I>,
{
    type Item = &'a mut I;

    fn next(&mut self) -> Option<Self::Item> {
        let std::ops::Range { start, end } = self.range;

        if start < end {
            self.range.start += 1;
            let point = Point {
                x: start,
                y: self.row,
            };

            // SAFETY:
            // constructors guaranty that:
            // item_unchecked returns valid, non-overlapping references.
            // Then, it is safe to extend grid's lifetime
            let grid = unsafe { std::mem::transmute::<&mut T, &mut T>(self.grid) };

            // SAFETY:
            // constructors guaranty that:
            debug_assert!(point < self.grid.size());
            let item = unsafe { grid.item_unchecked(point) };

            Some(item)
        } else {
            None
        }
    }
}

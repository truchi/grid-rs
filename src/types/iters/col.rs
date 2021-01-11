use crate::*;
use std::{marker::PhantomData, ops::Range};

/// Generic column iterator leveraging
/// [`Grid::item_unchecked`](crate::Grid::item_unchecked).
///
/// @see also [`Row`](crate::Row).
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Col<I, T> {
    grid:    T,
    col:     usize,
    range:   Range<usize>,
    phantom: PhantomData<I>,
}

pub type ColRef<'a, I, T> = Col<&'a I, &'a T>;
pub type ColMut<'a, I, T> = Col<&'a mut I, &'a mut T>;

impl<I, T: Grid<I>> Col<I, T> {
    /// Returns a [`Col`](crate::Col), or `None` if out of bounds.
    fn new_owned(grid: T, index: impl Index1D) -> Option<Self> {
        let index = index.col(grid.size())?;

        // SAFETY:
        // Index1D::col guaranties that:
        {
            let (width, height) = grid.size().into();
            let (col, range) = index.clone();
            debug_assert!(col < width);
            debug_assert!(range.start <= range.end);
            debug_assert!(range.end <= height);
        }
        Some(unsafe { Self::new_unchecked_owned(grid, index) })
    }

    /// Returns a [`Col`](crate::Col), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::item_unchecked`](crate::Grid::item_unchecked) returns
    ///   non-overlapping references (for [`ColMut`](crate::ColMut))
    /// - `col < width`
    /// - `start <= end`
    /// - `end <= height`
    unsafe fn new_unchecked_owned(grid: T, index: impl Index1D) -> Self {
        let (col, range) = index.col_unchecked(grid.size());

        Self {
            grid,
            col,
            range,
            phantom: PhantomData,
        }
    }
}

/// @see [`ColRef`](crate::ColRef).
impl<'a, I, T> ColRef<'a, I, T>
where
    &'a T: Grid<&'a I>,
{
    /// Returns a [`ColRef`](crate::ColRef), or `None` if out of bounds.
    pub fn new(grid: &'a T, index: impl Index1D) -> Option<Self> {
        Col::new_owned(grid, index)
    }

    /// Returns a [`ColRef`](crate::ColRef), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `col < width`
    /// - `start <= end`
    /// - `end <= height`
    pub unsafe fn new_unchecked(grid: &'a T, index: impl Index1D) -> Self {
        Col::new_unchecked_owned(grid, index)
    }
}

/// @see [`ColMut`](crate::ColMut).
impl<'a, I, T> ColMut<'a, I, T>
where
    &'a mut T: Grid<&'a mut I>,
{
    /// Returns a [`ColMut`](crate::ColMut), or `None` if out of bounds.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::item_unchecked`](crate::Grid::item_unchecked) returns
    ///   non-overlapping references
    pub unsafe fn new(grid: &'a mut T, index: impl Index1D) -> Option<Self> {
        Col::new_owned(grid, index)
    }

    /// Returns a [`ColMut`](crate::ColMut), without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - [`Grid::item_unchecked`](crate::Grid::item_unchecked) returns
    ///   non-overlapping references
    /// - `col < width`
    /// - `start <= end`
    /// - `end <= height`
    pub unsafe fn new_unchecked(grid: &'a mut T, index: impl Index1D) -> Self {
        Col::new_unchecked_owned(grid, index)
    }
}

impl<'a, I, T> Iterator for ColRef<'a, I, T>
where
    &'a T: Grid<&'a I>,
{
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.range;

        if start < end {
            self.range.start += 1;
            let point = Point {
                x: self.col,
                y: start,
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

impl<'a, I, T> Iterator for ColMut<'a, I, T>
where
    &'a mut T: Grid<&'a mut I>,
{
    type Item = &'a mut I;

    fn next(&mut self) -> Option<Self::Item> {
        let std::ops::Range { start, end } = self.range;

        if start < end {
            self.range.start += 1;
            let point = Point {
                x: self.col,
                y: start,
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

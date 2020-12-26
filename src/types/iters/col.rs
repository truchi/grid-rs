use crate::*;
use std::ops::Range;

/// Column iterator helper.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ColHelper<'a, T: ?Sized> {
    grid:  &'a T,
    col:   usize,
    range: Range<usize>,
}

impl<'a, T: IGrid<'a> + ?Sized> ColHelper<'a, T> {
    /// Returns a [`ColHelper`](crate::ColHelper), or `None` if `col >= width`.
    pub fn new(grid: &'a T, index: impl Index1D) -> Option<Self> {
        let (width, height) = grid.size().into();
        let (col, range) = index.checked(width, height)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(col < width);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= height);
        Some(unsafe { Self::new_unchecked(grid, (col, range)) })
    }

    /// Returns a [`ColHelper`](crate::ColHelper) without bounds checking.
    ///
    /// See [`IGrid::col_unchecked`](crate::IGrid::col_unchecked) for safety.
    pub unsafe fn new_unchecked(grid: &'a T, index: impl Index1D) -> Self {
        let (col, range) = index.unchecked(grid.size().height);

        Self { grid, col, range }
    }
}

impl<'a, T: IGrid<'a>> Iterator for ColHelper<'a, T> {
    type Item = &'a T::Cell;

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
            Some(unsafe { self.grid.cell_unchecked(point) })
        } else {
            None
        }
    }
}

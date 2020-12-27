use crate::*;
use std::ops::Range;

/// Row iterator helper.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RowHelper<'a, T: ?Sized> {
    grid:  &'a T,
    row:   usize,
    range: Range<usize>,
}

impl<'a, T: GridRef<'a> + ?Sized> RowHelper<'a, T> {
    /// Returns a [`RowHelper`](crate::RowHelper), or `None` if `row >= height`.
    pub fn new(grid: &'a T, index: impl Index1D) -> Option<Self> {
        let (width, height) = grid.size().into();
        let (row, range) = index.checked(height, width)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(row < height);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= width);
        Some(unsafe { Self::new_unchecked(grid, (row, range)) })
    }

    /// Returns a [`RowHelper`](crate::RowHelper) without bounds checking.
    ///
    /// See [`Grid::row_unchecked`](crate::Grid::row_unchecked) for safety.
    pub unsafe fn new_unchecked(grid: &'a T, index: impl Index1D) -> Self {
        let (row, range) = index.unchecked(grid.size().width);

        Self { grid, row, range }
    }
}

impl<'a, T: GridRef<'a>> Iterator for RowHelper<'a, T> {
    type Item = &'a T::Cell;

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
            Some(unsafe { self.grid.cell_unchecked(point) })
        } else {
            None
        }
    }
}

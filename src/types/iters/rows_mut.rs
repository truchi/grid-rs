use crate::*;
use std::ops::Range;

/// Rows iterator helper.
#[derive(Eq, PartialEq, Debug)]
pub struct RowsMutHelper<'a, T: ?Sized> {
    grid:  &'a mut T,
    index: Point<Range<usize>>,
}

impl<'a, T: Grid<'a> + ?Sized> RowsMutHelper<'a, T> {
    /// ### Safety
    /// callers **MUST** ensure:
    /// - grid is valid
    /// - row_mut_unchecked returns valid, non-overlapping references
    pub fn new(grid: &'a mut T, index: impl Index2D) -> Option<Self> {
        let (width, height) = grid.size().into();
        let Point { x, y } = index.checked(grid.size())?;

        // SAFETY: Index2D::checked guaranties:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { Self::new_unchecked(grid, (x, y)) })
    }

    /// ### Safety
    /// callers **MUST** ensure:
    /// - grid is valid
    /// - row_mut_unchecked returns valid, non-overlapping references
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    pub unsafe fn new_unchecked(grid: &'a mut T, index: impl Index2D) -> Self {
        let index = index.unchecked(grid.size());

        Self { grid, index }
    }
}

impl<'a, T: GridMut<'a>> Iterator for RowsMutHelper<'a, T> {
    type Item = T::RowMut;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.index.y;

        if start < end {
            self.index.y.start += 1;

            // SAFETY:
            // row_mut_unchecked returns valid, non-overlapping references.
            // Then, it is safe to extend iterator's items lifetime
            let grid = unsafe { std::mem::transmute::<&mut T, &mut T>(self.grid) };

            // SAFETY: constructors guaranty this is safe
            let it = unsafe { grid.row_mut_unchecked((start, self.index.x.clone())) };

            Some(it)
        } else {
            None
        }
    }
}

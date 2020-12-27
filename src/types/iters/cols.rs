use crate::*;
use std::ops::Range;

/// Rows iterator helper.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ColsHelper<'a, T: ?Sized> {
    grid:  &'a T,
    index: Point<Range<usize>>,
}

impl<'a, T: Grid<'a> + ?Sized> ColsHelper<'a, T> {
    pub fn new(grid: &'a T, index: impl Index2D) -> Option<Self> {
        let (width, height) = grid.size().into();
        let Point { x, y } = index.checked(grid.size())?;

        // SAFETY: Index2D::checked guaranties:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { Self::new_unchecked(grid, (x, y)) })
    }

    pub unsafe fn new_unchecked(grid: &'a T, index: impl Index2D) -> Self {
        Self {
            grid,
            index: index.unchecked(grid.size()),
        }
    }
}

impl<'a, T: Grid<'a>> Iterator for ColsHelper<'a, T> {
    type Item = T::Col;

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

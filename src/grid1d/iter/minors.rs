use super::*;
use std::ops::Range;

pub struct Minors<'a, M, I, T> {
    grid:  &'a Grid1D<M, I, T>,
    index: Point<Range<usize>>,
}

impl<'a, M, I, T> Minors<'a, M, I, T> {
    pub unsafe fn new(grid: &'a Grid1D<M, I, T>, index: Point<Range<usize>>) -> Self {
        Self { grid, index }
    }
}

impl<'a, I, T: AsRef<[I]>> Iterator for Minors<'a, RowMajor, I, T> {
    type Item = Minor<'a, RowMajor, I, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = (self.index.x.next()?, self.index.y.clone());

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe { self.grid.col_unchecked(index) })
    }
}

impl<'a, I, T: AsRef<[I]>> Iterator for Minors<'a, ColMajor, I, T> {
    type Item = Minor<'a, ColMajor, I, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = (self.index.y.next()?, self.index.x.clone());

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe { self.grid.row_unchecked(index) })
    }
}

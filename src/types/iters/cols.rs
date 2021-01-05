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

impl<I, T: Grid<I>> Cols<I, T> {
    pub fn new(grid: T, index: impl Index2D) -> Option<Self> {
        let (width, height) = grid.size().into();
        let Point { x, y } = index.checked(grid.size())?;

        // SAFETY: Index2D::checked guaranties:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { Self::new_unchecked(grid, (x, y)) })
    }

    pub unsafe fn new_unchecked(grid: T, index: impl Index2D) -> Self {
        let index = index.unchecked(grid.size());

        Self {
            grid,
            index,
            phantom: PhantomData,
        }
    }
}

/*
impl<I, T: Grid<I>> Iterator for Cols<I, T> {
    type Item = <T as Grid<I>>::Col;

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
*/

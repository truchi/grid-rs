use crate::*;
use std::ops::Range;

#[derive(Debug)]
pub enum ColMajor {}

impl ColMajor {
    pub fn cell(size: Size<usize>, cell: Point<usize>) -> Option<usize> {
        if cell < size {
            Some(Self::cell_unchecked(size, cell))
        } else {
            None
        }
    }

    pub fn cell_unchecked(size: Size<usize>, cell: Point<usize>) -> usize {
        cell.x * size.height + cell.y
    }

    pub fn col(size: Size<usize>, index: impl Index1D) -> Option<Range<usize>> {
        let (width, height) = size.into();
        let (col, range) = index.checked((width, height))?;

        if col < width {
            Some(Self::col_unchecked(size, (col, range)))
        } else {
            None
        }
    }

    pub fn col_unchecked(size: Size<usize>, index: impl Index1D) -> Range<usize> {
        let (col, range) = index.unchecked(size.height);

        let start = Self::cell_unchecked(size, Point {
            x: col,
            y: range.start,
        });

        Range {
            start,
            end: start + range.end,
        }
    }
}

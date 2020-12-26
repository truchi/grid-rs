use crate::*;
use std::ops::Range;

#[derive(Debug)]
pub enum RowMajor {}

impl RowMajor {
    pub fn cell(size: Size<usize>, cell: Point<usize>) -> Option<usize> {
        if cell < size {
            Some(Self::cell_unchecked(size, cell))
        } else {
            None
        }
    }

    pub fn cell_unchecked(size: Size<usize>, cell: Point<usize>) -> usize {
        cell.y * size.width + cell.x
    }

    pub fn row(size: Size<usize>, index: impl Index1D) -> Option<Range<usize>> {
        let (width, height) = size.into();
        let (row, range) = index.checked((height, width))?;

        if row < height {
            Some(Self::row_unchecked(size, (row, range)))
        } else {
            None
        }
    }

    pub fn row_unchecked(size: Size<usize>, index: impl Index1D) -> Range<usize> {
        let (row, range) = index.unchecked(size.width);

        let start = Self::cell_unchecked(size, Point {
            x: range.start,
            y: row,
        });

        Range {
            start,
            end: start + range.end,
        }
    }
}

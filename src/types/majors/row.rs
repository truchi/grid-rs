use crate::*;
use std::ops::Range;

/// A marker type for [`RowMajor`](crate::RowMajor) grids.
///
/// This type cannot be instanciated. It serves as a marker for grid whose
/// collection is storing its element in a row-major fashion (row by row).
#[derive(Debug)]
pub enum RowMajor {}

/// ### Methods
impl RowMajor {
    /// Returns the index of the element at `point` in a row-major grid of
    /// size `size` if `point < size`, `None` otherwise.
    pub fn cell(size: Size<usize>, point: Point<usize>) -> Option<usize> {
        if point < size {
            Some(Self::cell_unchecked(size, point))
        } else {
            None
        }
    }

    /// Returns the index of the element at `point` in a row-major grid of
    /// size `size`.
    ///
    /// Does **not** check if `point < size`.
    pub fn cell_unchecked(size: Size<usize>, point: Point<usize>) -> usize {
        point.y * size.width + point.x
    }

    /// Returns the range of the row `index` in a row-major grid of size `size`
    /// if `index < size`, `None` otherwise.
    pub fn row(size: Size<usize>, index: impl Index1D) -> Option<Range<usize>> {
        let (width, height) = size.into();
        let (row, range) = index.checked(height, width)?;

        if row < height {
            Some(Self::row_unchecked(size, (row, range)))
        } else {
            None
        }
    }

    /// Returns the range of the row `index` in a row-major grid of size `size`.
    ///
    /// Does **not** check if `index < size`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cell() {
        let size = (3, 2).into();

        // It returns None when out of bounds
        assert_eq!(RowMajor::cell(size, (3, 1).into()), None);
        assert_eq!(RowMajor::cell(size, (2, 2).into()), None);
        assert_eq!(RowMajor::cell(size, (3, 2).into()), None);
        assert_eq!(RowMajor::cell(size, (4, 3).into()), None);
    }

    #[test]
    fn cell_unchecked() {
        let size = (3, 2).into();

        // It returns the correct index
        assert_eq!(RowMajor::cell_unchecked(size, (0, 0).into()), 0);
        assert_eq!(RowMajor::cell_unchecked(size, (1, 0).into()), 1);
        assert_eq!(RowMajor::cell_unchecked(size, (2, 0).into()), 2);
        assert_eq!(RowMajor::cell_unchecked(size, (0, 1).into()), 3);
        assert_eq!(RowMajor::cell_unchecked(size, (1, 1).into()), 4);
        assert_eq!(RowMajor::cell_unchecked(size, (2, 1).into()), 5);
    }

    #[test]
    fn row() {
        let size = (3, 2).into();

        // It returns None when out of bounds
        assert_eq!(RowMajor::row(size, 2), None);
        assert_eq!(RowMajor::row(size, 3), None);
    }

    #[test]
    fn row_unchecked() {
        let size = (3, 2).into();

        // It returns the correct range
        assert_eq!(RowMajor::row_unchecked(size, 0), 0..3);
        assert_eq!(RowMajor::row_unchecked(size, 1), 3..6);
    }
}

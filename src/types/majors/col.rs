use crate::*;
use std::ops::Range;

/// A marker type for [`ColMajor`](crate::ColMajor) grids.
///
/// This type cannot be instanciated. It serves as a marker for grid whose
/// collection is storing its element in a column-major fashion (column by
/// column).
#[derive(Debug)]
pub enum ColMajor {}

/// ### Methods
impl ColMajor {
    /// Returns the index of the element at `point` in a column-major grid of
    /// size `size` if `point < size`, `None` otherwise.
    pub fn cell(size: Size<usize>, point: Point<usize>) -> Option<usize> {
        if point < size {
            Some(Self::cell_unchecked(size, point))
        } else {
            None
        }
    }

    /// Returns the index of the element at `point` in a column-major grid of
    /// size `size`.
    ///
    /// Does **not** check if `point < size`.
    pub fn cell_unchecked(size: Size<usize>, point: Point<usize>) -> usize {
        point.x * size.height + point.y
    }

    /// Returns the range of the col `index` in a column-major grid of size
    /// `size` if `index < size`, `None` otherwise.
    pub fn col(size: Size<usize>, index: impl Index1D) -> Option<Range<usize>> {
        let (width, height) = size.into();
        let (col, range) = index.checked(width, height)?;

        if col < width {
            Some(Self::col_unchecked(size, (col, range)))
        } else {
            None
        }
    }

    /// Returns the range of the column `index` in a column-major grid of size
    /// `size`.
    ///
    /// Does **not** check if `index < size`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cell() {
        let size = (3, 2).into();

        // It returns None when out of bounds
        assert_eq!(ColMajor::cell(size, (3, 1).into()), None);
        assert_eq!(ColMajor::cell(size, (2, 2).into()), None);
        assert_eq!(ColMajor::cell(size, (3, 2).into()), None);
        assert_eq!(ColMajor::cell(size, (4, 3).into()), None);
    }

    #[test]
    fn cell_unchecked() {
        let size = (3, 2).into();

        // It returns the correct index
        assert_eq!(ColMajor::cell_unchecked(size, (0, 0).into()), 0);
        assert_eq!(ColMajor::cell_unchecked(size, (0, 1).into()), 1);
        assert_eq!(ColMajor::cell_unchecked(size, (1, 0).into()), 2);
        assert_eq!(ColMajor::cell_unchecked(size, (1, 1).into()), 3);
        assert_eq!(ColMajor::cell_unchecked(size, (2, 0).into()), 4);
        assert_eq!(ColMajor::cell_unchecked(size, (2, 1).into()), 5);
    }

    #[test]
    fn col() {
        let size = (3, 2).into();

        // It returns None when out of bounds
        assert_eq!(ColMajor::col(size, 3), None);
        assert_eq!(ColMajor::col(size, 4), None);
    }

    #[test]
    fn col_unchecked() {
        let size = (3, 2).into();

        // It returns the correct range
        assert_eq!(ColMajor::col_unchecked(size, 0), 0..2);
        assert_eq!(ColMajor::col_unchecked(size, 1), 2..4);
        assert_eq!(ColMajor::col_unchecked(size, 2), 4..6);
    }
}

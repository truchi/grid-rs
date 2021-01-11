use crate::*;
use std::ops::{Range, RangeBounds};

/// Indexing into a column or a row of a grid, with optional slicing.
pub trait Index1D: Sized {
    /// Returns the index **with** bounds checking.
    fn checked(self, max_index: usize, max_end: usize) -> Option<(usize, Range<usize>)>;

    /// Returns the index **without** bounds checking.
    fn unchecked(self, max_end: usize) -> (usize, Range<usize>);

    fn row(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(size.height, size.width)
    }

    fn row_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(size.width)
    }

    fn col(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(size.width, size.height)
    }

    fn col_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(size.height)
    }
}

impl Index1D for usize {
    fn checked(self, max_index: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
        if self < max_index {
            Some(self.unchecked(max_end))
        } else {
            None
        }
    }

    fn unchecked(self, max_end: usize) -> (usize, Range<usize>) {
        (self, 0..max_end)
    }
}

impl<T: RangeBounds<usize>> Index1D for (usize, T) {
    fn checked(self, max_index: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
        let (i, range) = self;

        if i < max_index {
            Some((i, ToRange::checked(range, max_end)?))
        } else {
            None
        }
    }

    fn unchecked(self, max_end: usize) -> (usize, Range<usize>) {
        (self.0, ToRange::unchecked(self.1, max_end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn checked() {
        // It returns None when i >= len
        assert_eq!(20.checked(10, 12), None);
        assert_eq!((20, 0..5).checked(10, 12), None);
    }

    #[test]
    fn unchecked() {
        // It does not bounds check
        assert_eq!(10.unchecked(2), (10, 0..2));
        assert_eq!((10, 100..1000).unchecked(2), (10, 100..1000));
    }
}

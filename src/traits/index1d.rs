use crate::*;
use std::ops::{Range, RangeBounds};

/// Indexing into a column or a row of a grid, with optional slicing.
pub trait Index1D {
    /// Returns the index **without** bounds checking.
    fn unchecked(self, max_index: usize) -> (usize, Range<usize>);

    /// Returns the index **with** bounds checking.
    fn checked(self, max_index: usize, max_end: usize) -> Option<(usize, Range<usize>)>;
}

impl Index1D for usize {
    fn unchecked(self, max_index: usize) -> (usize, Range<usize>) {
        (self, 0..max_index)
    }

    fn checked(self, max_index: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
        if self < max_index {
            Some((self, 0..max_end))
        } else {
            None
        }
    }
}

impl<T: RangeBounds<usize>> Index1D for (usize, T) {
    fn unchecked(self, max_index: usize) -> (usize, Range<usize>) {
        (self.0, ToRange::unchecked(self.1, max_index))
    }

    fn checked(self, max_index: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
        let (i, range) = self;

        if i < max_index {
            Some((i, ToRange::checked(range, max_end)?))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn unchecked() {
        // It does not bounds check;
        assert_eq!((10, 100..1000).unchecked(2), (10, 100..1000));
    }

    #[test]
    fn checked() {
        // It returns None when i >= len
        assert_eq!((20, 0..5).checked(10, 12), None);
    }
}

use crate::*;
use std::ops::{Range, RangeBounds};

/// Indexing into a column or a row of a grid.
pub trait Index1D {
    /// Returns the index **without** bounds checking.
    fn unchecked(self, max: usize) -> (usize, Range<usize>);

    /// Returns the index **with** bounds checking.
    fn checked(self, max: (usize, usize)) -> Option<(usize, Range<usize>)>;
}

impl Index1D for usize {
    fn unchecked(self, max: usize) -> (usize, Range<usize>) {
        (self, 0..max)
    }

    fn checked(self, max: (usize, usize)) -> Option<(usize, Range<usize>)> {
        if self < max.0 {
            Some((self, 0..max.1))
        } else {
            None
        }
    }
}

impl<T: RangeBounds<usize>> Index1D for (usize, T) {
    fn unchecked(self, max: usize) -> (usize, Range<usize>) {
        (self.0, ToRange::unchecked(self.1, max))
    }

    fn checked(self, max: (usize, usize)) -> Option<(usize, Range<usize>)> {
        let (i, range) = self;

        if i < max.0 {
            Some((i, ToRange::checked(range, max.1)?))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::ops::Bound;

    #[test]
    fn unchecked() {
        // It does not bounds check;
        assert_eq!((10, 100..1000).unchecked(2), (10, 100..1000));
    }

    #[test]
    fn checked() {
        // It returns None when i >= len
        assert_eq!((20, 0..5).checked((10, 10)), None);
    }
}

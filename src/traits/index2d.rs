use crate::*;
use std::ops::{Range, RangeBounds};

/// Indexing into a rectangle inside a grid, with optional cropping.
pub trait Index2D {
    /// Returns the index **with** bounds checking.
    fn checked(self, size: Size) -> Option<Point<Range<usize>>>;

    /// Returns the index **without** bounds checking.
    fn unchecked(self, size: Size) -> Point<Range<usize>>;
}

impl Index2D for () {
    fn checked(self, size: Size) -> Option<Point<Range<usize>>> {
        Some(self.unchecked(size))
    }

    fn unchecked(self, size: Size) -> Point<Range<usize>> {
        Point {
            x: 0..size.width,
            y: 0..size.height,
        }
    }
}

impl<X: RangeBounds<usize>, Y: RangeBounds<usize>> Index2D for (X, Y) {
    fn checked(self, size: Size) -> Option<Point<Range<usize>>> {
        Some(Point {
            x: ToRange::checked(self.0, size.width)?,
            y: ToRange::checked(self.1, size.height)?,
        })
    }

    fn unchecked(self, size: Size) -> Point<Range<usize>> {
        Point {
            x: ToRange::unchecked(self.0, size.width),
            y: ToRange::unchecked(self.1, size.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn checked() {
        // It returns None when out of bounds
        assert_eq!((10..100, 10..100).checked((2, 3).into()), None);
    }

    #[test]
    fn unchecked() {
        // It does not bounds check.
        assert_eq!(
            (10..100, 10..100).unchecked((2, 3).into()),
            Point::from((10..100, 10..100))
        );
    }
}

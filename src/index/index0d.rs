use crate::*;

/// Indexes for [`Grid::item`](Grid::item).
///
/// The underlying type to index an item is [`Point`](Point).
///
/// Anything that `Into<Point>` is an [`Index0D`](Index0D).
pub trait Index0D {
    /// Returns the index as a [`Point`](Point), without bounds checking.
    fn unchecked(self) -> Point;

    /// Returns the index as a [`Point`](Point), or
    /// [`None`](std::option::Option::None) if out of bounds.
    ///
    /// When `Some`, guaranties:
    /// - `point.x < size.x`
    /// - `point.y < size.y`
    fn checked(self, size: Size) -> Option<Point>;
}

impl<T: Into<Point>> Index0D for T {
    fn unchecked(self) -> Point {
        self.into()
    }

    fn checked(self, size: Size) -> Option<Point> {
        let point = self.into();

        if point < size {
            Some(point)
        } else {
            None
        }
    }
}

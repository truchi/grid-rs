use crate::*;
use std::ops::RangeBounds;

/// Indexes for [`GridCols::cols`](GridCols::cols) /
/// [`GridRows::rows`](GridRows::rows) /
/// [`GridItems::items`](GridItems::items).
///
/// The underlying type to index columns/rows/items is [`Rect`](Rect)
/// (`Coord<Range<usize>>`).
///
/// [`RangeFull`](std::ops::RangeFull), `Coord<X: RangeBounds<usize>, Y:
/// RangeBounds<usize>>` and `(X: RangeBounds<usize>, Y: RangeBounds<usize>)`
/// are [`Index2D`](Index2D)s.
pub trait Index2D {
    /// Returns the index as a [`Rect`](Rect), without bounds checking.
    ///
    /// [`Unbounded`](std::ops::Bound::Unbounded) start/end bounds will
    /// transform into `0`/`size`.  
    /// [`Excluded`](std::ops::Bound::Excluded) start bounds and
    /// [`Included`](std::ops::Bound::Included) end bounds may overflow.
    fn unchecked(self, size: Size) -> Rect;

    /// Returns the index as [`Rect`](Rect), or
    /// [`None`](std::option::Option::None) if out of bounds.
    ///
    /// [`Excluded`](std::ops::Bound::Excluded) start bounds and
    /// [`Included`](std::ops::Bound::Included) end bounds saturate.
    ///
    /// When `Some`, guaranties on both axis:
    /// - `start <= end`
    /// - `end <= len`
    fn checked(self, size: Size) -> Option<Rect>;
}

impl Index2D for std::ops::RangeFull {
    fn unchecked(self, size: Size) -> Rect {
        Point {
            x: 0..size.x,
            y: 0..size.y,
        }
    }

    fn checked(self, size: Size) -> Option<Rect> {
        Some(Index2D::unchecked(self, size))
    }
}

impl<X: RangeBounds<usize>, Y: RangeBounds<usize>> Index2D for Coord<X, Y> {
    fn unchecked(self, size: Size) -> Rect {
        (self.x, self.y).unchecked(size)
    }

    fn checked(self, size: Size) -> Option<Rect> {
        (self.x, self.y).checked(size)
    }
}

impl<X: RangeBounds<usize>, Y: RangeBounds<usize>> Index2D for (X, Y) {
    fn unchecked(self, size: Size) -> Rect {
        Point {
            x: ToRange::unchecked(self.0, size.x),
            y: ToRange::unchecked(self.1, size.y),
        }
    }

    fn checked(self, size: Size) -> Option<Rect> {
        Some(Point {
            x: ToRange::checked(self.0, size.x)?,
            y: ToRange::checked(self.1, size.y)?,
        })
    }
}

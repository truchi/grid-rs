use crate::*;
use std::ops::{Range, RangeBounds};

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

/// Indexes for [`GridCol::col`](GridCol::col) /
/// [`GridRow::row`](GridRow::row).
///
/// The underlying type to index a column/row is `(usize, Range<usize>)`, with:
/// - [`usize`](usize): the index of the column/row,
/// - [`Range<usize>`](std::ops::Range): the range of items in that column/row.
///
/// `usize` (implied [`RangeFull`](std::ops::RangeFull)) and `(usize, T:
/// RangeBounds<usize>)` are [`Index1D`](Index1D)s.
pub trait Index1D: Sized {
    /// Returns the index as `(usize, Range<usize>)`, without bounds checking.
    ///
    /// [`Unbounded`](std::ops::Bound::Unbounded) start/end bounds will
    /// transform into `0`/`max_end`.  
    /// [`Excluded`](std::ops::Bound::Excluded) start bounds and
    /// [`Included`](std::ops::Bound::Included) end bounds may overflow.
    fn unchecked(self, max_end: usize) -> (usize, Range<usize>);

    /// Returns the index as `(usize, Range<usize>)`, or
    /// [`None`](std::option::Option::None) if out of bounds.
    ///
    /// [`Excluded`](std::ops::Bound::Excluded) start bounds and
    /// [`Included`](std::ops::Bound::Included) end bounds saturate.
    ///
    /// When `Some`, guaranties:
    /// - `usize < max_i`
    /// - `range.start <= range.end`
    /// - `range.end <= end`
    fn checked(self, max_i: usize, max_end: usize) -> Option<(usize, Range<usize>)>;

    /// Returns the column index as `(usize, Range<usize>)`, without bounds
    /// checking.
    ///
    /// [`Unbounded`](std::ops::Bound::Unbounded) start/end bounds will
    /// transform into `0`/`max_end`.  
    /// [`Excluded`](std::ops::Bound::Excluded) start bounds and
    /// [`Included`](std::ops::Bound::Included) end bounds may overflow.
    fn col_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(size.y)
    }

    /// Returns the column index as `(usize, Range<usize>)`, or
    /// [`None`](std::option::Option::None) if out of bounds.
    ///
    /// [`Excluded`](std::ops::Bound::Excluded) start bounds and
    /// [`Included`](std::ops::Bound::Included) end bounds saturate.
    ///
    /// When `Some`, guaranties:
    /// - `usize < size.x`
    /// - `range.start <= range.end`
    /// - `range.end <= size.y`
    /// - `range.end <= usize::MAX` (saturates end bound)
    fn col(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(size.x, size.y)
    }

    /// Returns the row index as `(usize, Range<usize>)`, without bounds
    /// checking.
    ///
    /// [`Unbounded`](std::ops::Bound::Unbounded) start/end bounds will
    /// transform into `0`/`max_end`.  
    /// [`Excluded`](std::ops::Bound::Excluded) start bounds and
    /// [`Included`](std::ops::Bound::Included) end bounds may overflow.
    fn row_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(size.x)
    }

    /// Returns the row index as `(usize, Range<usize>)`, or
    /// [`None`](std::option::Option::None) if out of bounds.
    ///
    /// [`Excluded`](std::ops::Bound::Excluded) start bounds and
    /// [`Included`](std::ops::Bound::Included) end bounds saturate.
    ///
    /// When `Some`, guaranties:
    /// - `usize < size.y`
    /// - `range.start <= range.end`
    /// - `range.end <= size.x`
    fn row(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(size.y, size.x)
    }
}

impl Index1D for usize {
    fn unchecked(self, max_end: usize) -> (usize, Range<usize>) {
        (self, 0..max_end)
    }

    fn checked(self, max_i: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
        if self < max_i {
            Some(self.unchecked(max_end))
        } else {
            None
        }
    }
}

impl<T: RangeBounds<usize>> Index1D for (usize, T) {
    fn unchecked(self, max_end: usize) -> (usize, Range<usize>) {
        (self.0, ToRange::unchecked(self.1, max_end))
    }

    fn checked(self, max_i: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
        let (i, range) = self;

        if i < max_i {
            Some((i, ToRange::checked(range, max_end)?))
        } else {
            None
        }
    }
}

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

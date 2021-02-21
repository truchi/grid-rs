use crate::*;
use std::ops::{Range, RangeBounds};

/// Indexes for [`Grid::item`](Grid::item).
///
/// The underlying type to index an item is [`Point`](Point).
///
/// Anything that `Into<Point>` is an `Index0D`.
pub trait Index0D {
    /// Returns the index as a [`Point`](Point), without bounds checking.
    ///
    /// ### Examples
    ///
    /// ```
    /// # use grid::*;
    /// assert!((0, 0).unchecked() == Point { x: 0, y: 0 });
    /// ```
    fn unchecked(self) -> Point;

    /// Returns the index as a [`Point`](Point), or `None` if out of
    /// `size`.
    ///
    /// When `Some`, guaranties:
    /// - `point.x < size.x`
    /// - `point.y < size.y`
    ///
    /// ### Examples
    ///
    /// ```
    /// # use grid::*;
    /// let size = Size { x: 5, y: 5 };
    ///
    /// assert!((0, 0).checked(size) == Some(Point { x: 0, y: 0 }));
    /// assert!((4, 4).checked(size) == Some(Point { x: 4, y: 4 }));
    /// assert!((1, 5).checked(size) == None);
    /// ```
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
/// - `usize`: the index of the column/row,
/// - `Range<usize>`: the range in that column/row.
///
/// Both `usize` (implied `RangeFull`) and `(usize, T: RangeBounds<usize>)` are
/// `Index1D`s.
pub trait Index1D {
    /// Returns the index as `(usize, Range<usize>)`, without bounds checking.
    ///
    /// ### Examples
    ///
    /// ```
    /// // TODO
    /// ```
    fn unchecked<M: Major>(self, size: M) -> (usize, Range<usize>);

    /// Returns the index as `(usize, Range<usize>)`, or `None` if out of
    /// `size`.
    ///
    /// When `Some`, guaranties:
    /// - `usize < size.minor()`
    /// - `range.start <= range.end`
    /// - `range.end <= size.major()`
    /// - (`range.end <= usize::MAX`)
    fn checked<M: Major>(self, size: M) -> Option<(usize, Range<usize>)>;
}

impl Index1D for usize {
    fn checked<M: Major>(self, size: M) -> Option<(usize, Range<usize>)> {
        if self < size.minor() {
            Some(self.unchecked(size))
        } else {
            None
        }
    }

    fn unchecked<M: Major>(self, size: M) -> (usize, Range<usize>) {
        (self, 0..size.major())
    }
}

impl<T: RangeBounds<usize>> Index1D for (usize, T) {
    fn checked<M: Major>(self, size: M) -> Option<(usize, Range<usize>)> {
        let (i, range) = self;

        if i < size.minor() {
            Some((i, ToRange::checked(range, size.major())?))
        } else {
            None
        }
    }

    fn unchecked<M: Major>(self, size: M) -> (usize, Range<usize>) {
        (self.0, ToRange::unchecked(self.1, size.major()))
    }
}

/// Indexes for [`GridCols::cols`](GridCols::cols) /
/// [`GridRows::rows`](GridRows::rows) /
/// [`GridItems::items`](GridItems::items).
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
            x: 0..size.x,
            y: 0..size.y,
        }
    }
}

impl<X: RangeBounds<usize>, Y: RangeBounds<usize>> Index2D for Point<X, Y> {
    fn checked(self, size: Size) -> Option<Point<Range<usize>>> {
        (self.x, self.y).checked(size)
    }

    fn unchecked(self, size: Size) -> Point<Range<usize>> {
        (self.x, self.y).unchecked(size)
    }
}

impl<X: RangeBounds<usize>, Y: RangeBounds<usize>> Index2D for (X, Y) {
    fn checked(self, size: Size) -> Option<Point<Range<usize>>> {
        Some(Point {
            x: ToRange::checked(self.0, size.x)?,
            y: ToRange::checked(self.1, size.y)?,
        })
    }

    fn unchecked(self, size: Size) -> Point<Range<usize>> {
        Point {
            x: ToRange::unchecked(self.0, size.x),
            y: ToRange::unchecked(self.1, size.y),
        }
    }
}

use crate::*;
use std::ops::{Range, RangeBounds};

pub trait Index0D {
    fn unchecked<S: Into<Size>>(self, size: S) -> Point;
    fn checked<S: Into<Size>>(self, size: S) -> Option<Point>;
}

impl Index0D for Point {
    fn unchecked<S: Into<Size>>(self, size: S) -> Point {
        self
    }

    fn checked<S: Into<Size>>(self, size: S) -> Option<Point> {
        if self < size.into() {
            Some(self)
        } else {
            None
        }
    }
}

pub trait Index1D: Sized {
    fn unchecked<M: Major>(self, size: M) -> (usize, Range<usize>);
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
            x: 0..size.x,
            y: 0..size.x,
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

pub(crate) mod flat {
    use crate::{Major, Point};
    use std::ops::Range;

    pub trait Index0D {
        fn index<M: Major>(self, size: M) -> usize;
    }

    impl Index0D for Point {
        fn index<M: Major>(self, size: M) -> usize {
            let point = M::from(self);

            point.minor() * size.major() + point.major()
        }
    }

    pub trait Index1D {
        fn index<M: Major>(self, size: M) -> Range<usize>;
    }

    impl Index1D for (usize, Range<usize>) {
        fn index<M: Major>(self, size: M) -> Range<usize> {
            let (i, Range { start, end }) = self;
            let point = M::new(start, i).into();
            let start = point.index(size);

            start..start + end
        }
    }
}

mod coord;
mod flat;
mod grid;
// mod index;
mod major;
mod range;

pub use coord::*;
pub use flat::*;
pub use grid::*;
// pub use index::*;
pub use major::*;
pub use range::*;

pub use index::*;
mod index {
    use crate::{Coord, Flat, Major, Point, Size, ToRange};
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

    pub trait Index1D {
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
}

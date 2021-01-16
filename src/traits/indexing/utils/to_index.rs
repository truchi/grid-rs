use super::*;

pub trait ToIndex {
    type Index;
    fn checked<M: Major>(self, size: M) -> Option<Self::Index>;
    fn unchecked<M: Major>(self, size: M) -> Self::Index;
}

impl ToIndex for Point {
    type Index = Point;

    fn checked<M: Major>(self, size: M) -> Option<Self::Index> {
        if self < Into::<Point>::into(size) {
            Some(self)
        } else {
            None
        }
    }

    fn unchecked<M: Major>(self, _: M) -> Self::Index {
        self
    }
}

impl ToIndex for usize {
    type Index = (usize, Range<usize>);

    fn checked<M: Major>(self, size: M) -> Option<Self::Index> {
        if self < size.minor() {
            Some(self.unchecked(size))
        } else {
            None
        }
    }

    fn unchecked<M: Major>(self, size: M) -> Self::Index {
        (self, 0..size.major())
    }
}

impl<T: RangeBounds<usize>> ToIndex for (usize, T) {
    type Index = (usize, Range<usize>);

    fn checked<M: Major>(self, size: M) -> Option<Self::Index> {
        let (i, range) = self;

        if i < size.minor() {
            Some((i, range.checked(size.major())?))
        } else {
            None
        }
    }

    fn unchecked<M: Major>(self, size: M) -> Self::Index {
        (self.0, self.1.unchecked(size.major()))
    }
}

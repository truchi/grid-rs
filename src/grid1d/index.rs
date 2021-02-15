use crate::*;
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

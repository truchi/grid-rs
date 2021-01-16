use super::*;

pub trait ToIndex1D<I> {
    type Index1D: SliceIndex<[I]>;

    fn index_1d<M: Major>(self, size: M) -> Self::Index1D;
}

impl<I> ToIndex1D<I> for Point {
    type Index1D = usize;

    fn index_1d<M: Major>(self, size: M) -> Self::Index1D {
        let point: M = self.into();
        point.minor() * size.major() + point.major()
    }
}

impl<I> ToIndex1D<I> for (usize, Range<usize>) {
    type Index1D = Range<usize>;

    fn index_1d<M: Major>(self, size: M) -> Self::Index1D {
        let (i, Range { start, end }) = self;
        let point: Point = M::new(start, i).into();
        let start = ToIndex1D::<I>::index_1d(point, size);

        start..start + end
    }
}

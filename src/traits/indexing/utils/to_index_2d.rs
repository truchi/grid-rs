use super::*;

pub trait ToIndex2D<I, U> {
    type Index2D: Major2D<I, U>;

    fn index_2d<M: Major>(self, size: M) -> Self::Index2D;
}

impl<I, U> ToIndex2D<I, U> for Point {
    type Index2D = MajorIndex<I, U>;

    fn index_2d<M: Major>(self, _: M) -> Self::Index2D {
        let point: M = self.into();
        MajorIndex::cast(point)
    }
}

impl<I, U> ToIndex2D<I, U> for (usize, Range<usize>) {
    type Index2D = MajorSlice<I, U>;

    fn index_2d<M: Major>(self, _: M) -> Self::Index2D {
        let (i, range) = self;
        MajorSlice::new(range, i)
    }
}

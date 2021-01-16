use std::slice::SliceIndex;

pub trait Major2D<I, U>: Clone {
    type Major: SliceIndex<[I]>;
    type Minor: SliceIndex<[U], Output = U>;

    fn major(self) -> Self::Major;
    fn minor(self) -> Self::Minor;
}

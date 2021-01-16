use super::*;

pub trait SliceIndex1D<I>: ToIndex1D<I> + Sized {
    unsafe fn get_1d<M: Major, T: AsRef<[I]>>(
        self,
        grid: &Grid1D<M, I, T>,
    ) -> &<Self::Index1D as SliceIndex<[I]>>::Output {
        let index = self.index_1d(grid.msize());
        grid.as_ref().get_unchecked(index)
    }

    unsafe fn get_mut_1d<M: Major, T: AsMut<[I]>>(
        self,
        grid: &mut Grid1D<M, I, T>,
    ) -> &mut <Self::Index1D as SliceIndex<[I]>>::Output
    where
        T: AsMut<[I]>,
    {
        let index = self.index_1d(grid.msize());
        grid.as_mut().get_unchecked_mut(index)
    }
}

pub trait SliceIndex2D<I, U>: ToIndex2D<I, U> + Sized {
    unsafe fn get_2d<M: Major, T: AsRef<[U]>>(
        self,
        grid: &Grid2D<M, I, T, U>,
    ) -> &<<Self::Index2D as Major2D<I, U>>::Major as SliceIndex<[I]>>::Output
    where
        U: AsRef<[I]>,
    {
        let index = self.index_2d(grid.msize());
        grid.as_ref()
            .get_unchecked(index.clone().minor())
            .as_ref()
            .get_unchecked(index.major())
    }

    unsafe fn get_mut_2d<M: Major, T: AsMut<[U]>>(
        self,
        grid: &mut Grid2D<M, I, T, U>,
    ) -> &mut <<Self::Index2D as Major2D<I, U>>::Major as SliceIndex<[I]>>::Output
    where
        U: AsMut<[I]>,
    {
        let index = self.index_2d(grid.msize());
        grid.as_mut()
            .get_unchecked_mut(index.clone().minor())
            .as_mut()
            .get_unchecked_mut(index.major())
    }
}

impl<I, T: ToIndex1D<I>> SliceIndex1D<I> for T {}
impl<I, U, T: ToIndex2D<I, U>> SliceIndex2D<I, U> for T {}

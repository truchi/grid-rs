mod utils;

use utils::*;

use crate::{Grid1D, Grid2D, Major, Point};
use std::ops::Range;

pub trait GridIndex<I>: GridSliceIndex<I, Output = I> {}
pub trait GridSlice<I>: GridSliceIndex<I, Output = [I]> {}
pub trait GridSliceIndex<I> {
    type Output: ?Sized;

    fn get_1d<M: Major, T: AsRef<[I]>>(self, grid: &Grid1D<M, I, T>) -> Option<&Self::Output>;

    fn get_mut_1d<M: Major, T: AsMut<[I]>>(
        self,
        grid: &mut Grid1D<M, I, T>,
    ) -> Option<&mut Self::Output>;

    unsafe fn get_unchecked_1d<M: Major, T: AsRef<[I]>>(
        self,
        grid: &Grid1D<M, I, T>,
    ) -> &Self::Output;

    unsafe fn get_mut_unchecked_1d<M: Major, T: AsMut<[I]>>(
        self,
        grid: &mut Grid1D<M, I, T>,
    ) -> &mut Self::Output;

    fn get_2d<M: Major, T: AsRef<[U]>, U: AsRef<[I]>>(
        self,
        grid: &Grid2D<M, I, T, U>,
    ) -> Option<&Self::Output>;

    fn get_mut_2d<M: Major, T: AsMut<[U]>, U: AsMut<[I]>>(
        self,
        grid: &mut Grid2D<M, I, T, U>,
    ) -> Option<&mut Self::Output>;

    unsafe fn get_unchecked_2d<M: Major, T: AsRef<[U]>, U: AsRef<[I]>>(
        self,
        grid: &Grid2D<M, I, T, U>,
    ) -> &Self::Output;

    unsafe fn get_mut_unchecked_2d<M: Major, T: AsMut<[U]>, U: AsMut<[I]>>(
        self,
        grid: &mut Grid2D<M, I, T, U>,
    ) -> &mut Self::Output;
}

// Implements GridSliceIndex for T: utils::ToIndex
macro_rules! grid_slice_index {
    ($($Self:ty => $Output:ty)*) => { $(
        impl<I> GridSliceIndex<I> for $Self {
            type Output = $Output;

            fn get_1d<M: Major, T: AsRef<[I]>>(self, grid: &Grid1D<M, I, T>) -> Option<&Self::Output> {
                Some(unsafe { SliceIndex1D::<I>::get_1d(self.checked(grid.msize())?, grid) })
            }

            fn get_mut_1d<M: Major, T: AsMut<[I]>>(
                self,
                grid: &mut Grid1D<M, I, T>,
            ) -> Option<&mut Self::Output> {
                Some(unsafe { SliceIndex1D::<I>::get_mut_1d(self.checked(grid.msize())?, grid) })
            }

            unsafe fn get_unchecked_1d<M: Major, T: AsRef<[I]>>(
                self,
                grid: &Grid1D<M, I, T>,
            ) -> &Self::Output {
                SliceIndex1D::<I>::get_1d(self.unchecked(grid.msize()), grid)
            }

            unsafe fn get_mut_unchecked_1d<M: Major, T: AsMut<[I]>>(
                self,
                grid: &mut Grid1D<M, I, T>,
            ) -> &mut Self::Output {
                SliceIndex1D::<I>::get_mut_1d(self.unchecked(grid.msize()), grid)
            }

            fn get_2d<M: Major, T: AsRef<[U]>, U: AsRef<[I]>>(
                self,
                grid: &Grid2D<M, I, T, U>,
            ) -> Option<&Self::Output> {
                Some(unsafe { SliceIndex2D::<I, U>::get_2d(self.checked(grid.msize())?, grid) })
            }

            fn get_mut_2d<M: Major, T: AsMut<[U]>, U: AsMut<[I]>>(
                self,
                grid: &mut Grid2D<M, I, T, U>,
            ) -> Option<&mut Self::Output> {
                Some(unsafe { SliceIndex2D::<I, U>::get_mut_2d(self.checked(grid.msize())?, grid) })
            }

            unsafe fn get_unchecked_2d<M: Major, T: AsRef<[U]>, U: AsRef<[I]>>(
                self,
                grid: &Grid2D<M, I, T, U>,
            ) -> &Self::Output {
                SliceIndex2D::<I, U>::get_2d(self.unchecked(grid.msize()), grid)
            }

            unsafe fn get_mut_unchecked_2d<M: Major, T: AsMut<[U]>, U: AsMut<[I]>>(
                self,
                grid: &mut Grid2D<M, I, T, U>,
            ) -> &mut Self::Output {
                SliceIndex2D::<I, U>::get_mut_2d(self.unchecked(grid.msize()), grid)
            }
        }
    )* };
}

grid_slice_index!(
    Point                 =>  I
    usize                 => [I]
    (usize, Range<usize>) => [I]
);

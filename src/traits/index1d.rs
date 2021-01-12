use crate::*;
use std::ops::{Range, RangeBounds};

/// Indexing into a column or a row of a grid, with optional slicing.
pub trait Index1D: Sized {
    fn checked<M: Major>(self, size: M) -> Option<(usize, Range<usize>)>;

    fn unchecked<M: Major>(self, size: M) -> (usize, Range<usize>);

    fn row(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(RowMajor::from(size))
    }

    fn row_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(RowMajor::from(size))
    }

    fn col(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(ColMajor::from(size))
    }

    fn col_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(ColMajor::from(size))
    }

    fn index_1d<M: Major>(self, size: M) -> Option<Range<usize>> {
        Some(index_1d(self.checked(size)?, size))
    }

    fn index_1d_unchecked<M: Major>(self, size: M) -> Range<usize> {
        index_1d(self.unchecked(size), size)
    }

    fn index_2d<M: Major>(self, size: M) -> Option<(usize, Range<usize>)> {
        self.checked(size)
    }

    fn index_2d_unchecked<M: Major>(self, size: M) -> (usize, Range<usize>) {
        self.unchecked(size)
    }

    fn get_1d<M: Major, I, T: AsRef<[I]>>(self, grid: &Grid1D<M, I, T>) -> Option<&[I]> {
        let index = self.checked(grid.msize())?;
        // SAFETY: index is checked
        Some(unsafe { get_1d(index, grid) })
    }

    unsafe fn get_1d_unchecked<M: Major, I, T: AsRef<[I]>>(self, grid: &Grid1D<M, I, T>) -> &[I] {
        get_1d(self.unchecked(grid.msize()), grid)
    }

    fn get_mut_1d<M: Major, I, T: AsMut<[I]>>(
        self,
        grid: &mut Grid1D<M, I, T>,
    ) -> Option<&mut [I]> {
        let index = self.checked(grid.msize())?;
        // SAFETY: index is checked
        Some(unsafe { get_mut_1d(index, grid) })
    }

    unsafe fn get_mut_1d_unchecked<M: Major, I, T: AsMut<[I]>>(
        self,
        grid: &mut Grid1D<M, I, T>,
    ) -> &mut [I] {
        get_mut_1d(self.unchecked(grid.msize()), grid)
    }

    fn get_2d<M: Major, I, T: AsRef<[U]>, U: AsRef<[I]>>(
        self,
        grid: &Grid2D<M, I, T, U>,
    ) -> Option<&[I]> {
        let index = self.checked(grid.msize())?;
        // SAFETY: index is checked
        Some(unsafe { get_2d(index, grid) })
    }

    unsafe fn get_2d_unchecked<M: Major, I, T: AsRef<[U]>, U: AsRef<[I]>>(
        self,
        grid: &Grid2D<M, I, T, U>,
    ) -> &[I] {
        get_2d(self.unchecked(grid.msize()), grid)
    }

    fn get_mut_2d<M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>>(
        self,
        grid: &mut Grid2D<M, I, T, U>,
    ) -> Option<&mut [I]> {
        let index = self.checked(grid.msize())?;
        // SAFETY: index is checked
        Some(unsafe { get_mut_2d(index, grid) })
    }

    unsafe fn get_mut_2d_unchecked<M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>>(
        self,
        grid: &mut Grid2D<M, I, T, U>,
    ) -> &mut [I] {
        get_mut_2d(self.unchecked(grid.msize()), grid)
    }
}

fn index_1d<M: Major>(index: (usize, Range<usize>), size: M) -> Range<usize> {
    let (i, Range { start, end }) = index;
    let point: Point = M::new(start, i).into();
    let start = point.index_1d_unchecked(size);

    start..start + end
}

unsafe fn get_1d<M: Major, I, T: AsRef<[I]>>(
    index: (usize, Range<usize>),
    grid: &Grid1D<M, I, T>,
) -> &[I] {
    let range = index_1d(index, grid.msize());

    grid.as_ref().get_unchecked(range)
}

unsafe fn get_mut_1d<M: Major, I, T: AsMut<[I]>>(
    index: (usize, Range<usize>),
    grid: &mut Grid1D<M, I, T>,
) -> &mut [I] {
    let range = index_1d(index, grid.msize());

    grid.as_mut().get_unchecked_mut(range)
}

unsafe fn get_2d<M: Major, I, T: AsRef<[U]>, U: AsRef<[I]>>(
    index: (usize, Range<usize>),
    grid: &Grid2D<M, I, T, U>,
) -> &[I] {
    let (i, range) = index;

    grid.as_ref().get_unchecked(i).as_ref().get_unchecked(range)
}

unsafe fn get_mut_2d<M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>>(
    index: (usize, Range<usize>),
    grid: &mut Grid2D<M, I, T, U>,
) -> &mut [I] {
    let (i, range) = index;

    grid.as_mut()
        .get_unchecked_mut(i)
        .as_mut()
        .get_unchecked_mut(range)
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

/*
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn checked() {
        // It returns None when i >= len
        assert_eq!(20.checked(10, 12), None);
        assert_eq!((20, 0..5).checked(10, 12), None);
    }

    #[test]
    fn unchecked() {
        // It does not bounds check
        assert_eq!(10.unchecked(2), (10, 0..2));
        assert_eq!((10, 100..1000).unchecked(2), (10, 100..1000));
    }
}
*/

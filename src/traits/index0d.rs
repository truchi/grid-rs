use crate::*;

pub trait Index0D: Sized {
    /// Returns the index **with** bounds checking.
    fn checked<M: Major>(self, size: M) -> Option<Point> {
        let point = self.unchecked();

        if point < Into::<Point>::into(size) {
            Some(point)
        } else {
            None
        }
    }

    /// Returns the index **without** bounds checking.
    fn unchecked(self) -> Point;

    fn index_1d<M: Major>(self, size: M) -> Option<usize> {
        Some(index_1d(self.checked(size)?, size))
    }

    fn index_1d_unchecked<M: Major>(self, size: M) -> usize {
        index_1d(self.unchecked(), size)
    }

    fn index_2d<M: Major>(self, size: M) -> Option<M> {
        Some(index_2d(self.checked(size)?))
    }

    fn index_2d_unchecked<M: Major>(self) -> M {
        index_2d(self.unchecked())
    }

    fn get_1d<M: Major, I, T: AsRef<[I]>>(self, grid: &Grid1D<M, I, T>) -> Option<&I> {
        let point = self.checked(grid.msize())?;

        // SAFETY: point is checked
        debug_assert!(point < grid.size());
        Some(unsafe { get_1d(point, grid) })
    }

    unsafe fn get_1d_unchecked<M: Major, I, T: AsRef<[I]>>(self, grid: &Grid1D<M, I, T>) -> &I {
        get_1d(self.unchecked(), grid)
    }

    fn get_mut_1d<M: Major, I, T: AsMut<[I]>>(self, grid: &mut Grid1D<M, I, T>) -> Option<&mut I> {
        let point = self.checked(grid.msize())?;

        // SAFETY: point is checked
        debug_assert!(point < grid.size());
        Some(unsafe { get_mut_1d(point, grid) })
    }

    unsafe fn get_mut_1d_unchecked<M: Major, I, T: AsMut<[I]>>(
        self,
        grid: &mut Grid1D<M, I, T>,
    ) -> &mut I {
        get_mut_1d(self.unchecked(), grid)
    }

    fn get_2d<M: Major, I, T: AsRef<[U]>, U: AsRef<[I]>>(
        self,
        grid: &Grid2D<M, I, T, U>,
    ) -> Option<&I> {
        let point = self.checked(grid.msize())?;

        // SAFETY: point is checked
        debug_assert!(point < grid.size());
        Some(unsafe { get_2d(point, grid) })
    }

    unsafe fn get_2d_unchecked<M: Major, I, T: AsRef<[U]>, U: AsRef<[I]>>(
        self,
        grid: &Grid2D<M, I, T, U>,
    ) -> &I {
        get_2d(self.unchecked(), grid)
    }

    fn get_mut_2d<M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>>(
        self,
        grid: &mut Grid2D<M, I, T, U>,
    ) -> Option<&mut I> {
        let point = self.checked(grid.msize())?;

        // SAFETY: point is checked
        debug_assert!(point < grid.size());
        Some(unsafe { get_mut_2d(point, grid) })
    }

    unsafe fn get_mut_2d_unchecked<M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>>(
        self,
        grid: &mut Grid2D<M, I, T, U>,
    ) -> &mut I {
        get_mut_2d(self.unchecked(), grid)
    }
}

fn index_1d<M: Major>(point: Point, size: M) -> usize {
    let point = M::from(point);

    point.minor() * size.major() + point.major()
}

fn index_2d<M: Major>(point: Point) -> M {
    M::from(point)
}

unsafe fn get_1d<M: Major, I, T: AsRef<[I]>>(point: Point, grid: &Grid1D<M, I, T>) -> &I {
    let index = index_1d(point, grid.msize());

    grid.as_ref().get_unchecked(index)
}

unsafe fn get_mut_1d<M: Major, I, T: AsMut<[I]>>(
    point: Point,
    grid: &mut Grid1D<M, I, T>,
) -> &mut I {
    let index = index_1d(point, grid.msize());

    grid.as_mut().get_unchecked_mut(index)
}

unsafe fn get_2d<M: Major, I, T: AsRef<[U]>, U: AsRef<[I]>>(
    point: Point,
    grid: &Grid2D<M, I, T, U>,
) -> &I {
    let point: M = index_2d(point);

    grid.as_ref()
        .get_unchecked(point.minor())
        .as_ref()
        .get_unchecked(point.major())
}

unsafe fn get_mut_2d<M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>>(
    point: Point,
    grid: &mut Grid2D<M, I, T, U>,
) -> &mut I {
    let point: M = index_2d(point);

    grid.as_mut()
        .get_unchecked_mut(point.minor())
        .as_mut()
        .get_unchecked_mut(point.major())
}

impl Index0D for Point {
    fn unchecked(self) -> Point {
        self
    }
}

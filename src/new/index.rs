use crate::{Coord, Flat, Major, Point, ToRange};
use std::ops::{Range, RangeBounds};

mod index0d {
    use super::*;

    pub fn index<M: Major>(point: Point, size: M) -> usize {
        let point = M::from(point);
        point.minor() * size.major() + point.major()
    }

    pub unsafe fn get<M: Major, I, T: AsRef<[I]>>(point: Point, grid: &Flat<M, I, T>) -> &I {
        let index = index(point, grid.size());
        grid.as_ref().get_unchecked(index)
    }

    pub unsafe fn get_mut<M: Major, I, T: AsMut<[I]>>(
        point: Point,
        grid: &mut Flat<M, I, T>,
    ) -> &mut I {
        let index = index(point, grid.size());
        grid.as_mut().get_unchecked_mut(index)
    }
}

mod index1d {
    use super::*;

    pub fn index<M: Major>(
        (i, Range { start, end }): (usize, Range<usize>),
        size: M,
    ) -> Range<usize> {
        let point = M::new(start, i).into();
        let start = index0d::index(point, size);
        start..start + end
    }

    pub unsafe fn get<M: Major, I, T: AsRef<[I]>>(
        i: (usize, Range<usize>),
        grid: &Flat<M, I, T>,
    ) -> &[I] {
        let range = index(i, grid.size());
        grid.as_ref().get_unchecked(range)
    }

    pub unsafe fn get_mut<M: Major, I, T: AsMut<[I]>>(
        i: (usize, Range<usize>),
        grid: &mut Flat<M, I, T>,
    ) -> &mut [I] {
        let range = index(i, grid.size());
        grid.as_mut().get_unchecked_mut(range)
    }
}

macro_rules! index {
    ($(($mod:ident) $Trait:ident -> $Output:ty: $Base:ty, $Index:ty,)*) => { $(
        pub trait $Trait: Sized {
            fn unchecked<M: Major>(self, size: M) -> $Base;
            fn checked<M: Major>(self, size: M) -> Option<$Base>;

            fn index<M: Major>(self, size: M) -> Option<$Index> {
                Some($mod::index(self.checked(size)?, size))
            }

            fn index_unchecked<M: Major>(self, size: M) -> $Index {
                $mod::index(self.unchecked(size), size)
            }

            fn get<M: Major, I, T: AsRef<[I]>>(self, grid: &Flat<M, I, T>) -> Option<&$Output> {
                let index = self.checked(grid.size())?;
                // SAFETY: index is checked
                Some(unsafe { $mod::get(index, grid) })
            }

            fn get_mut<M: Major, I, T: AsMut<[I]>>(
                self,
                grid: &mut Flat<M, I, T>,
            ) -> Option<&mut $Output> {
                let index = self.checked(grid.size())?;
                // SAFETY: point is checked
                Some(unsafe { $mod::get_mut(index, grid) })
            }

            unsafe fn get_unchecked<M: Major, I, T: AsRef<[I]>>(
                self,
                grid: &Flat<M, I, T>,
            ) -> &$Output {
                let index = self.unchecked(grid.size());
                $mod::get(index, grid)
            }

            unsafe fn get_unchecked_mut<M: Major, I, T: AsMut<[I]>>(
                self,
                grid: &mut Flat<M, I, T>,
            ) -> &mut $Output {
                let index = self.unchecked(grid.size());
                $mod::get_mut(index, grid)
            }
        }
    )* };
}

index!(
    (index0d) Index0D -> I: Point, usize,
    (index1d) Index1D -> [I]: (usize, Range<usize>), Range<usize>,
);

impl Index0D for Point {
    fn unchecked<M: Major>(self, _: M) -> Point {
        self
    }

    fn checked<M: Major>(self, size: M) -> Option<Point> {
        let point = self.unchecked(size);

        if point < Into::<Point>::into(size) {
            Some(point)
        } else {
            None
        }
    }
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

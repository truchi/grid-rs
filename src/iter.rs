use crate::{
    index::{flat::Index0D, iter},
    *,
};
use std::{
    marker::PhantomData,
    ops::Range,
    slice::{from_raw_parts, from_raw_parts_mut},
};

/*
pub type Iter1D<M, T> = Iter<iter::Index1D<M>, T, unsafe fn(T, Point) -> <T as Grid>::Item>;
pub type Iter2D<M, T, Item> =
    Iter<iter::Index2D<M>, T, unsafe fn(T, (usize, Range<usize>)) -> Item>;
pub type ColIter<T> = Iter1D<ColMajor, T>;
pub type RowIter<T> = Iter1D<RowMajor, T>;
pub type ColsIter<T> = Iter2D<ColMajor, T, <T as Grid>::Col>;
pub type RowsIter<T> = Iter2D<RowMajor, T, <T as Grid>::Row>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Iter<I, T, F> {
    index: I,
    grid:  T,
    func:  F,
}

impl<M, T, F> Iter<iter::Index1D<M>, T, F> {
    /// SAFETY: TODO
    pub unsafe fn new<TM: Major>(grid: T, index: impl Index1D, func: F) -> Option<Self>
    where
        T: WithMSize<TM>,
    {
        let index = index.checked(grid.msize())?.into();

        Some(Self { index, grid, func })
    }

    /// SAFETY: TODO
    pub unsafe fn new_unchecked<TM: Major>(grid: T, index: impl Index1D, func: F) -> Self
    where
        T: WithMSize<TM>,
    {
        let index = index.unchecked(grid.msize()).into();

        Self { index, grid, func }
    }
}

impl<M: Major, T: WithSize, F> Iter<iter::Index2D<M>, T, F> {
    /// SAFETY: TODO
    pub unsafe fn new(grid: T, index: impl Index2D, func: F) -> Option<Self> {
        let index = index.checked(grid.size())?.into();

        Some(Self { index, grid, func })
    }

    /// SAFETY: TODO
    pub unsafe fn new_unchecked(grid: T, index: impl Index2D, func: F) -> Self {
        let index = index.unchecked(grid.size()).into();

        Self { index, grid, func }
    }
}

impl<'a, I: Iterator, T, Item> Iterator for Iter<I, &'a T, unsafe fn(&'a T, I::Item) -> Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.next()?;

        // SAFETY: TODO
        Some(unsafe { (self.func)(self.grid, index) })
    }
}

impl<'a, I: Iterator, T, Item> Iterator
    for Iter<I, &'a mut T, unsafe fn(&'a mut T, I::Item) -> Item>
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.next()?;

        // SAFETY:
        // constructors guaranty that:
        // item_unchecked returns valid, non-overlapping references.
        // Then, it is safe to extend grid's lifetime
        let grid = unsafe { std::mem::transmute::<&mut T, &mut T>(self.grid) };

        // SAFETY: TODO
        Some(unsafe { (self.func)(grid, index) })
    }
}
*/

// ==================================================== //

// TODO Minors

pub struct Minor<'a, M, I, T> {
    items:    &'a [I],
    current:  usize,
    by:       usize,
    count:    usize,
    _phantom: PhantomData<(M, T)>,
}

impl<'a, M: Major, I, T: AsRef<[I]>> Minor<'a, M, I, T> {
    pub unsafe fn new(
        grid: &'a Flat<M, I, T>,
        (i, Range { start, end }): (usize, Range<usize>),
    ) -> Self {
        let msize = grid.msize();

        Self {
            items:    grid.as_ref(),
            current:  M::new(start, i).into().index(msize),
            count:    end - start,
            by:       msize.major(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, M, I, T> Iterator for Minor<'a, M, I, T> {
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let current = self.current;
            self.current += self.by;
            self.count -= 1;

            // SAFETY: users guaranty index is in bounds at construction
            debug_assert!(current < self.items.len(), "Index out of bounds");
            Some(unsafe { self.items.get_unchecked(current) })
        }
    }
}

pub struct MinorMut<'a, M, I, T> {
    items:    &'a mut [I],
    i:        usize,
    major:    usize,
    count:    usize,
    _phantom: PhantomData<(M, T)>,
}

impl<'a, M: Major, I, T: AsMut<[I]>> MinorMut<'a, M, I, T> {
    pub unsafe fn new(
        grid: &'a mut Flat<M, I, T>,
        (i, Range { start, end }): (usize, Range<usize>),
    ) -> Self {
        // Splitting to the first col/row of interest
        let major = grid.msize().major();
        let first = start * major;
        let items = grid.as_mut();
        debug_assert!(first <= items.len(), "Index out of bounds");
        let items = items.get_unchecked_mut(first..);

        Self {
            items,
            i,
            major,
            count: end - start,
            _phantom: PhantomData,
        }
    }
}

impl<'a, M, I, T> Iterator for MinorMut<'a, M, I, T> {
    type Item = &'a mut I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let len = self.items.len();
            let ptr = self.items.as_mut_ptr();

            // SAFETY: users guaranty index is in bounds at construction
            let (item, items) = unsafe {
                debug_assert!(self.major <= self.items.len());
                let (slice, items) = (
                    from_raw_parts_mut(ptr, self.major),
                    from_raw_parts_mut(ptr.add(self.major), len - self.major),
                );

                debug_assert!(self.i < slice.len());
                (slice.get_unchecked_mut(self.i), items)
            };

            self.items = items;
            self.count -= 1;

            Some(item)
        }
    }
}

pub struct Majors<'a, M, I, T> {
    grid:  &'a Flat<M, I, T>,
    index: Point<Range<usize>>,
}

impl<'a, M, I, T> Majors<'a, M, I, T> {
    pub unsafe fn new(grid: &'a Flat<M, I, T>, index: Point<Range<usize>>) -> Self {
        Self { grid, index }
    }
}

impl<'a, I, T: AsRef<[I]>> Iterator for Majors<'a, RowMajor, I, T> {
    type Item = &'a [I];

    fn next(&mut self) -> Option<Self::Item> {
        let index = (self.index.y.next()?, self.index.x.clone());

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe { self.grid.row_unchecked(index) })
    }
}

impl<'a, I, T: AsRef<[I]>> Iterator for Majors<'a, ColMajor, I, T> {
    type Item = &'a [I];

    fn next(&mut self) -> Option<Self::Item> {
        let index = (self.index.x.next()?, self.index.y.clone());

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe { self.grid.col_unchecked(index) })
    }
}

pub struct MajorsMut<'a, M, I, T> {
    items:    &'a mut [I],
    range:    Range<usize>,
    major:    usize,
    count:    usize,
    _phantom: PhantomData<(M, T)>,
}

impl<'a, M: Major, I, T: AsMut<[I]>> MajorsMut<'a, M, I, T> {
    pub unsafe fn new(grid: &'a mut Flat<M, I, T>, index: Point<Range<usize>>) -> Self {
        let (range, minor) = to_major::<M>(index);

        // Splitting to the first col/row of interest
        let major = grid.msize().major();
        let first = minor.start * major;
        let items = grid.as_mut();
        debug_assert!(first <= items.len(), "Index out of bounds");
        let items = items.get_unchecked_mut(first..);

        Self {
            items,
            range,
            major,
            count: minor.end - minor.start,
            _phantom: PhantomData,
        }
    }
}

impl<'a, M, I, T> Iterator for MajorsMut<'a, M, I, T> {
    type Item = &'a mut [I];

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let len = self.items.len();
            let ptr = self.items.as_mut_ptr();

            // SAFETY: users guaranty index is in bounds at construction
            let (slice, items) = unsafe {
                debug_assert!(self.major <= self.items.len());
                let (slice, items) = (
                    from_raw_parts_mut(ptr, self.major),
                    from_raw_parts_mut(ptr.add(self.major), len - self.major),
                );

                debug_assert!(self.range.end <= slice.len());
                (slice.get_unchecked_mut(self.range.clone()), items)
            };

            self.items = items;
            self.count -= 1;

            Some(slice)
        }
    }
}

fn to_major<M: Major>(index: Point<Range<usize>>) -> (Range<usize>, Range<usize>) {
    let start = M::from(Point {
        x: index.x.start,
        y: index.y.start,
    });
    let end = M::from(Point {
        x: index.x.end,
        y: index.y.end,
    });

    (start.major()..end.major(), start.minor()..end.minor())
}

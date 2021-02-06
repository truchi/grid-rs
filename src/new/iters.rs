use crate::{new::index::iters, *};
use std::{marker::PhantomData, ops::Range};

type Iter1D<M, T> = Iter<iters::Index1D<M>, T, unsafe fn(T, Point) -> <T as Grid>::Item>;
type Iter2D<M, T, Item> = Iter<iters::Index2D<M>, T, unsafe fn(T, (usize, Range<usize>)) -> Item>;
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

impl<M, T: Grid> Iter1D<M, T> {
    /// SAFETY: TODO
    pub unsafe fn new<TM: Major>(
        grid: T,
        index: impl Index1D,
        func: unsafe fn(T, Point) -> T::Item,
    ) -> Option<Self>
    where
        T: WithMSize<TM>,
    {
        let index = index.checked(grid.msize())?.into();

        Some(Self { index, grid, func })
    }

    /// SAFETY: TODO
    pub unsafe fn new_unchecked<TM: Major>(
        grid: T,
        index: impl Index1D,
        func: unsafe fn(T, Point) -> T::Item,
    ) -> Self
    where
        T: WithMSize<TM>,
    {
        let index = index.unchecked(grid.msize()).into();

        Self { index, grid, func }
    }
}

impl<M: Major, T: WithSize, Item> Iter2D<M, T, Item> {
    /// SAFETY: TODO
    pub unsafe fn new(
        grid: T,
        index: impl Index2D,
        func: unsafe fn(T, (usize, Range<usize>)) -> Item,
    ) -> Option<Self> {
        let index = index.checked(grid.size())?.into();

        Some(Self { index, grid, func })
    }

    /// SAFETY: TODO
    pub unsafe fn new_unchecked(
        grid: T,
        index: impl Index2D,
        func: unsafe fn(T, (usize, Range<usize>)) -> Item,
    ) -> Self {
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

/*
type Helper1D<M, G, Ret> = Helper<XMajor, iters::Index1D<M>, G, Ret>;
type Helper2D<M, G, Ret> = Helper<XMajor, iters::Index2D<M>, G, Ret>;
pub type XHelper1D<G, Ret> = Helper1D<XMajor, G, Ret>;
pub type YHelper1D<G, Ret> = Helper1D<YMajor, G, Ret>;
pub type XHelper2D<G, I> = Helper2D<XMajor, G, <G as GridIter<I>>::Row>;
pub type YHelper2D<G, I> = Helper2D<YMajor, G, <G as GridIter<I>>::Col>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Helper<M, Idx: Iterator, G, Ret> {
    grid:    G,
    f:       unsafe fn(G, <Idx as Iterator>::Item) -> Ret,
    index:   Idx,
    phantom: PhantomData<M>,
}

impl<M: Major, G: WithMSize<M>, Ret> Helper1D<M, G, Ret> {
    /// SAFETY: TODO
    pub unsafe fn new(grid: G, index: impl Index1D, f: unsafe fn(G, Point) -> Ret) -> Option<Self> {
        let index = index.checked(grid.msize())?.into();

        Some(Self {
            grid,
            f,
            index,
            phantom: PhantomData,
        })
    }

    /// SAFETY: TODO
    pub unsafe fn new_unchecked(
        grid: G,
        index: impl Index1D,
        f: unsafe fn(G, Point) -> Ret,
    ) -> Self {
        let index = index.unchecked(grid.msize()).into();

        Self {
            grid,
            f,
            index,
            phantom: PhantomData,
        }
    }
}

impl<M: Major, G: WithSize, Ret> Helper2D<M, G, Ret> {
    /// SAFETY: TODO
    pub unsafe fn new(
        grid: G,
        index: impl Index2D,
        f: unsafe fn(G, (usize, Range<usize>)) -> Ret,
    ) -> Option<Self> {
        let index = index.checked(grid.size())?.into();

        Some(Self {
            grid,
            f,
            index,
            phantom: PhantomData,
        })
    }

    /// SAFETY: TODO
    pub unsafe fn new_unchecked(
        grid: G,
        index: impl Index2D,
        f: unsafe fn(G, (usize, Range<usize>)) -> Ret,
    ) -> Self {
        let index = index.unchecked(grid.size()).into();

        Self {
            grid,
            f,
            index,
            phantom: PhantomData,
        }
    }
}

impl<'a, M: Major, Idx: Iterator, G, Ret> Iterator for Helper<M, Idx, &'a G, Ret> {
    type Item = Ret;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.index.next()?;

        // SAFETY: TODO
        Some(unsafe { (self.f)(self.grid, point) })
    }
}

impl<'a, M: Major, Idx: Iterator, G, Ret> Iterator for Helper<M, Idx, &'a mut G, Ret> {
    type Item = Ret;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.index.next()?;

        // SAFETY:
        // constructors guaranty that:
        // item_unchecked returns valid, non-overlapping references.
        // Then, it is safe to extend grid's lifetime
        let grid = unsafe { std::mem::transmute::<&mut G, &mut G>(self.grid) };

        // SAFETY: TODO
        Some(unsafe { (self.f)(grid, point) })
    }
}
*/

// =================================================================== //
// Hopefully those 2 are the same as the one above
// TODO delete Slice, Slices
// =================================================================== //

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Slice<M, I, G> {
    grid:  G,
    f:     unsafe fn(G, Point) -> I,
    index: iters::Index1D<M>,
}

impl<M: Major, I, G: WithMSize<M>> Slice<M, I, G> {
    /// SAFETY: TODO
    pub unsafe fn new(grid: G, index: impl Index1D, f: unsafe fn(G, Point) -> I) -> Option<Self> {
        let index = index.checked(grid.msize())?.into();

        Some(Self { grid, f, index })
    }

    /// SAFETY: TODO
    unsafe fn new_unchecked(grid: G, index: impl Index1D, f: unsafe fn(G, Point) -> I) -> Self {
        let index = index.unchecked(grid.msize()).into();

        Self { grid, f, index }
    }
}

impl<'a, M: Major, I, G> Iterator for Slice<M, &'a I, &'a G> {
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.index.next()?;

        // SAFETY: TODO
        Some(unsafe { (self.f)(self.grid, point) })
    }
}

impl<'a, M: Major, I, G> Iterator for Slice<M, &'a mut I, &'a mut G> {
    type Item = &'a mut I;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.index.next()?;

        // SAFETY:
        // constructors guaranty that:
        // item_unchecked returns valid, non-overlapping references.
        // Then, it is safe to extend grid's lifetime
        let grid = unsafe { std::mem::transmute::<&mut G, &mut G>(self.grid) };

        // SAFETY: TODO
        Some(unsafe { (self.f)(grid, point) })
    }
}

// ========================================================================================== //
// ========================================================================================== //
// ========================================================================================== //
// ========================================================================================== //

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Slices<M, G, S> {
    grid:  G,
    f:     unsafe fn(G, (usize, Range<usize>)) -> S,
    index: iters::Index2D<M>,
}

impl<M: Major, G: WithSize, S> Slices<M, G, S> {
    /// SAFETY: TODO
    unsafe fn new(
        grid: G,
        index: impl Index2D,
        f: unsafe fn(G, (usize, Range<usize>)) -> S,
    ) -> Option<Self> {
        let index = index.checked(grid.size())?.into();

        Some(Self { grid, f, index })
    }

    /// SAFETY: TODO
    unsafe fn new_unchecked(
        grid: G,
        index: impl Index2D,
        f: unsafe fn(G, (usize, Range<usize>)) -> S,
    ) -> Self {
        let index = index.unchecked(grid.size()).into();

        Self { grid, f, index }
    }
}

impl<'a, M: Major, G, S> Iterator for Slices<M, &'a G, S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.next()?;

        // SAFETY: TODO
        Some(unsafe { (self.f)(self.grid, index) })
    }
}

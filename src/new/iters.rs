use crate::{new::index::iters, *};
use std::{marker::PhantomData, ops::Range};

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

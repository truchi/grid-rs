use crate::*;
use std::{marker::PhantomData, ops::Range};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Slice<M, I, G> {
    grid:    G,
    f:       unsafe fn(G, Point) -> I,
    i:       usize,
    range:   Range<usize>,
    phantom: PhantomData<(M, I)>,
}

impl<M: Major, I, G: WithMSize<M>> Slice<M, I, G> {
    /// SAFETY: TODO
    pub unsafe fn new(grid: G, index: impl Index1D, f: unsafe fn(G, Point) -> I) -> Option<Self> {
        let index = index.checked(grid.msize())?;

        // SAFETY: index is checked
        Some(Self::new_unchecked(grid, index, f))
    }

    /// SAFETY: TODO
    unsafe fn new_unchecked(grid: G, index: impl Index1D, f: unsafe fn(G, Point) -> I) -> Self {
        let (i, range) = index.unchecked(grid.msize());

        Self {
            grid,
            f,
            i,
            range,
            phantom: PhantomData,
        }
    }
}

impl<'a, M: Major, I, G> Iterator for Slice<M, &'a I, &'a G> {
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.range;

        if start < end {
            self.range.start += 1;
            let point: Point = M::new(start, self.i).into();

            // SAFETY: TODO
            Some(unsafe { (self.f)(self.grid, point) })
        } else {
            None
        }
    }
}

impl<'a, M: Major, I, G> Iterator for Slice<M, &'a mut I, &'a mut G> {
    type Item = &'a mut I;

    fn next(&mut self) -> Option<Self::Item> {
        let Range { start, end } = self.range;

        if start < end {
            self.range.start += 1;
            let point: Point = M::new(start, self.i).into();

            // SAFETY:
            // constructors guaranty that:
            // item_unchecked returns valid, non-overlapping references.
            // Then, it is safe to extend grid's lifetime
            let grid = unsafe { std::mem::transmute::<&mut G, &mut G>(self.grid) };

            // SAFETY: TODO
            Some(unsafe { (self.f)(grid, point) })
        } else {
            None
        }
    }
}

// ========================================================================================== //
// ========================================================================================== //
// ========================================================================================== //
// ========================================================================================== //

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Slices<M, G, S> {
    grid:    G,
    f:       unsafe fn(G, (usize, Range<usize>)) -> S,
    index:   Point<Range<usize>>,
    phantom: PhantomData<M>,
}

impl<M: Major, G: WithSize, S> Slices<M, G, S> {
    /// SAFETY: TODO
    unsafe fn new(
        grid: G,
        index: impl Index2D,
        f: unsafe fn(G, (usize, Range<usize>)) -> S,
    ) -> Option<Self> {
        let index = index.checked(grid.size())?;

        // SAFETY: index is checked
        Some(Self::new_unchecked(grid, index, f))
    }

    /// SAFETY: TODO
    unsafe fn new_unchecked(
        grid: G,
        index: impl Index2D,
        f: unsafe fn(G, (usize, Range<usize>)) -> S,
    ) -> Self {
        let index = index.unchecked(grid.size());

        Self {
            grid,
            f,
            index,
            phantom: PhantomData,
        }
    }
}

impl<'a, M: Major, G, S> Iterator for Slices<M, &'a G, S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        None
        // let Range { start, end } = self.index.x;
        //
        // if start < end {
        //     // TODO WRONG!!!!!!!!!!!!!!!!!!!!!!!!
        //     self.index.x.start += 1;
        //     // let point
        //
        //     // SAFETY: constructors guaranty this is safe
        //     Some(unsafe { (self.f)(self.grid, (start, self.index.y.clone()))
        // }) } else {
        //     None
        // }
    }
}

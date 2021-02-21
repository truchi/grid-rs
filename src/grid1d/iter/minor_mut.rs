use super::*;
use std::{marker::PhantomData, ops::Range, slice::from_raw_parts_mut};

pub struct MinorMut<'a, M, I, T> {
    items:    &'a mut [I],
    i:        usize,
    major:    usize,
    count:    usize,
    _phantom: PhantomData<(M, T)>,
}

impl<'a, M: Major, I, T: AsMut<[I]>> MinorMut<'a, M, I, T> {
    pub(crate) unsafe fn new_unchecked(grid: &'a mut Grid1D<M, I, T>, index: impl Index1D) -> Self {
        let msize = grid.msize();
        let (i, Range { start, end }) = index.unchecked(msize.minor());

        // Splitting to the first col/row of interest
        let major = msize.major();
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

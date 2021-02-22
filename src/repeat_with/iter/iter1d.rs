use crate::*;
use std::{marker::PhantomData, ops::Range};

pub struct Iter1D<M, F> {
    fun:      F,
    index:    (usize, Range<usize>),
    _phantom: PhantomData<M>,
}

impl<M, F> Iter1D<M, F> {
    pub fn new(fun: F, index: (usize, Range<usize>)) -> Self {
        Self {
            fun,
            index,
            _phantom: PhantomData,
        }
    }
}

impl<M: Major, I, F: FnMut(Point) -> I> Iterator for Iter1D<M, F> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, current) = (self.index.0, self.index.1.next()?);

        Some((self.fun)(M::new(current, i).into()))
    }
}

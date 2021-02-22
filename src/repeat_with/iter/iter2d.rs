use crate::*;
use std::marker::PhantomData;

pub struct Iter2D<M, F> {
    fun:      F,
    index:    Rect,
    _phantom: PhantomData<M>,
}

impl<M, F> Iter2D<M, F> {
    pub fn new(fun: F, index: Rect) -> Self {
        Self {
            fun,
            index,
            _phantom: PhantomData,
        }
    }
}

impl<I, F: Clone + Fn(Point) -> I> Iterator for Iter2D<RowMajor, F> {
    type Item = super::Iter1D<RowMajor, F>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item::new(
            self.fun.clone(),
            (self.index.y.next()?, self.index.x.clone()),
        ))
    }
}

impl<I, F: Clone + Fn(Point) -> I> Iterator for Iter2D<ColMajor, F> {
    type Item = super::Iter1D<ColMajor, F>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item::new(
            self.fun.clone(),
            (self.index.x.next()?, self.index.y.clone()),
        ))
    }
}

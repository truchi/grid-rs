use crate::*;
use std::{marker::PhantomData, ops::Range};

#[derive(Copy, Clone, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct RepeatWith<F> {
    size: Size,
    fun:  F,
}

impl<F> RepeatWith<F> {
    pub fn new(size: Size, fun: F) -> Self {
        Self { size, fun }
    }
}

impl<F> WithSize for RepeatWith<F> {
    fn size(&self) -> Size {
        self.size
    }
}

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

pub struct Iter2D<M, F> {
    fun:      F,
    index:    Point<Range<usize>>,
    _phantom: PhantomData<M>,
}

impl<M, F> Iter2D<M, F> {
    pub fn new(fun: F, index: Point<Range<usize>>) -> Self {
        Self {
            fun,
            index,
            _phantom: PhantomData,
        }
    }
}

impl<I, F: Clone + Fn(Point) -> I> Iterator for Iter2D<RowMajor, F> {
    type Item = Iter1D<RowMajor, F>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item::new(
            self.fun.clone(),
            (self.index.y.next()?, self.index.x.clone()),
        ))
    }
}

impl<I, F: Clone + Fn(Point) -> I> Iterator for Iter2D<ColMajor, F> {
    type Item = Iter1D<ColMajor, F>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item::new(
            self.fun.clone(),
            (self.index.x.next()?, self.index.y.clone()),
        ))
    }
}

pub struct Items<F> {
    fun:    F,
    xstart: usize,
    xend:   usize,
    yend:   usize,
    x:      usize,
    y:      usize,
}

impl<F> Items<F> {
    pub fn new(fun: F, index: Point<Range<usize>>) -> Self {
        let Point {
            x: Range {
                start: x,
                end: xend,
            },
            y: Range {
                start: y,
                end: yend,
            },
        } = index;

        Self {
            fun,
            xstart: x,
            xend,
            yend,
            x,
            y,
        }
    }
}

impl<I, F: FnMut(Point) -> I> Iterator for Items<F> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.x, self.y);

        let (x, y) = if x < self.xend {
            self.x += 1;
            (x, y)
        } else if y < self.yend {
            self.x = self.xstart;
            self.y += 1;
            (x, y)
        } else {
            return None;
        };

        Some((self.fun)(Point { x, y }))
    }
}

// ================= //

impl<I, F: FnMut(Point) -> I> Grid for RepeatWith<F> {
    type Item = I;

    unsafe fn item_unchecked(mut self, index: impl Index0D) -> Self::Item {
        (self.fun)(index.unchecked())
    }
}

impl<I, F: FnMut(Point) -> I> GridRow for RepeatWith<F> {
    type Row = Iter1D<RowMajor, F>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        Self::Row::new(self.fun, index.unchecked(RowMajor::from(self.size)))
    }
}

impl<I, F: FnMut(Point) -> I> GridCol for RepeatWith<F> {
    type Col = Iter1D<ColMajor, F>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Self::Col::new(self.fun, index.unchecked(ColMajor::from(self.size)))
    }
}

impl<I, F: Clone + Fn(Point) -> I> GridRows for RepeatWith<F> {
    type Rows = Iter2D<RowMajor, F>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        Self::Rows::new(self.fun, index.unchecked(self.size))
    }
}

impl<I, F: Clone + Fn(Point) -> I> GridCols for RepeatWith<F> {
    type Cols = Iter2D<ColMajor, F>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        Self::Cols::new(self.fun, index.unchecked(self.size))
    }
}

impl<I, F: FnMut(Point) -> I> GridItems for RepeatWith<F> {
    type Items = Items<F>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        Self::Items::new(self.fun, index.unchecked(self.size))
    }
}

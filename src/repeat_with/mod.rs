pub mod iter;

use crate::*;

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

impl<I, F: FnMut(Point) -> I> Grid for RepeatWith<F> {
    type Item = I;

    unsafe fn item_unchecked(mut self, index: impl Index0D) -> Self::Item {
        (self.fun)(index.unchecked())
    }
}

impl<I, F: FnMut(Point) -> I> GridRow for RepeatWith<F> {
    type Row = iter::Iter1D<RowMajor, F>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        Self::Row::new(self.fun, index.row_unchecked(self.size))
    }
}

impl<I, F: FnMut(Point) -> I> GridCol for RepeatWith<F> {
    type Col = iter::Iter1D<ColMajor, F>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Self::Col::new(self.fun, index.col_unchecked(self.size))
    }
}

impl<I, F: Clone + Fn(Point) -> I> GridRows for RepeatWith<F> {
    type Rows = iter::Iter2D<RowMajor, F>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        Self::Rows::new(self.fun, index.unchecked(self.size))
    }
}

impl<I, F: Clone + Fn(Point) -> I> GridCols for RepeatWith<F> {
    type Cols = iter::Iter2D<ColMajor, F>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        Self::Cols::new(self.fun, index.unchecked(self.size))
    }
}

impl<I, F: FnMut(Point) -> I> GridItems for RepeatWith<F> {
    type Items = iter::Items<F>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        Self::Items::new(self.fun, index.unchecked(self.size))
    }
}

use crate::*;
use std::{
    iter::{repeat, Repeat as StdRepeat, Take},
    ops::Range,
};

pub struct Repeat<I> {
    size: Size,
    item: I,
}

impl<I> Repeat<I> {
    pub fn new(size: Size, item: I) -> Self {
        Self { size, item }
    }
}

impl<I> WithSize for Repeat<I> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<I: Clone> Grid for Repeat<I> {
    type Col = Take<StdRepeat<I>>;
    type Cols = Take<StdRepeat<Self::Col>>;
    type Item = I;
    type Items = Take<StdRepeat<I>>;
    type Row = Take<StdRepeat<I>>;
    type Rows = Take<StdRepeat<Self::Row>>;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        self.item
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        let (_, Range { start, end }) = index.unchecked(ColMajor::from(self.size));

        repeat(self.item).take(end - start)
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        let (_, Range { start, end }) = index.unchecked(RowMajor::from(self.size));

        repeat(self.item).take(end - start)
    }

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        let Point { x, y } = index.unchecked(self.size);

        repeat(self.col_unchecked((0, x))).take(y.end - y.start)
    }

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        let Point { x, y } = index.unchecked(self.size);

        repeat(self.row_unchecked((0, y))).take(x.end - x.start)
    }

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        let Point { x, y } = index.unchecked(self.size);

        repeat(self.item).take((x.end - x.start) * (y.end - y.start))
    }
}

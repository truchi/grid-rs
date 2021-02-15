use crate::*;
use std::{iter::repeat, ops::Range};

#[derive(Copy, Clone, PartialOrd, Eq, PartialEq, Default, Debug)]
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

impl<I> GridItem for Repeat<I> {
    type Item = I;

    unsafe fn item_unchecked(self, _: impl Index0D) -> Self::Item {
        self.item
    }
}

impl<I: Clone> GridRow for Repeat<I> {
    type Row = std::iter::Take<std::iter::Repeat<I>>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        let (_, Range { start, end }) = index.unchecked(RowMajor::from(self.size));

        repeat(self.item).take(end - start)
    }
}

impl<I: Clone> GridCol for Repeat<I> {
    type Col = std::iter::Take<std::iter::Repeat<I>>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        let (_, Range { start, end }) = index.unchecked(ColMajor::from(self.size));

        repeat(self.item).take(end - start)
    }
}

impl<I: Clone> GridRows for Repeat<I> {
    type Rows = std::iter::Take<std::iter::Repeat<Self::Row>>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        let Point { x, y } = index.unchecked(self.size);

        repeat(self.row_unchecked((0, y))).take(x.end - x.start)
    }
}

impl<I: Clone> GridCols for Repeat<I> {
    type Cols = std::iter::Take<std::iter::Repeat<Self::Col>>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        let Point { x, y } = index.unchecked(self.size);

        repeat(self.col_unchecked((0, y))).take(x.end - x.start)
    }
}

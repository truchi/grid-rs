use crate::*;
use std::iter::{Cloned as StdCloned, Map};

pub struct Cloned<T>(T);

impl<T: WithSize> WithSize for Cloned<T> {
    fn size(&self) -> Size {
        self.0.size()
    }
}

impl<'a, I: 'a + Clone, T: Grid<Item = &'a I>> Grid for Cloned<T> {
    type Col = StdCloned<<T::Col as IntoIterator>::IntoIter>;
    type Cols = Map<T::Cols, fn(T::Col) -> Self::Col>;
    type Item = I;
    type Items = StdCloned<T::Items>;
    type Row = StdCloned<<T::Row as IntoIterator>::IntoIter>;
    type Rows = Map<T::Rows, fn(T::Row) -> Self::Row>;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        self.0.item_unchecked(index).clone()
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        self.0.col_unchecked(index).into_iter().cloned()
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        self.0.row_unchecked(index).into_iter().cloned()
    }

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        self.0
            .cols_unchecked(index)
            .map(|col| col.into_iter().cloned())
    }

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        self.0
            .rows_unchecked(index)
            .map(|row| row.into_iter().cloned())
    }

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.0.items_unchecked(index).cloned()
    }
}

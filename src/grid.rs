use crate::*;
use std::ops::{Index, IndexMut};

pub trait GridItem<I>: WithSize + Sized {
    unsafe fn item_unchecked(self, index: impl Index0D) -> I;

    fn item(self, index: impl Index0D) -> Option<I> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }
}

pub trait GridCol<I>: GridItem<I> {
    type Col: IntoIterator<Item = I>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col;

    fn col(self, index: impl Index1D) -> Option<Self::Col> {
        let index = index.checked(ColMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.col_unchecked(index) })
    }
}

pub trait GridRow<I>: GridItem<I> {
    type Row: IntoIterator<Item = I>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row;

    fn row(self, index: impl Index1D) -> Option<Self::Row> {
        let index = index.checked(RowMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.row_unchecked(index) })
    }
}

pub trait GridCols<I>: GridCol<I> {
    type Cols: Iterator<Item = Self::Col>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols;

    fn cols(self, index: impl Index2D) -> Option<Self::Cols> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.cols_unchecked(index) })
    }
}

pub trait GridRows<I>: GridRow<I> {
    type Rows: Iterator<Item = Self::Row>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows;

    fn rows(self, index: impl Index2D) -> Option<Self::Rows> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.rows_unchecked(index) })
    }
}

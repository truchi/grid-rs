use crate::*;
use std::ops::{Index, IndexMut};

pub trait GridItem: WithSize + Sized {
    type Item;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item;

    fn item(self, index: impl Index0D) -> Option<Self::Item> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }
}

pub trait GridCol: GridItem {
    type Col: IntoIterator<Item = Self::Item>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col;

    fn col(self, index: impl Index1D) -> Option<Self::Col> {
        let index = index.checked(ColMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.col_unchecked(index) })
    }
}

pub trait GridRow: GridItem {
    type Row: IntoIterator<Item = Self::Item>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row;

    fn row(self, index: impl Index1D) -> Option<Self::Row> {
        let index = index.checked(RowMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.row_unchecked(index) })
    }
}

pub trait GridCols: GridCol {
    type Cols: IntoIterator<Item = Self::Col>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols;

    fn cols(self, index: impl Index2D) -> Option<Self::Cols> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.cols_unchecked(index) })
    }
}

pub trait GridRows: GridRow {
    type Rows: IntoIterator<Item = Self::Row>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows;

    fn rows(self, index: impl Index2D) -> Option<Self::Rows> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.rows_unchecked(index) })
    }
}

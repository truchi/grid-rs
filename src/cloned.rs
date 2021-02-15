use crate::*;
use std::ops::{Deref, DerefMut};

pub struct Cloned<T>(T);

impl<T> Deref for Cloned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Cloned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, I: 'a + Clone, T: GridItem<Item = &'a I>> GridItem for Cloned<T> {
    type Item = I;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        self.0.item_unchecked(index).clone()
    }
}

impl<'a, I: 'a + Clone, T: GridRow<Item = &'a I>> GridRow for Cloned<T> {
    type Row = std::iter::Cloned<<T::Row as IntoIterator>::IntoIter>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        self.0.row_unchecked(index).into_iter().cloned()
    }
}

impl<'a, I: 'a + Clone, T: GridCol<Item = &'a I>> GridCol for Cloned<T> {
    type Col = std::iter::Cloned<<T::Col as IntoIterator>::IntoIter>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        self.0.col_unchecked(index).into_iter().cloned()
    }
}

impl<'a, I: 'a + Clone, T: GridRows<Item = &'a I>> GridRows for Cloned<T> {
    type Rows = std::iter::Map<T::Rows, fn(T::Row) -> Self::Row>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        self.0
            .rows_unchecked(index)
            .map(|row| row.into_iter().cloned())
    }
}

impl<'a, I: 'a + Clone, T: GridCols<Item = &'a I>> GridCols for Cloned<T> {
    type Cols = std::iter::Map<T::Cols, fn(T::Col) -> Self::Col>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        self.0
            .cols_unchecked(index)
            .map(|col| col.into_iter().cloned())
    }
}

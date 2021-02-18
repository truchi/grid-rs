use crate::*;

pub trait GridItem: WithSize + Sized {
    type Item;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item;

    fn item(self, index: impl Index0D) -> Option<Self::Item> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }

    fn crop(self, rect: impl Index2D) -> Option<Crop<Self>> {
        Crop::new(rect, self)
    }

    unsafe fn crop_unchecked(self, rect: impl Index2D) -> Crop<Self> {
        Crop::new_unchecked(rect, self)
    }

    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: GridItem<Item = &'a T>,
        T: 'a + Clone,
    {
        Cloned::new(self)
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

pub trait GridItems: GridItem {
    type Items: IntoIterator<Item = Self::Item>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items;

    fn items(self, index: impl Index2D) -> Option<Self::Items> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.items_unchecked(index) })
    }
}

use crate::*;

#[allow(dead_code)]
pub struct Crop<T> {
    rect: Rect,
    grid: T,
}

impl<T: WithSize> Crop<T> {
    pub fn new(rect: impl Index2D, grid: T) -> Option<Self> {
        let rect = rect.checked(grid.size())?;

        Some(Self { rect, grid })
    }

    pub unsafe fn new_unchecked(rect: impl Index2D, grid: T) -> Self {
        let rect = rect.unchecked(grid.size());

        Self { rect, grid }
    }
}

impl<I> WithSize for Crop<I> {
    fn size(&self) -> Size {
        let Size { x, y } = self.rect.clone();

        Size {
            x: x.end - x.start,
            y: y.end - y.start,
        }
    }
}

impl<T: Grid> Grid for Crop<T> {
    type Item = T::Item;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        let mut index = index.unchecked();
        index.x += self.rect.x.start;
        index.y += self.rect.y.start;

        self.grid.item_unchecked(index)
    }
}

impl<T: GridCol> GridCol for Crop<T> {
    type Col = T::Col;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        let mut index = index.col_unchecked(self.size());
        index.0 += self.rect.x.start;
        index.1.start += self.rect.y.start;

        self.grid.col_unchecked(index)
    }
}

impl<T: GridRow> GridRow for Crop<T> {
    type Row = T::Row;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        let mut index = index.row_unchecked(self.size());
        index.0 += self.rect.y.start;
        index.1.start += self.rect.x.start;

        self.grid.row_unchecked(index)
    }
}

impl<T: GridCols> GridCols for Crop<T> {
    type Cols = T::Cols;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        let mut index = index.unchecked(self.size());
        index.x.start += self.rect.x.start;
        index.y.start += self.rect.y.start;

        self.grid.cols_unchecked(index)
    }
}

impl<T: GridRows> GridRows for Crop<T> {
    type Rows = T::Rows;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        let mut index = index.unchecked(self.size());
        index.x.start += self.rect.x.start;
        index.y.start += self.rect.y.start;

        self.grid.rows_unchecked(index)
    }
}

impl<T: GridItems> GridItems for Crop<T> {
    type Items = T::Items;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        let mut index = index.unchecked(self.size());
        index.x.start += self.rect.x.start;
        index.y.start += self.rect.y.start;

        self.grid.items_unchecked(index)
    }
}

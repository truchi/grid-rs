use crate::*;

pub trait Grid<Item>: WithSize + Sized {
    /// The type of the column iterator.
    type Col: Iterator<Item = Item>;

    /// The type of the row iterator.
    type Row: Iterator<Item = Item>;

    /// The type of the columns iterator.
    type Cols: Iterator<Item = Self::Col>;

    /// The type of the rows iterator.
    type Rows: Iterator<Item = Self::Row>;

    /// The type of the items iterator.
    type Items: Iterator<Item = Item>;

    /// Returns the item at `point` without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `point < size`
    unsafe fn item_unchecked(self, point: Point) -> Item;

    /// Returns an iterator over items at column `index`, without bounds
    /// checking.
    ///
    /// Callers **MUST** ensure:
    /// - `col < width`
    /// - `start <= end`
    /// - `end <= height`
    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col;

    /// Returns an iterator over items at row `index`, without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `row < height`
    /// - `start <= end`
    /// - `end <= width`
    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row;

    /// Returns an iterator over columns at `index`, without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols;

    /// Returns an iterator over rows at `index`, without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows;

    /// Returns an iterator over items at `index`, without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items;

    /// Returns the item at `point`, or `None` if `point >= size`.
    fn item(self, point: Point) -> Option<Item> {
        if point < self.size() {
            // SAFETY:
            // point < size
            Some(unsafe { self.item_unchecked(point) })
        } else {
            None
        }
    }

    /// Returns an iterator over items at column `index`,
    /// or `None` if out of bounds.
    fn col(self, index: impl Index1D) -> Option<Self::Col> {
        let (width, height) = self.size().into();
        let (col, range) = index.checked(width, height)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(col < width);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= height);
        Some(unsafe { self.col_unchecked((col, range)) })
    }

    /// Returns an iterator over items at row `index`,
    /// or `None` if out of bounds.
    fn row(self, index: impl Index1D) -> Option<Self::Row> {
        let (width, height) = self.size().into();
        let (row, range) = index.checked(height, width)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(row < height);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= width);
        Some(unsafe { self.row_unchecked((row, range)) })
    }

    /// Returns an iterator over columns at `index`,
    /// or `None` if out of bounds.
    fn cols(self, index: impl Index2D) -> Option<Self::Cols> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.cols_unchecked((x, y)) })
    }

    /// Returns an iterator over rows at `index`,
    /// or `None` if out of bounds.
    fn rows(self, index: impl Index2D) -> Option<Self::Rows> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.rows_unchecked((x, y)) })
    }

    /// Returns an iterator over items at `index`,
    /// or `None` if out of bounds.
    fn items(self, index: impl Index2D) -> Option<Self::Items> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.items_unchecked((x, y)) })
    }
}

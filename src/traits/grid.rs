use crate::*;

/// Grid trait.
pub trait Grid<'a> {
    /// The type of the elements.
    type Cell: 'a;

    /// The type of the column iterator.
    type Col: Iterator<Item = &'a Self::Cell>;

    /// The type of the row iterator.
    type Row: Iterator<Item = &'a Self::Cell>;

    /// The type of the columns iterator.
    type Cols: Iterator<Item = Self::Col>;

    /// The type of the rows iterator.
    type Rows: Iterator<Item = Self::Row>;

    /// The type of the cells iterator.
    type Cells: Iterator<Item = &'a Self::Cell>;

    /// Returns the [`Size`](crate::Size) of this `grid`.
    fn size(&self) -> Size<usize>;

    /// Returns the element at `point`,
    /// or `None` if `point >= size`.
    fn cell(&self, point: Point<usize>) -> Option<&Self::Cell> {
        if point < self.size() {
            // SAFETY:
            // point < size
            Some(unsafe { self.cell_unchecked(point) })
        } else {
            None
        }
    }

    /// Returns an iterator over elements at `col` in `range`,
    /// or `None` if `col >= width`.
    fn col(&'a self, index: impl Index1D) -> Option<Self::Col> {
        let (width, height) = self.size().into();
        let (col, range) = index.checked(width, height)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(col < width);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= height);
        Some(unsafe { self.col_unchecked((col, range)) })
    }

    /// Returns an iterator over elements at `row` in `range`,
    /// or `None` if `row >= height`.
    fn row(&'a self, index: impl Index1D) -> Option<Self::Row> {
        let (width, height) = self.size().into();
        let (row, range) = index.checked(height, width)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(row < height);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= width);
        Some(unsafe { self.row_unchecked((row, range)) })
    }

    fn cols(&'a self, index: impl Index2D) -> Option<Self::Cols> {
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

    fn rows(&'a self, index: impl Index2D) -> Option<Self::Rows> {
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

    fn cells(&'a self, index: impl Index2D) -> Option<Self::Cells> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.cells_unchecked((x, y)) })
    }

    /// Returns the element at `point` without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `point < size`
    unsafe fn cell_unchecked(&self, point: Point<usize>) -> &Self::Cell;

    /// Returns an iterator over elements at `col` in `range` without bounds
    /// checking.
    ///
    /// Callers **MUST** ensure:
    /// - `col < width`
    /// - `start <= end`
    /// - `end <= height`
    unsafe fn col_unchecked(&'a self, index: impl Index1D) -> Self::Col;

    /// Returns an iterator over elements at `row` in `range` without bounds
    /// checking.
    ///
    /// Callers **MUST** ensure:
    /// - `row < height`
    /// - `start <= end`
    /// - `end <= width`
    unsafe fn row_unchecked(&'a self, index: impl Index1D) -> Self::Row;

    /// Returns an iterator over columns in `range` without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn cols_unchecked(&'a self, index: impl Index2D) -> Self::Cols;

    /// Returns an iterator over rows in `range` without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn rows_unchecked(&'a self, index: impl Index2D) -> Self::Rows;

    /// Returns an iterator over cells in `range` without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn cells_unchecked(&'a self, index: impl Index2D) -> Self::Cells;
}

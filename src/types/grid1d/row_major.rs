use crate::*;

pub type RowMajor1D<Cell, Collection> = Grid1D<RowMajor, Cell, Collection>;

/// ### Methods
/// @see [`RowMajor1D`](crate::RowMajor1D).
impl<Cell, Collection: AsRef<[Cell]>> RowMajor1D<Cell, Collection> {
    pub unsafe fn get_row_unchecked(&self, index: impl Index1D) -> &[Cell] {
        self.cells
            .as_ref()
            .get_unchecked(RowMajor::row_unchecked(self.size, index))
    }

    pub fn get_row(&self, index: impl Index1D) -> Option<&[Cell]> {
        let range = RowMajor::row(self.size, index)?;
        let cells = self.cells.as_ref();

        // SAFETY:
        // RowMajor::row does the bounds checking
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= self.size.width * self.size.height);
        Some(unsafe { cells.get_unchecked(range) })
    }
}

impl<'a, Cell: 'a, Collection: 'a + AsRef<[Cell]>> IGrid<'a> for RowMajor1D<Cell, Collection> {
    type Cell = Cell;
    type Cells = std::iter::Flatten<Self::Rows>;
    type Col = ColHelper<'a, Self>;
    type Cols = ColsHelper<'a, Self>;
    type Row = std::slice::Iter<'a, Cell>;
    type Rows = RowsHelper<'a, Self>;

    fn size(&self) -> Size<usize> {
        self.size
    }

    unsafe fn cell_unchecked(&self, point: Point<usize>) -> &Cell {
        self.cells
            .as_ref()
            .get_unchecked(RowMajor::cell_unchecked(self.size, point))
    }

    unsafe fn row_unchecked(&'a self, index: impl Index1D) -> Self::Row {
        self.get_row_unchecked(index).iter()
    }

    unsafe fn col_unchecked(&'a self, index: impl Index1D) -> Self::Col {
        ColHelper::new_unchecked(self, index)
    }

    unsafe fn rows_unchecked(&'a self, index: impl Index2D) -> Self::Rows {
        RowsHelper::new_unchecked(self, index)
    }

    unsafe fn cols_unchecked(&'a self, index: impl Index2D) -> Self::Cols {
        ColsHelper::new_unchecked(self, index)
    }

    unsafe fn cells_unchecked(&'a self, index: impl Index2D) -> Self::Cells {
        self.rows_unchecked(index).flatten()
    }
}

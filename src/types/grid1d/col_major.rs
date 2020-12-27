use crate::*;

pub type ColMajor1D<Cell, Collection> = Grid1D<ColMajor, Cell, Collection>;

/// ### `ColMajor1D` methods
/// @see [`ColMajor1D`](crate::ColMajor1D).
impl<Cell, Collection> ColMajor1D<Cell, Collection> {
    pub fn get_col(&self, index: impl Index1D) -> Option<&[Cell]>
    where
        Collection: AsRef<[Cell]>,
    {
        let range = ColMajor::col(self.size, index)?;
        let cells = self.as_ref();

        // SAFETY:
        // ColMajor::col does the bounds checking
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= cells.len());
        Some(unsafe { cells.get_unchecked(range) })
    }

    pub unsafe fn get_col_unchecked(&self, index: impl Index1D) -> &[Cell]
    where
        Collection: AsRef<[Cell]>,
    {
        self.as_ref()
            .get_unchecked(ColMajor::col_unchecked(self.size, index))
    }

    pub fn get_col_mut(&mut self, index: impl Index1D) -> Option<&mut [Cell]>
    where
        Collection: AsMut<[Cell]>,
    {
        let range = ColMajor::col(self.size, index)?;
        let cells = self.as_mut();

        // SAFETY:
        // ColMajor::col does the bounds checking
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= cells.len());
        Some(unsafe { cells.get_unchecked_mut(range) })
    }

    pub unsafe fn get_col_unchecked_mut(&mut self, index: impl Index1D) -> &mut [Cell]
    where
        Collection: AsMut<[Cell]>,
    {
        self.cells
            .as_mut()
            .get_unchecked_mut(ColMajor::col_unchecked(self.size, index))
    }
}

impl<Cell, Collection> Grid for ColMajor1D<Cell, Collection> {
    type Cell = Cell;

    fn size(&self) -> Size<usize> {
        self.size
    }
}

impl<'a, Cell: 'a, Collection: 'a + AsRef<[Cell]>> GridRef<'a> for ColMajor1D<Cell, Collection> {
    type Cells = std::iter::Flatten<Self::Cols>;
    type Col = std::slice::Iter<'a, Cell>;
    type Cols = ColsHelper<'a, Self>;
    type Row = RowHelper<'a, Self>;
    type Rows = RowsHelper<'a, Self>;

    unsafe fn cell_unchecked(&self, point: Point<usize>) -> &Cell {
        self.cells
            .as_ref()
            .get_unchecked(RowMajor::cell_unchecked(self.size, point))
    }

    unsafe fn col_unchecked(&'a self, index: impl Index1D) -> Self::Col {
        self.get_col_unchecked(index).iter()
    }

    unsafe fn row_unchecked(&'a self, index: impl Index1D) -> Self::Row {
        RowHelper::new_unchecked(self, index)
    }

    unsafe fn cols_unchecked(&'a self, index: impl Index2D) -> Self::Cols {
        ColsHelper::new_unchecked(self, index)
    }

    unsafe fn rows_unchecked(&'a self, index: impl Index2D) -> Self::Rows {
        RowsHelper::new_unchecked(self, index)
    }

    unsafe fn cells_unchecked(&'a self, index: impl Index2D) -> Self::Cells {
        self.cols_unchecked(index).flatten()
    }
}

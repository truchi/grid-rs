use crate::*;

pub type RowMajor1D<Cell, Collection> = Grid1D<RowMajor, Cell, Collection>;

/// ### `RowMajor1D` methods
/// @see [`RowMajor1D`](crate::RowMajor1D).
impl<Cell, Collection> RowMajor1D<Cell, Collection> {
    pub fn get_row(&self, index: impl Index1D) -> Option<&[Cell]>
    where
        Collection: AsRef<[Cell]>,
    {
        let range = RowMajor::row(self.size, index)?;
        let cells = self.as_ref();

        // SAFETY:
        // RowMajor::row does the bounds checking
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= cells.len());
        Some(unsafe { cells.get_unchecked(range) })
    }

    pub unsafe fn get_row_unchecked(&self, index: impl Index1D) -> &[Cell]
    where
        Collection: AsRef<[Cell]>,
    {
        self.as_ref()
            .get_unchecked(RowMajor::row_unchecked(self.size, index))
    }

    pub fn get_row_mut(&mut self, index: impl Index1D) -> Option<&mut [Cell]>
    where
        Collection: AsMut<[Cell]>,
    {
        let range = RowMajor::row(self.size, index)?;
        let cells = self.as_mut();

        // SAFETY:
        // RowMajor::row does the bounds checking
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= cells.len());
        Some(unsafe { cells.get_unchecked_mut(range) })
    }

    pub unsafe fn get_row_unchecked_mut(&mut self, index: impl Index1D) -> &mut [Cell]
    where
        Collection: AsMut<[Cell]>,
    {
        self.cells
            .as_mut()
            .get_unchecked_mut(RowMajor::row_unchecked(self.size, index))
    }
}

impl<Cell, Collection> Grid for RowMajor1D<Cell, Collection> {
    type Cell = Cell;

    fn size(&self) -> Size<usize> {
        self.size
    }
}

impl<'a, Cell: 'a, Collection: 'a + AsRef<[Cell]>> GridRef<'a> for RowMajor1D<Cell, Collection> {
    type Cells = std::iter::Flatten<Self::Rows>;
    type Col = ColHelper<'a, Self>;
    type Cols = ColsHelper<'a, Self>;
    type Row = std::slice::Iter<'a, Cell>;
    type Rows = RowsHelper<'a, Self>;

    unsafe fn cell_unchecked(&self, point: Point<usize>) -> &Cell {
        self.cells
            .as_ref()
            .get_unchecked(RowMajor::cell_unchecked(self.size, point))
    }

    unsafe fn col_unchecked(&'a self, index: impl Index1D) -> Self::Col {
        ColHelper::new_unchecked(self, index)
    }

    unsafe fn row_unchecked(&'a self, index: impl Index1D) -> Self::Row {
        self.get_row_unchecked(index).iter()
    }

    unsafe fn cols_unchecked(&'a self, index: impl Index2D) -> Self::Cols {
        ColsHelper::new_unchecked(self, index)
    }

    unsafe fn rows_unchecked(&'a self, index: impl Index2D) -> Self::Rows {
        RowsHelper::new_unchecked(self, index)
    }

    unsafe fn cells_unchecked(&'a self, index: impl Index2D) -> Self::Cells {
        self.rows_unchecked(index).flatten()
    }
}

impl<'a, Cell: 'a, Collection: 'a + AsMut<[Cell]>> GridMut<'a> for RowMajor1D<Cell, Collection> {
    type CellsMut = std::iter::Flatten<Self::RowsMut>;
    type ColMut = ColMutHelper<'a, Self>;
    type ColsMut = ColsMutHelper<'a, Self>;
    type RowMut = std::slice::IterMut<'a, Cell>;
    type RowsMut = RowsMutHelper<'a, Self>;

    unsafe fn cell_unchecked_mut(&mut self, point: Point<usize>) -> &mut Cell {
        self.cells
            .as_mut()
            .get_unchecked_mut(RowMajor::cell_unchecked(self.size, point))
    }

    unsafe fn col_unchecked_mut(&'a mut self, index: impl Index1D) -> Self::ColMut {
        ColMutHelper::new_unchecked(self, index)
    }

    unsafe fn row_unchecked_mut(&'a mut self, index: impl Index1D) -> Self::RowMut {
        self.get_row_unchecked_mut(index).iter_mut()
    }

    unsafe fn cols_unchecked_mut(&'a mut self, index: impl Index2D) -> Self::ColsMut {
        ColsMutHelper::new_unchecked(self, index)
    }

    unsafe fn rows_unchecked_mut(&'a mut self, index: impl Index2D) -> Self::RowsMut {
        RowsMutHelper::new_unchecked(self, index)
    }

    unsafe fn cells_unchecked_mut(&'a mut self, index: impl Index2D) -> Self::CellsMut {
        self.rows_unchecked_mut(index).flatten()
    }
}

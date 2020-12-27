use crate::*;

pub trait GridMut<'a>: Grid<'a> {
    type ColMut: Iterator<Item = &'a mut Self::Cell>;
    type RowMut: Iterator<Item = &'a mut Self::Cell>;
    type ColsMut: Iterator<Item = Self::ColMut>;
    type RowsMut: Iterator<Item = Self::RowMut>;
    type CellsMut: Iterator<Item = &'a mut Self::Cell>;

    fn cell_mut(&mut self, point: Point<usize>) -> Option<&mut Self::Cell> {
        if point < self.size() {
            // SAFETY:
            // point < size
            Some(unsafe { self.cell_mut_unchecked(point) })
        } else {
            None
        }
    }

    fn col_mut(&'a mut self, index: impl Index1D) -> Option<Self::ColMut> {
        let (width, height) = self.size().into();
        let (col, range) = index.checked(width, height)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(col < width);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= height);
        Some(unsafe { self.col_mut_unchecked((col, range)) })
    }

    fn row_mut(&'a mut self, index: impl Index1D) -> Option<Self::RowMut> {
        let (width, height) = self.size().into();
        let (row, range) = index.checked(height, width)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(row < height);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= width);
        Some(unsafe { self.row_mut_unchecked((row, range)) })
    }

    fn cols_mut(&'a mut self, index: impl Index2D) -> Option<Self::ColsMut> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.cols_mut_unchecked((x, y)) })
    }

    fn rows_mut(&'a mut self, index: impl Index2D) -> Option<Self::RowsMut> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.rows_mut_unchecked((x, y)) })
    }

    fn cells_mut(&'a mut self, index: impl Index2D) -> Option<Self::CellsMut> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.cells_mut_unchecked((x, y)) })
    }

    unsafe fn cell_mut_unchecked(&mut self, point: Point<usize>) -> &mut Self::Cell;

    unsafe fn col_mut_unchecked(&'a mut self, index: impl Index1D) -> Self::ColMut;

    unsafe fn row_mut_unchecked(&'a mut self, index: impl Index1D) -> Self::RowMut;

    unsafe fn cols_mut_unchecked(&'a mut self, index: impl Index2D) -> Self::ColsMut;

    unsafe fn rows_mut_unchecked(&'a mut self, index: impl Index2D) -> Self::RowsMut;

    unsafe fn cells_mut_unchecked(&'a mut self, index: impl Index2D) -> Self::CellsMut;
}

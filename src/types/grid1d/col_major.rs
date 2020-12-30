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
            .get_unchecked(ColMajor::cell_unchecked(self.size, point))
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

impl<'a, Cell: 'a, Collection: 'a + AsMut<[Cell]>> GridMut<'a> for ColMajor1D<Cell, Collection> {
    type CellsMut = std::iter::Flatten<Self::ColsMut>;
    type ColMut = std::slice::IterMut<'a, Cell>;
    type ColsMut = ColsMutHelper<'a, Self>;
    type RowMut = RowMutHelper<'a, Self>;
    type RowsMut = RowsMutHelper<'a, Self>;

    unsafe fn cell_unchecked_mut(&mut self, point: Point<usize>) -> &mut Cell {
        self.cells
            .as_mut()
            .get_unchecked_mut(ColMajor::cell_unchecked(self.size, point))
    }

    unsafe fn col_unchecked_mut(&'a mut self, index: impl Index1D) -> Self::ColMut {
        self.get_col_unchecked_mut(index).iter_mut()
    }

    unsafe fn row_unchecked_mut(&'a mut self, index: impl Index1D) -> Self::RowMut {
        RowMutHelper::new_unchecked(self, index)
    }

    unsafe fn cols_unchecked_mut(&'a mut self, index: impl Index2D) -> Self::ColsMut {
        ColsMutHelper::new_unchecked(self, index)
    }

    unsafe fn rows_unchecked_mut(&'a mut self, index: impl Index2D) -> Self::RowsMut {
        RowsMutHelper::new_unchecked(self, index)
    }

    unsafe fn cells_unchecked_mut(&'a mut self, index: impl Index2D) -> Self::CellsMut {
        self.cols_unchecked_mut(index).flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DEBUG: bool = false;
    const COLS: usize = 3;
    const ROWS: usize = 2;
    const LEN: usize = COLS * ROWS;

    struct Expected {
        cols:  [[usize; ROWS]; COLS],
        rows:  [[usize; COLS]; ROWS],
        cells: [usize; LEN],
    }

    fn data() -> (ColMajor1D<usize, [usize; LEN]>, Expected) {
        assert!(COLS != 0, "COLS is 0");
        assert!(ROWS != 0, "ROWS is 0");

        let mut cols = [[0; ROWS]; COLS];
        let mut rows = [[0; COLS]; ROWS];
        let mut cells = [0; LEN];

        let mut i = 0;
        for col in 0..COLS {
            for row in 0..ROWS {
                cols[col][row] = i;
                rows[row][col] = i;
                cells[i] = i;

                i += 1;
            }
        }

        if DEBUG {
            println!("========= DEBUG =========");
            dbg!(cols);
            dbg!(rows);
            dbg!(cells);
            println!("========= /DEBUG =========");
            assert!(false);
        }

        (
            Grid1D::<ColMajor, usize, [usize; LEN]>::new((COLS, ROWS), cells).unwrap(),
            Expected { cols, rows, cells },
        )
    }

    #[test]
    fn slices() {
        let (mut grid, Expected { mut cols, .. }) = data();

        for i in 0..COLS {
            assert_eq!(unsafe { grid.get_col_unchecked(i) }, &cols[i][..]);
            assert_eq!(unsafe { grid.get_col_unchecked_mut(i) }, &mut cols[i][..]);
            assert_eq!(grid.get_col(i), Some(&cols[i][..]));
            assert_eq!(grid.get_col_mut(i), Some(&mut cols[i][..]));
        }
        assert_eq!(grid.get_col(COLS), None);
        assert_eq!(grid.get_col_mut(COLS), None);
    }

    #[test]
    fn iters() {
        let (mut grid, Expected { cols, rows, cells }) = data();

        fn collect<'a>(it: impl Iterator<Item = &'a usize>) -> Vec<usize> {
            it.map(|u| *u).collect()
        }

        fn collect_mut<'a>(it: impl Iterator<Item = &'a mut usize>) -> Vec<usize> {
            it.map(|u| *u).collect()
        }

        macro_rules! iters {
            (
                ($($args:tt)*) ($($expected:tt)*)
                $unchecked:ident $unchecked_mut:ident $checked:ident $checked_mut:ident
            ) => {
                assert_eq!(collect(unsafe { grid.$unchecked($($args)*) }), $($expected)*[..]);
                assert_eq!(collect_mut(unsafe { grid.$unchecked_mut($($args)*) }), $($expected)*[..]);
                assert_eq!(collect(grid.$checked($($args)*).unwrap()), $($expected)*[..]);
                assert_eq!(collect_mut(grid.$checked_mut($($args)*).unwrap()), $($expected)*[..]);
            };
            (None $(($($none:tt)*))*) => { $(
                assert!(grid.$($none)*.is_none());
            )* };
        }

        for x in 0..COLS {
            for y in 0..ROWS {
                assert_eq!(unsafe { grid.cell_unchecked((x, y).into()) }, &rows[y][x]);
                assert_eq!(grid.cell((x, y).into()).unwrap(), &rows[y][x]);
            }
        }

        for i in 0..COLS {
            iters!((i) (&cols[i]) col_unchecked col_unchecked_mut col col_mut);
        }

        for i in 0..ROWS {
            iters!((i) (&rows[i]) row_unchecked row_unchecked_mut row row_mut);
        }

        iters!(((.., ..)) (&cells) cells_unchecked cells_unchecked_mut cells cells_mut);

        iters!(None(col(COLS))(col_mut(COLS))(row(ROWS))(row_mut(ROWS)));
    }
}

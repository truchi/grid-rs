mod col_major;
mod row_major;

pub use col_major::*;
pub use row_major::*;

use crate::*;
use std::{convert::TryFrom, marker::PhantomData};

/// Error type for [`Grid1D`](crate::Grid1D) constructors.
#[derive(Copy, Clone, Debug)]
pub enum Grid1DError<T> {
    /// `width * height > usize::MAX`.
    Overflow(Size<usize>, T),
    /// `width * height != len`.
    Mismatch(Size<usize>, T),
}

/// 2D [`Grid1D`](crate::Grid1D).
///
/// @see [`ColMajor1D`](crate::ColMajor1D) and
/// [`RowMajor1D`](crate::RowMajor1D).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Grid1D<Major, Cell, Collection> {
    phantom: PhantomData<(Major, Cell)>,
    size:    Size<usize>,
    cells:   Collection,
}

/// ### Constructors
impl<Major, Cell, Collection> Grid1D<Major, Cell, Collection> {
    /// Creates a new [`Grid1D`](crate::Grid1D)
    /// or returns a [`Grid1DError`](Grid1DError).
    pub fn new<S: Into<Size<usize>>>(
        size: S,
        cells: Collection,
    ) -> Result<Self, Grid1DError<Collection>>
    where
        Collection: AsRef<[Cell]>,
    {
        let size = size.into();

        match size.width.checked_mul(size.height) {
            None => Err(Grid1DError::Overflow(size, cells)),
            Some(area) =>
                if area != cells.as_ref().len() {
                    Err(Grid1DError::Mismatch(size, cells))
                } else {
                    Ok(Self {
                        size,
                        cells,
                        phantom: PhantomData,
                    })
                },
        }
    }

    /// Creates a new [`Grid1D`](crate::Grid1D)
    /// or returns a [`Grid1DError`](Grid1DError).
    pub fn new_mut<S: Into<Size<usize>>>(
        size: S,
        mut cells: Collection,
    ) -> Result<Self, Grid1DError<Collection>>
    where
        Collection: AsMut<[Cell]>,
    {
        let size = size.into();

        match size.width.checked_mul(size.height) {
            None => Err(Grid1DError::Overflow(size, cells)),
            Some(area) =>
                if area != cells.as_mut().len() {
                    Err(Grid1DError::Mismatch(size, cells))
                } else {
                    Ok(Self {
                        size,
                        cells,
                        phantom: PhantomData,
                    })
                },
        }
    }
}

/// ### Methods
impl<Major, Cell, Collection: AsRef<[Cell]>> Grid1D<Major, Cell, Collection> {
    /// Returns the [`Size`](crate::Size).
    pub fn size(&self) -> Size<usize> {
        self.size
    }
}

impl<Major, Cell, Collection: AsRef<[Cell]>, S: Into<Size<usize>>> TryFrom<(S, Collection)>
    for Grid1D<Major, Cell, Collection>
{
    type Error = Grid1DError<Collection>;

    fn try_from((size, cells): (S, Collection)) -> Result<Self, Self::Error> {
        Self::new(size, cells)
    }
}

impl<Major, Cell, Collection: AsRef<[Cell]>> AsRef<[Cell]> for Grid1D<Major, Cell, Collection> {
    fn as_ref(&self) -> &[Cell] {
        self.cells.as_ref()
    }
}

impl<Major, Cell, Collection: AsMut<[Cell]>> AsMut<[Cell]> for Grid1D<Major, Cell, Collection> {
    fn as_mut(&mut self) -> &mut [Cell] {
        self.cells.as_mut()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DEBUG: bool = false;
    const COLS: usize = 3;
    const ROWS: usize = 2;
    const LEN: usize = COLS * ROWS;

    pub struct Expected {
        cols:  [[usize; ROWS]; COLS],
        rows:  [[usize; COLS]; ROWS],
        cells: [usize; LEN],
    }

    pub fn colmajor() -> (ColMajor1D<usize, [usize; LEN]>, Expected) {
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

    fn collect<'a>(it: impl Iterator<Item = &'a usize>) -> Vec<usize> {
        it.map(|u| *u).collect()
    }

    fn collect_mut<'a>(it: impl Iterator<Item = &'a mut usize>) -> Vec<usize> {
        it.map(|u| *u).collect()
    }

    pub fn grid_ref<'a, T: Grid<Cell = usize> + GridRef<'a>>(
        grid: &'a T,
        Expected { cols, rows, cells }: Expected,
    ) where
        T::Cell: 'a,
    {
        macro_rules! iters {
            (
                ($($args:tt)*) ($($expected:tt)*)
                $unchecked:ident $checked:ident
            ) => {
                assert_eq!(collect(unsafe { grid.$unchecked($($args)*) }), $($expected)*[..]);
                assert_eq!(collect(grid.$checked($($args)*).unwrap()), $($expected)*[..]);
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
            iters!((i) (&cols[i]) col_unchecked col);
        }

        for i in 0..ROWS {
            iters!((i) (&rows[i]) row_unchecked row);
        }

        iters!(((.., ..)) (&cells) cells_unchecked cells);

        iters!(None(col(COLS))(row(ROWS)));
    }

    pub fn grid_mut<T: Grid<Cell = usize>>(grid: &mut T, Expected { cols, rows, cells }: Expected)
    where
        T: for<'a> GridMut<'a>,
    {
        macro_rules! iters {
            (
                ($($args:tt)*) ($($expected:tt)*)
                $unchecked:ident $checked:ident
            ) => {
                assert_eq!(collect_mut(unsafe { grid.$unchecked($($args)*) }), $($expected)*[..]);
                assert_eq!(collect_mut(grid.$checked($($args)*).unwrap()), $($expected)*[..]);
            };
            (None $(($($none:tt)*))*) => { $(
                assert!(grid.$($none)*.is_none());
            )* };
        }

        for x in 0..COLS {
            for y in 0..ROWS {
                assert_eq!(
                    unsafe { grid.cell_unchecked_mut((x, y).into()) },
                    &rows[y][x]
                );
                assert_eq!(grid.cell_mut((x, y).into()).unwrap(), &rows[y][x]);
            }
        }

        for i in 0..COLS {
            iters!((i) (&cols[i]) col_unchecked_mut col_mut);
        }

        for i in 0..ROWS {
            iters!((i) (&rows[i]) row_unchecked_mut row_mut);
        }

        iters!(((.., ..)) (&cells) cells_unchecked_mut cells_mut);

        iters!(None(col_mut(COLS))(row_mut(ROWS)));
    }
}

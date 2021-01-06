mod col_major;
mod row_major;

pub use col_major::*;
pub use row_major::*;

use crate::*;
use std::marker::PhantomData;

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
pub struct Grid1D<M, I, T> {
    size:    M,
    items:   T,
    phantom: PhantomData<I>,
}

/// ### Constructors
impl<M: Major, I, T> Grid1D<M, I, T> {
    /// Creates a new [`Grid1D`](crate::Grid1D)
    /// or returns a [`Grid1DError`](Grid1DError).
    pub fn new(size: Size<usize>, items: T) -> Result<Self, Grid1DError<T>>
    where
        T: AsRef<[I]>,
    {
        match size.width.checked_mul(size.height) {
            None => Err(Grid1DError::Overflow(size, items)),
            Some(len) =>
                if len != items.as_ref().len() {
                    Err(Grid1DError::Mismatch(size, items))
                } else {
                    Ok(Self {
                        size: size.into(),
                        items,
                        phantom: PhantomData,
                    })
                },
        }
    }

    /// Creates a new [`Grid1D`](crate::Grid1D)
    /// or returns a [`Grid1DError`](Grid1DError).
    pub fn new_mut(size: Size<usize>, mut items: T) -> Result<Self, Grid1DError<T>>
    where
        T: AsMut<[I]>,
    {
        match size.width.checked_mul(size.height) {
            None => Err(Grid1DError::Overflow(size, items)),
            Some(len) =>
                if len != items.as_mut().len() {
                    Err(Grid1DError::Mismatch(size, items))
                } else {
                    Ok(Self {
                        size: size.into(),
                        items,
                        phantom: PhantomData,
                    })
                },
        }
    }
}

impl<M, I, T: AsRef<[I]>> AsRef<[I]> for Grid1D<M, I, T> {
    fn as_ref(&self) -> &[I] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[I]>> AsMut<[I]> for Grid1D<M, I, T> {
    fn as_mut(&mut self) -> &mut [I] {
        self.items.as_mut()
    }
}

impl<M: Major, I, T> WithSize for Grid1D<M, I, T> {
    fn size(&self) -> Size<usize> {
        self.size.into()
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

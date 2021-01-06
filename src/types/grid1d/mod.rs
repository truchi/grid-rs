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
    Overflow(Size, T),
    /// `width * height != len`.
    Mismatch(Size, T),
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
    pub fn new(size: Size, items: T) -> Result<Self, Grid1DError<T>>
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
    pub fn new_mut(size: Size, mut items: T) -> Result<Self, Grid1DError<T>>
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
    fn size(&self) -> Size {
        self.size.into()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug)]
    pub struct Expected {
        cols:  Vec<Vec<usize>>,
        rows:  Vec<Vec<usize>>,
        items: Vec<usize>,
    }

    impl Expected {
        pub fn with_capacity(size: Size) -> Self {
            let (width, height) = size.into();

            let mut cols = Vec::with_capacity(width);
            let mut rows = Vec::with_capacity(height);
            let items = Vec::with_capacity(width * height);

            for _ in 0..width {
                cols.push(Vec::with_capacity(height));
            }

            for _ in 0..height {
                rows.push(Vec::with_capacity(width));
            }

            Self { cols, rows, items }
        }
    }

    pub fn col_1d(size: Size) -> (ColMajor1D<usize, Vec<usize>>, Expected) {
        let (width, height) = size.into();

        let Expected {
            mut cols,
            mut rows,
            mut items,
        } = Expected::with_capacity(size);

        let mut i = 0;
        for col in 0..width {
            for row in 0..height {
                cols[col].push(i);
                rows[row].push(i);
                items.push(i);

                i += 1;
            }
        }

        (
            ColMajor1D::<usize, Vec<usize>>::new(size, items.clone()).unwrap(),
            Expected { cols, rows, items },
        )
    }

    pub fn row_1d(size: Size) -> (RowMajor1D<usize, Vec<usize>>, Expected) {
        let (width, height) = size.into();

        let Expected {
            mut cols,
            mut rows,
            mut items,
        } = Expected::with_capacity(size);

        let mut i = 0;
        for row in 0..height {
            for col in 0..width {
                cols[col].push(i);
                rows[row].push(i);
                items.push(i);

                i += 1;
            }
        }

        (
            RowMajor1D::<usize, Vec<usize>>::new(size, items.clone()).unwrap(),
            Expected { cols, rows, items },
        )
    }

    fn collect<'a>(it: impl Iterator<Item = &'a usize>) -> Vec<usize> {
        it.map(|u| *u).collect()
    }

    fn collect_mut<'a>(it: impl Iterator<Item = &'a mut usize>) -> Vec<usize> {
        it.map(|u| *u).collect()
    }

    pub fn grid_ref<'a, T>(grid: &'a T, Expected { cols, rows, items }: Expected)
    where
        &'a T: Grid<&'a usize>,
    {
        let (width, height) = grid.size().into();

        // ITEM
        for x in 0..width {
            for y in 0..height {
                let point = (x, y).into();

                assert_eq!(unsafe { grid.item_unchecked(point) }, &rows[y][x]);
                assert_eq!(grid.item(point).unwrap(), &rows[y][x]);
            }
        }

        // COL
        for i in 0..width {
            assert_eq!(collect(unsafe { grid.col_unchecked(i) }), &cols[i][..]);
            assert_eq!(collect(grid.col(i).unwrap()), &cols[i][..]);
        }

        // ROW
        for i in 0..height {
            assert_eq!(collect(unsafe { grid.row_unchecked(i) }), &rows[i][..]);
            assert_eq!(collect(grid.row(i).unwrap()), &rows[i][..]);
        }

        // COLS
        assert_eq!(
            collect(unsafe { grid.cols_unchecked(()) }.flatten()),
            collect(cols.iter().flatten()),
        );
        assert_eq!(
            collect(grid.cols(()).unwrap().flatten()),
            collect(cols.iter().flatten()),
        );

        // ROWS
        assert_eq!(
            collect(unsafe { grid.rows_unchecked(()) }.flatten()),
            collect(rows.iter().flatten()),
        );
        assert_eq!(
            collect(grid.rows(()).unwrap().flatten()),
            collect(rows.iter().flatten()),
        );

        // ITEMS
        assert_eq!(collect(unsafe { grid.items_unchecked(()) }), &items[..]);
        assert_eq!(collect(grid.items(()).unwrap()), &items[..]);
    }
}

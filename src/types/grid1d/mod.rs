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
    Overflow(T),
    /// `width * height != len`.
    Mismatch(T),
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
    /// Creates a new [`Grid1D`](crate::Grid1D), without errors checking.
    pub fn new_unchecked(size: Size, items: T) -> Self {
        Self {
            size: size.into(),
            items,
            phantom: PhantomData,
        }
    }

    /// Creates a new [`Grid1D`](crate::Grid1D)
    /// or returns a [`Grid1DError`](Grid1DError).
    pub fn new(size: Size, items: T) -> Result<Self, Grid1DError<T>>
    where
        T: AsRef<[I]>,
    {
        match size.width.checked_mul(size.height) {
            None => Err(Grid1DError::Overflow(items)),
            Some(len) =>
                if len != items.as_ref().len() {
                    Err(Grid1DError::Mismatch(items))
                } else {
                    Ok(Self::new_unchecked(size, items))
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

    #[derive(Clone, Debug)]
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

    fn collect<'a>(it: impl Iterator<Item = &'a usize>) -> Vec<usize> {
        it.map(|u| *u).collect()
    }

    fn collect_mut<'a>(it: impl Iterator<Item = &'a mut usize>) -> Vec<usize> {
        it.map(|u| *u).collect()
    }

    macro_rules! grid_1d {
        (
            $width:ident $height:ident $col:ident $row:ident
            $($fn:ident $T:ident $major:ident $Major:ident $minor:ident $Minor:ident)*
        ) => { $(
            pub fn $fn(size: Size) -> ($T<usize, Vec<usize>>, Expected) {
                let ($width, $height) = size.into();

                let Expected {
                    mut cols,
                    mut rows,
                    mut items,
                } = Expected::with_capacity(size);

                let mut i = 0;
                for $minor in 0..$Minor {
                    for $major in 0..$Major {
                        cols[$col].push(i);
                        rows[$row].push(i);
                        items.push(i);

                        i += 1;
                    }
                }

                (
                    $T::<usize, Vec<usize>>::new(size, items.clone()).unwrap(),
                    Expected { cols, rows, items },
                )
            }
        )* };
    }

    grid_1d!(width height col row
        col_1d ColMajor1D row height col width
        row_1d RowMajor1D col width row height
    );

    macro_rules! grid {
        ($($fn:ident: ($($mut:tt)?) $collect:ident)*) => { $(
            pub fn $fn<'a, T>(grid: &'a $($mut)? T, Expected { cols, rows, items }: Expected)
            where
                for<'b> &'b $($mut)? T: Grid<&'b $($mut)? usize>,
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
                    assert_eq!($collect(unsafe { grid.col_unchecked(i) }), &cols[i][..]);
                    assert_eq!($collect(grid.col(i).unwrap()), &cols[i][..]);
                }

                // ROW
                for i in 0..height {
                    assert_eq!($collect(unsafe { grid.row_unchecked(i) }), &rows[i][..]);
                    assert_eq!($collect(grid.row(i).unwrap()), &rows[i][..]);
                }

                // COLS
                assert_eq!(
                    $collect(unsafe { grid.cols_unchecked(()) }.flatten()),
                    collect(cols.iter().flatten()),
                );
                assert_eq!(
                    $collect(grid.cols(()).unwrap().flatten()),
                    collect(cols.iter().flatten()),
                );

                // ROWS
                assert_eq!(
                    $collect(unsafe { grid.rows_unchecked(()) }.flatten()),
                    collect(rows.iter().flatten()),
                );
                assert_eq!(
                    $collect(grid.rows(()).unwrap().flatten()),
                    collect(rows.iter().flatten()),
                );

                // ITEMS
                assert_eq!($collect(unsafe { grid.items_unchecked(()) }), &items[..]);
                assert_eq!($collect(grid.items(()).unwrap()), &items[..]);
            }
        )* };
    }

    grid!(
        grid_mut: (mut) collect_mut
        grid_ref: ()    collect
    );
}

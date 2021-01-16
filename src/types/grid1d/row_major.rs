use crate::*;

pub type RowMajor1D<I, T> = Grid1D<RowMajor, I, T>;

/// ### `RowMajor1D` methods
/// @see [`RowMajor1D`](crate::RowMajor1D).
impl<I, T> RowMajor1D<I, T> {
    pub fn row(&self, index: impl Index1D) -> Option<&[I]>
    where
        T: AsRef<[I]>,
    {
        index.get_1d(self)
    }

    pub unsafe fn row_unchecked(&self, index: impl Index1D) -> &[I]
    where
        T: AsRef<[I]>,
    {
        index.get_1d_unchecked(self)
    }

    pub fn row_mut(&mut self, index: impl Index1D) -> Option<&mut [I]>
    where
        T: AsMut<[I]>,
    {
        index.get_mut_1d(self)
    }

    pub unsafe fn row_mut_unchecked(&mut self, index: impl Index1D) -> &mut [I]
    where
        T: AsMut<[I]>,
    {
        index.get_mut_1d_unchecked(self)
    }
}

impl<'a, I, T: AsRef<[I]>> Grid<&'a I> for &'a RowMajor1D<I, T> {
    type Col = ColRef<'a, I, RowMajor1D<I, T>>;
    type Cols = ColsRef<'a, I, RowMajor1D<I, T>>;
    type Items = std::iter::Flatten<Self::Rows>;
    type Row = std::slice::Iter<'a, I>;
    type Rows = RowsRef<'a, I, RowMajor1D<I, T>>;

    unsafe fn item_unchecked(self, point: Point) -> &'a I {
        Index0D::get_1d_unchecked(point, self)
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        ColRef::new_unchecked(self, index)
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        self.row_unchecked(index).iter()
    }

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        ColsRef::new_unchecked(self, index)
    }

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        RowsRef::new_unchecked(self, index)
    }

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.rows_unchecked(index).flatten()
    }
}

impl<'a, I, T: AsMut<[I]>> Grid<&'a mut I> for &'a mut RowMajor1D<I, T> {
    type Col = ColMut<'a, I, RowMajor1D<I, T>>;
    type Cols = ColsMut<'a, I, RowMajor1D<I, T>>;
    type Items = std::iter::Flatten<Self::Rows>;
    type Row = std::slice::IterMut<'a, I>;
    type Rows = RowsMut<'a, I, RowMajor1D<I, T>>;

    unsafe fn item_unchecked(self, point: Point) -> &'a mut I {
        Index0D::get_mut_1d_unchecked(point, self)
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        ColMut::new_unchecked(self, index)
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        self.row_mut_unchecked(index).iter_mut()
    }

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        ColsMut::new_unchecked(self, index)
    }

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        RowsMut::new_unchecked(self, index)
    }

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.rows_unchecked(index).flatten()
    }
}

#[cfg(test)]
pub mod tests {
    use super::{super::tests::*, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn grid() {
        let (mut grid, expected) = row_1d((5, 3).into());

        grid_ref(&grid, expected.clone());
        grid_mut(&mut grid, expected);
    }
}

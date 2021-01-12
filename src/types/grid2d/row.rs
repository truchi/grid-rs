use crate::*;

pub type RowMajor2D<I, T, U> = Grid2D<RowMajor, I, T, U>;

/// ### `RowMajor2D` methods
/// @see [`RowMajor2D`](crate::RowMajor2D).
impl<I, T, U> RowMajor2D<I, T, U> {
    pub fn row(&self, index: impl Index1D) -> Option<&[I]>
    where
        T: AsRef<[U]>,
        U: AsRef<[I]>,
    {
        let index = index.row(self.size().into())?;

        // SAFETY:
        // Index1D::checked does the bounds checking
        Some(unsafe { self.row_unchecked(index) })
    }

    pub unsafe fn row_unchecked(&self, index: impl Index1D) -> &[I]
    where
        T: AsRef<[U]>,
        U: AsRef<[I]>,
    {
        let (row, range) = index.row_unchecked(self.size().into());
        let rows = self.as_ref();

        debug_assert!(row < rows.len());
        let row = rows.get_unchecked(row).as_ref();

        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= row.len());
        row.get_unchecked(range)
    }

    pub fn row_mut(&mut self, index: impl Index1D) -> Option<&mut [I]>
    where
        T: AsMut<[U]>,
        U: AsMut<[I]>,
    {
        let index = index.row(self.size())?;

        // SAFETY:
        // Index1D::checked does the bounds checking
        Some(unsafe { self.row_mut_unchecked(index) })
    }

    pub unsafe fn row_mut_unchecked(&mut self, index: impl Index1D) -> &mut [I]
    where
        T: AsMut<[U]>,
        U: AsMut<[I]>,
    {
        let (row, range) = index.row_unchecked(self.size());
        let rows = self.as_mut();

        debug_assert!(row < rows.len());
        let row = rows.get_unchecked_mut(row).as_mut();

        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= row.len());
        row.get_unchecked_mut(range)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> Grid<&'a I> for &'a RowMajor2D<I, T, U> {
    type Col = ColRef<'a, I, RowMajor2D<I, T, U>>;
    type Cols = ColsRef<'a, I, RowMajor2D<I, T, U>>;
    type Items = std::iter::Flatten<Self::Cols>;
    type Row = std::slice::Iter<'a, I>;
    type Rows = RowsRef<'a, I, RowMajor2D<I, T, U>>;

    unsafe fn item_unchecked(self, point: Point) -> &'a I {
        let (x, y) = point.into();

        self.as_ref().get_unchecked(y).as_ref().get_unchecked(x)
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
        self.cols_unchecked(index).flatten()
    }
}

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> Grid<&'a mut I> for &'a mut RowMajor2D<I, T, U> {
    type Col = ColMut<'a, I, RowMajor2D<I, T, U>>;
    type Cols = ColsMut<'a, I, RowMajor2D<I, T, U>>;
    type Items = std::iter::Flatten<Self::Cols>;
    type Row = std::slice::IterMut<'a, I>;
    type Rows = RowsMut<'a, I, RowMajor2D<I, T, U>>;

    unsafe fn item_unchecked(self, point: Point) -> &'a mut I {
        let (x, y) = point.into();

        self.as_mut()
            .get_unchecked_mut(y)
            .as_mut()
            .get_unchecked_mut(x)
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
        self.cols_unchecked(index).flatten()
    }
}

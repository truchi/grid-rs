use crate::*;

pub type ColMajor2D<I, T, U> = Grid2D<ColMajor, I, T, U>;

/// ### `ColMajor2D` methods
/// @see [`ColMajor2D`](crate::ColMajor2D).
impl<I, T, U> ColMajor2D<I, T, U> {
    pub fn col(&self, index: impl Index1D) -> Option<&[I]>
    where
        T: AsRef<[U]>,
        U: AsRef<[I]>,
    {
        let index = index.checked(self.size.width, self.size.height)?;

        // SAFETY:
        // Index1D::checked does the bounds checking
        Some(unsafe { self.col_unchecked(index) })
    }

    pub unsafe fn col_unchecked(&self, index: impl Index1D) -> &[I]
    where
        T: AsRef<[U]>,
        U: AsRef<[I]>,
    {
        let (col, range) = index.unchecked(self.size.height);
        let cols = self.as_ref();

        debug_assert!(col < cols.len());
        let col = cols.get_unchecked(col).as_ref();

        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= col.len());
        col.get_unchecked(range)
    }

    pub fn col_mut(&mut self, index: impl Index1D) -> Option<&mut [I]>
    where
        T: AsMut<[U]>,
        U: AsMut<[I]>,
    {
        let index = index.checked(self.size.width, self.size.height)?;

        // SAFETY:
        // Index1D::checked does the bounds checking
        Some(unsafe { self.col_mut_unchecked(index) })
    }

    pub unsafe fn col_mut_unchecked(&mut self, index: impl Index1D) -> &mut [I]
    where
        T: AsMut<[U]>,
        U: AsMut<[I]>,
    {
        let (col, range) = index.unchecked(self.size.height);
        let cols = self.as_mut();

        debug_assert!(col < cols.len());
        let col = cols.get_unchecked_mut(col).as_mut();

        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= col.len());
        col.get_unchecked_mut(range)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> Grid<&'a I> for &'a ColMajor2D<I, T, U> {
    type Col = std::slice::Iter<'a, I>;
    type Cols = ColsRef<'a, I, ColMajor2D<I, T, U>>;
    type Items = std::iter::Flatten<Self::Cols>;
    type Row = RowRef<'a, I, ColMajor2D<I, T, U>>;
    type Rows = RowsRef<'a, I, ColMajor2D<I, T, U>>;

    unsafe fn item_unchecked(self, point: Point) -> &'a I {
        let (x, y) = point.into();

        self.as_ref().get_unchecked(x).as_ref().get_unchecked(y)
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        self.col_unchecked(index).iter()
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        RowRef::new_unchecked(self, index)
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

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> Grid<&'a mut I> for &'a mut ColMajor2D<I, T, U> {
    type Col = std::slice::IterMut<'a, I>;
    type Cols = ColsMut<'a, I, ColMajor2D<I, T, U>>;
    type Items = std::iter::Flatten<Self::Cols>;
    type Row = RowMut<'a, I, ColMajor2D<I, T, U>>;
    type Rows = RowsMut<'a, I, ColMajor2D<I, T, U>>;

    unsafe fn item_unchecked(self, point: Point) -> &'a mut I {
        let (x, y) = point.into();

        self.as_mut()
            .get_unchecked_mut(x)
            .as_mut()
            .get_unchecked_mut(y)
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        self.col_mut_unchecked(index).iter_mut()
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        RowMut::new_unchecked(self, index)
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

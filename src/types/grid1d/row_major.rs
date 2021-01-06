use crate::*;

pub type RowMajor1D<I, T> = Grid1D<RowMajor, I, T>;

/// ### `RowMajor1D` methods
/// @see [`RowMajor1D`](crate::RowMajor1D).
impl<I, T> RowMajor1D<I, T> {
    pub fn row(&self, index: impl Index1D) -> Option<&[I]>
    where
        T: AsRef<[I]>,
    {
        let range = self.size.range(index)?;
        let items = self.as_ref();

        // SAFETY:
        // Major::range does the bounds checking
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= items.len());
        Some(unsafe { items.get_unchecked(range) })
    }

    pub unsafe fn row_unchecked(&self, index: impl Index1D) -> &[I]
    where
        T: AsRef<[I]>,
    {
        self.as_ref()
            .get_unchecked(self.size.range_unchecked(index))
    }

    pub fn row_mut(&mut self, index: impl Index1D) -> Option<&mut [I]>
    where
        T: AsMut<[I]>,
    {
        let range = self.size.range(index)?;
        let items = self.as_mut();

        // SAFETY:
        // Major::range does the bounds checking
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= items.len());
        Some(unsafe { items.get_unchecked_mut(range) })
    }

    pub unsafe fn row_mut_unchecked(&mut self, index: impl Index1D) -> &mut [I]
    where
        T: AsMut<[I]>,
    {
        self.items
            .as_mut()
            .get_unchecked_mut(self.size.range_unchecked(index))
    }
}

impl<'a, I, T: AsRef<[I]>> Grid<&'a I> for &'a RowMajor1D<I, T> {
    type Col = ColRef<'a, I, RowMajor1D<I, T>>;
    type Cols = ColsRef<'a, I, RowMajor1D<I, T>>;
    type Items = std::iter::Flatten<Self::Rows>;
    type Row = std::slice::Iter<'a, I>;
    type Rows = RowsRef<'a, I, RowMajor1D<I, T>>;

    unsafe fn item_unchecked(self, point: Point<usize>) -> &'a I {
        self.as_ref()
            .get_unchecked(self.size.index_unchecked(point))
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

    unsafe fn item_unchecked(self, point: Point<usize>) -> &'a mut I {
        self.items
            .as_mut()
            .get_unchecked_mut(self.size.index_unchecked(point))
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

use crate::*;

pub type ColMajor1D<I, T> = Grid1D<ColMajor, I, T>;

/// ### `ColMajor1D` methods
/// @see [`ColMajor1D`](crate::ColMajor1D).
impl<I, T> ColMajor1D<I, T> {
    pub fn col(&self, index: impl Index1D) -> Option<&[I]>
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

    pub unsafe fn col_unchecked(&self, index: impl Index1D) -> &[I]
    where
        T: AsRef<[I]>,
    {
        self.as_ref()
            .get_unchecked(self.size.range_unchecked(index))
    }

    pub fn col_mut(&mut self, index: impl Index1D) -> Option<&mut [I]>
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

    pub unsafe fn col_mut_unchecked(&mut self, index: impl Index1D) -> &mut [I]
    where
        T: AsMut<[I]>,
    {
        self.items
            .as_mut()
            .get_unchecked_mut(self.size.range_unchecked(index))
    }
}

impl<'a, I, T: AsRef<[I]>> Grid<&'a I> for &'a ColMajor1D<I, T> {
    type Col = std::slice::Iter<'a, I>;
    type Cols = ColsRef<'a, I, ColMajor1D<I, T>>;
    type Items = std::iter::Flatten<Self::Cols>;
    type Row = RowRef<'a, I, ColMajor1D<I, T>>;
    type Rows = RowsRef<'a, I, ColMajor1D<I, T>>;

    unsafe fn item_unchecked(self, point: Point) -> &'a I {
        self.as_ref()
            .get_unchecked(self.size.index_unchecked(point))
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

impl<'a, I, T: AsMut<[I]>> Grid<&'a mut I> for &'a mut ColMajor1D<I, T> {
    type Col = std::slice::IterMut<'a, I>;
    type Cols = ColsMut<'a, I, ColMajor1D<I, T>>;
    type Items = std::iter::Flatten<Self::Cols>;
    type Row = RowMut<'a, I, ColMajor1D<I, T>>;
    type Rows = RowsMut<'a, I, ColMajor1D<I, T>>;

    unsafe fn item_unchecked(self, point: Point) -> &'a mut I {
        self.items
            .as_mut()
            .get_unchecked_mut(self.size.index_unchecked(point))
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

#[cfg(test)]
pub mod tests {
    use super::{super::tests::*, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn grid() {
        let (mut grid, expected) = col_1d((5, 3).into());

        grid_ref(&grid, expected.clone());
        grid_mut(&mut grid, expected);
    }
}

pub mod iter;

use crate::*;

/// Creates a grid that repeats elements of type `I` all over a [`Size`](Size)
/// by applying the provided closure.
pub fn repeat_with<I, F: FnMut(Size) -> I>(size: Size, fun: F) -> RepeatWith<F> {
    RepeatWith { size, fun }
}

/// A grid that repeats elements of type `I` all over a [`Size`](Size)
/// by applying the provided closure.
#[derive(Copy, Clone, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct RepeatWith<F> {
    size: Size,
    fun:  F,
}

impl<F> WithSize for RepeatWith<F> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<I, F: FnMut(Point) -> I> Grid for RepeatWith<F> {
    type Item = I;

    unsafe fn item_unchecked(mut self, index: impl Index0D) -> Self::Item {
        (self.fun)(index.unchecked())
    }
}

macro_rules! grid1d {
    ($($Trait:ident<$M:ident> $Assoc:ident $unchecked:ident)*) => { $(
        impl<I, F: FnMut(Point) -> I> $Trait for RepeatWith<F> {
            type $Assoc = iter::Iter1D<$M, F>;

            unsafe fn $unchecked(self, index: impl Index1D) -> Self::$Assoc {
                Self::$Assoc::new(self.fun, index.$unchecked(self.size))
            }
        }
    )* };
}

macro_rules! grid2d {
    ($($Trait:ident<$M:ident> $Assoc:ident $unchecked:ident)*) => { $(
        impl<I, F: Clone + Fn(Point) -> I> $Trait for RepeatWith<F> {
            type $Assoc = iter::Iter2D<$M, F>;

            unsafe fn $unchecked(self, index: impl Index2D) -> Self::$Assoc {
                Self::$Assoc::new(self.fun, index.unchecked(self.size))
            }
        }
    )* };
}

grid1d!(
    GridCol<ColMajor> Col col_unchecked
    GridRow<RowMajor> Row row_unchecked
);

grid2d!(
    GridCols<ColMajor> Cols cols_unchecked
    GridRows<RowMajor> Rows rows_unchecked
);

impl<I, F: FnMut(Point) -> I> GridItems for RepeatWith<F> {
    type Items = iter::Items<F>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        Self::Items::new(self.fun, index.unchecked(self.size))
    }
}

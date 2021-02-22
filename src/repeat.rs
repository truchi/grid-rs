use crate::*;
use std::{
    iter::{repeat as std_repeat, Repeat as StdRepeat, Take},
    ops::Range,
};

/// Creates a grid that repeats an element all over a [`Size`](Size).
pub fn repeat<I: Clone>(size: Size, item: I) -> Repeat<I> {
    Repeat { size, item }
}

/// A grid that repeats an element all over a [`Size`](Size).
///
/// This `struct` is created by [`repeat`](repeat::repeat()).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Repeat<I> {
    size: Size,
    item: I,
}

impl<I> WithSize for Repeat<I> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<I: Clone> Grid for Repeat<I> {
    type Item = I;

    unsafe fn item_unchecked(self, _: impl Index0D) -> Self::Item {
        self.item.clone()
    }
}

macro_rules! grid1d {
    ($($Trait:ident $Assoc:ident $unchecked:ident)*) => { $(
        impl<I: Clone> $Trait for Repeat<I> {
            type $Assoc = Take<StdRepeat<I>>;

            unsafe fn $unchecked(self, index: impl Index1D) -> Self::$Assoc {
                let (_, Range { start, end }) = index.$unchecked(self.size);

                std_repeat(self.item).take(end - start)
            }
        }
    )* };
}

macro_rules! grid2d {
    ($x:ident $y:ident $(
        $Trait:ident $Assoc:ident $unchecked:ident
        ($Item:ident $item_unchecked:ident)
        $main:ident $cross:ident
    )*) => { $(
        impl<I: Clone> $Trait for Repeat<I> {
            type $Assoc = Take<StdRepeat<Self::$Item>>;

            unsafe fn $unchecked(self, index: impl Index2D) -> Self::$Assoc {
                let Point { $x, $y } = index.unchecked(self.size);

                std_repeat(self.$item_unchecked((0, $main))).take($cross.end - $cross.start)
            }
        }
    )* };
}

grid1d!(
    GridCol Col col_unchecked
    GridRow Row row_unchecked
);

grid2d!(x y
    GridCols Cols cols_unchecked (Col col_unchecked) y x
    GridRows Rows rows_unchecked (Row row_unchecked) x y
);

impl<I: Clone> GridItems for Repeat<I> {
    type Items = Take<StdRepeat<Self::Item>>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        let Point { x, y } = index.unchecked(self.size);

        std_repeat(self.item).take((x.end - x.start) * (y.end - y.start))
    }
}

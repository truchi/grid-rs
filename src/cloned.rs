use crate::*;
use std::{
    iter::{Cloned as StdCloned, Copied as StdCopied, Map},
    ops::{Deref, DerefMut},
};

macro_rules! grid1d {
    ($Type:ident: $Clone:ident ($Std:ident $cloned:ident) $($Trait:ident $Assoc:ident $fn:ident)*) => { $(
        impl<'a, I: 'a + $Clone, T: $Trait<Item = &'a I>> $Trait for $Type<T> {
            type $Assoc = $Std<<T::$Assoc as IntoIterator>::IntoIter>;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                self.0.$fn(index).into_iter().$cloned()
            }
        }
    )* };
}

macro_rules! grid2d {
    ($Type:ident: $Clone:ident ($cloned:ident) $($Trait:ident $Assoc:ident $Item:ident $fn:ident)*) => { $(
        impl<'a, I: 'a + $Clone, T: $Trait<Item = &'a I>> $Trait for $Type<T> {
            type $Assoc = Map<<T::$Assoc as IntoIterator>::IntoIter, fn(T::$Item) -> Self::$Item>;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                self.0
                    .$fn(index)
                    .into_iter()
                    .map(|item| item.into_iter().$cloned())
            }
        }
    )* };
}

macro_rules! cloned {
    ($(
        $(#[$meta:meta])*
        $Type:ident: $Clone:ident ($Std:ident $cloned:ident)
    )*) => { $(
        $(#[$meta])*
        pub struct $Type<T>(pub(crate) T);

        impl<T> Deref for $Type<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> DerefMut for $Type<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<'a, I: 'a + Clone, T: Grid<Item = &'a I>> Grid for $Type<T> {
            type Item = I;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                self.0.item_unchecked(index).clone()
            }
        }

        grid1d!($Type: $Clone ($Std $cloned)
            GridRow Row row_unchecked
            GridCol Col col_unchecked
        );

        grid2d!($Type: $Clone ($cloned)
            GridRows Rows Row rows_unchecked
            GridCols Cols Col cols_unchecked
        );

        impl<'a, I: 'a + $Clone, T: GridItems<Item = &'a I>> GridItems for $Type<T> {
            type Items = $Std<<T::Items as IntoIterator>::IntoIter>;

            unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
                self.0.items_unchecked(index).into_iter().$cloned()
            }
        }
    )* };
}

cloned!(
    /// A grid yielding [`Clone`](std::clone::Clone)d items.
    ///
    /// This `struct` is created by [`Grid::cloned`](Grid::cloned).
    Cloned: Clone (StdCloned cloned)
    /// A grid yielding [`Copy`](std::marker::Copy)d items.
    ///
    /// This `struct` is created by [`Grid::copied`](Grid::copied).
    Copied: Copy (StdCopied copied)
);

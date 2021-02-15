use crate::*;
use std::ops::{Index, IndexMut};

/*
/// Trait for accessing items of 2D containers,
/// [`Iterator`](std::iter::Iterator) based.
///
/// ***The main trait of this crate.***
///
/// This trait is designed similar to [`IntoIterator`](std::iter::IntoIterator),
/// i.e. is to be implemented on `Self`, `&Self`, `&mut Self` (where
/// applicable). It provides consumers several ways to access the items of the
/// underlying type:
/// - [`item`](crate::Grid::item) (through [`Index0D`](crate::Index0D))
///   retrieves an individual item,
/// - [`row`](crate::Grid::row)/[`col`](crate::Grid::col) (through
///   [`Index1D`](crate::Index1D)) return corresponding `IntoIterator`s of items
///   for that row/col,
/// - [`rows`](crate::Grid::rows)/[`cols`](crate::Grid::cols) (through
///   [`Index2D`](crate::Index2D)) return `Iterator`s of `Iterator`s of items
///   (2D `Iterator`s),
/// - [`items`](crate::Grid::items) (through [`Index2D`](crate::Index2D))
///   returns an `Iterator` of all items.
///
/// Each of these functions come with an `unsafe` unchecked variant, where the
/// index is not checked to the [`Size`](crate::Size) of the container.
pub trait Grid: WithSize + Sized {
    /// The type of the items.
    type Item;

    /// The type of a column.
    type Col: IntoIterator<Item = Self::Item>;

    /// The type of a row.
    type Row: IntoIterator<Item = Self::Item>;

    /// The type of the columns iterator.
    type Cols: Iterator<Item = Self::Col>;

    /// The type of the rows iterator.
    type Rows: Iterator<Item = Self::Row>;

    /// The type of the items iterator.
    type Items: Iterator<Item = Self::Item>;

    /// Returns the item at `index`, without bounds checking.
    ///
    /// For a safe alternative see [`item`](crate::Grid::item).
    ///
    /// # Safety
    ///
    /// Callers **MUST** ensure:
    /// ```ignore
    /// assert!(index.checked(self.size()).is_some())
    /// ```
    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item;

    /// Returns the column at `index`, without bounds checking.
    ///
    /// For a safe alternative see [`col`](crate::Grid::col).
    ///
    /// # Safety
    ///
    /// Callers **MUST** ensure:
    /// ```ignore
    /// assert!(index.checked(ColMajor::from(self.size())).is_some())
    /// ```
    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col;

    /// Returns the row at `index`, without bounds checking.
    ///
    /// For a safe alternative see [`row`](crate::Grid::row).
    ///
    /// # Safety
    ///
    /// Callers **MUST** ensure:
    /// ```ignore
    /// assert!(index.checked(RowMajor::from(self.size())).is_some())
    /// ```
    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row;

    /// Returns an iterator over columns at `index`, without bounds checking.
    ///
    /// For a safe alternative see [`cols`](crate::Grid::cols).
    ///
    /// # Safety
    ///
    /// Callers **MUST** ensure:
    /// ```ignore
    /// assert!(index.checked(self.size()).is_some())
    /// ```
    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols;

    /// Returns an iterator over rows at `index`, without bounds checking.
    ///
    /// For a safe alternative see [`rows`](crate::Grid::rows).
    ///
    /// # Safety
    ///
    /// Callers **MUST** ensure:
    /// ```ignore
    /// assert!(index.checked(self.size()).is_some())
    /// ```
    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows;

    /// Returns an iterator over items at `index`, without bounds checking.
    ///
    /// For a safe alternative see [`items`](crate::Grid::items).
    ///
    /// # Safety
    ///
    /// Callers **MUST** ensure:
    /// ```ignore
    /// assert!(index.checked(self.size()).is_some())
    /// ```
    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items;

    /// Returns the item at `index`, or `None` if out of bounds.
    ///
    /// For an unsafe unchecked alternative see
    /// [`item_unchecked`](crate::Grid::item_unchecked).
    fn item(self, index: impl Index0D) -> Option<Self::Item> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }

    /// Returns the column at `index`, or `None` if out of bounds.
    ///
    /// For an unsafe unchecked alternative see
    /// [`col_unchecked`](crate::Grid::col_unchecked).
    fn col(self, index: impl Index1D) -> Option<Self::Col> {
        let index = index.checked(ColMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.col_unchecked(index) })
    }

    /// Returns the row at `index`, or `None` if out of bounds.
    ///
    /// For an unsafe unchecked alternative see
    /// [`row_unchecked`](crate::Grid::row_unchecked).
    fn row(self, index: impl Index1D) -> Option<Self::Row> {
        let index = index.checked(RowMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.row_unchecked(index) })
    }

    /// Returns an iterator over columns at `index`,
    /// or `None` if out of bounds.
    ///
    /// For an unsafe unchecked alternative see
    /// [`cols_unchecked`](crate::Grid::cols_unchecked).
    fn cols(self, index: impl Index2D) -> Option<Self::Cols> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.cols_unchecked(index) })
    }

    /// Returns an iterator over rows at `index`,
    /// or `None` if out of bounds.
    ///
    /// For an unsafe unchecked alternative see
    /// [`rows_unchecked`](crate::Grid::rows_unchecked).
    fn rows(self, index: impl Index2D) -> Option<Self::Rows> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.rows_unchecked(index) })
    }

    /// Returns an iterator over items at `index`,
    /// or `None` if out of bounds.
    ///
    /// For an unsafe unchecked alternative see
    /// [`items_unchecked`](crate::Grid::items_unchecked).
    fn items(self, index: impl Index2D) -> Option<Self::Items> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.items_unchecked(index) })
    }
}
*/

// =================================== //
// =================================== //
// =================================== //
// =================================== //
// =================================== //
// =================================== //
// =================================== //
// =================================== //
// =================================== //

pub trait GridItem<I>: WithSize + Sized {
    unsafe fn item_unchecked(self, index: impl Index0D) -> I;

    fn item(self, index: impl Index0D) -> Option<I> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }
}

pub trait GridCol<I>: GridItem<I> {
    type Col: IntoIterator<Item = I>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col;

    fn col(self, index: impl Index1D) -> Option<Self::Col> {
        let index = index.checked(ColMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.col_unchecked(index) })
    }
}

pub trait GridRow<I>: GridItem<I> {
    type Row: IntoIterator<Item = I>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row;

    fn row(self, index: impl Index1D) -> Option<Self::Row> {
        let index = index.checked(RowMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.row_unchecked(index) })
    }
}

pub trait GridCols<I>: GridCol<I> {
    type Cols: Iterator<Item = Self::Col>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols;

    fn cols(self, index: impl Index2D) -> Option<Self::Cols> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.cols_unchecked(index) })
    }
}

pub trait GridRows<I>: GridRow<I> {
    type Rows: Iterator<Item = Self::Row>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows;

    fn rows(self, index: impl Index2D) -> Option<Self::Rows> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.rows_unchecked(index) })
    }
}

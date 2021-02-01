use crate::*;
use std::ops::{Index, IndexMut};

pub trait GridIter: WithSize + Sized {
    /// The type of the items.
    type Item;

    /// The type of the column iterator.
    type Col: Iterator<Item = Self::Item>;

    /// The type of the row iterator.
    type Row: Iterator<Item = Self::Item>;

    /// The type of the columns iterator.
    type Cols: Iterator<Item = Self::Col>;

    /// The type of the rows iterator.
    type Rows: Iterator<Item = Self::Row>;

    /// The type of the items iterator.
    type Items: Iterator<Item = Self::Item>;

    /// Returns the item at `point` without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `point < size`
    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item;

    /// Returns an iterator over items at column `index`, without bounds
    /// checking.
    ///
    /// Callers **MUST** ensure:
    /// - `col < width`
    /// - `start <= end`
    /// - `end <= height`
    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col;

    /// Returns an iterator over items at row `index`, without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `row < height`
    /// - `start <= end`
    /// - `end <= width`
    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row;

    /// Returns an iterator over columns at `index`, without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols;

    /// Returns an iterator over rows at `index`, without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows;

    /// Returns an iterator over items at `index`, without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `start <= end` (both ranges)
    /// - `end <= len` (both axis)
    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items;

    /// Returns the item at `point`, or `None` if `point >= size`.
    fn item(self, index: impl Index0D) -> Option<Self::Item> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }

    /// Returns an iterator over items at column `index`,
    /// or `None` if out of bounds.
    fn col(self, index: impl Index1D) -> Option<Self::Col> {
        let index = index.checked(YMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.col_unchecked(index) })
    }

    /// Returns an iterator over items at row `index`,
    /// or `None` if out of bounds.
    fn row(self, index: impl Index1D) -> Option<Self::Row> {
        let index = index.checked(XMajor::from(self.size()))?;

        // SAFETY: index is checked
        Some(unsafe { self.row_unchecked(index) })
    }

    /// Returns an iterator over columns at `index`,
    /// or `None` if out of bounds.
    fn cols(self, index: impl Index2D) -> Option<Self::Cols> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.cols_unchecked(index) })
    }

    /// Returns an iterator over rows at `index`,
    /// or `None` if out of bounds.
    fn rows(self, index: impl Index2D) -> Option<Self::Rows> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.rows_unchecked(index) })
    }

    /// Returns an iterator over items at `index`,
    /// or `None` if out of bounds.
    fn items(self, index: impl Index2D) -> Option<Self::Items> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.items_unchecked(index) })
    }
}

macro_rules! grid {
    ($self:ident $(
        $(#[$meta:meta])*
        $(($mut:ident))? $Trait:ident: ($($Bounds:tt)*)
        $([
            $(#[$assoc_meta:meta])*
            $Assoc:ident
        ])?
        {
            $(#[$fn_unchecked_meta:meta])*
            $fn_unchecked:ident
            $(#[$fn_meta:meta])*
            $fn:ident
        }
    )*) => { $(
        $(#[$meta])*
        pub trait $Trait: $($Bounds)* {
            $(
                $(#[$assoc_meta])*
                type $Assoc;
            )?

            $(#[$fn_unchecked_meta])*
            unsafe fn $fn_unchecked(self: &$($mut)? Self, index: impl Index0D) -> &$($mut)? Self::Item;

            $(#[$fn_meta])*
            fn $fn(self: &$($mut)? Self, index: impl Index0D) -> Option<&$($mut)? Self::Item> {
                let index = index.checked(self.size())?;

                // SAFETY: index is checked
                Some(unsafe { self.$fn_unchecked(index) })
            }
        }
    )* };
}

grid!(self
    ///
    Grid: (WithSize) [
        ///
        Item
    ] {
        ///
        get_unchecked
        ///
        get
    }
    ///
    (mut) GridMut: (Grid) {
        ///
        get_unchecked_mut
        ///
        get_mut
    }
);

macro_rules! mgrid {
    ($self:ident $(
        $(#[$meta:meta])*
        $(($mut:ident))? $Trait:ident$(<$M:ident>)?: ($($Bounds:tt)*) {
            $(#[$fn_unchecked_meta:meta])*
            $fn_unchecked:ident
            $(#[$fn_meta:meta])*
            $fn:ident
        }
    )*) => { $(
        $(#[$meta])*
        pub trait $Trait<$($M: Major)?>: $($Bounds)* {
            $(#[$fn_unchecked_meta])*
            unsafe fn $fn_unchecked(self: &$($mut)? Self, index: impl Index1D) -> &$($mut)? [Self::Item];

            $(#[$fn_meta])*
            fn $fn(self: &$($mut)? Self, index: impl Index1D) -> Option<&$($mut)? [Self::Item]> {
                let index = index.checked(self.msize())?;

                // SAFETY: index is checked
                Some(unsafe { self.$fn_unchecked(index) })
            }
        }
    )* };
}

mgrid!(self
    ///
    MGrid<M>: (WithMSize<M> + Grid) {
        ///
        slice_unchecked
        ///
        slice
    }
    ///
    (mut) MGridMut<M>: (MGrid<M> + GridMut) {
        ///
        slice_unchecked_mut
        ///
        slice_mut
    }
    ///
    XGrid: (MGrid<XMajor>) {
        ///
        row_unchecked
        ///
        row
    }
    ///
    (mut) XGridMut: (XGrid + MGridMut<XMajor>) {
        ///
        row_unchecked_mut
        ///
        row_mut
    }
    ///
    YGrid: (MGrid<YMajor>) {
        ///
        col_unchecked
        ///
        col
    }
    ///
    (mut) YGridMut: (YGrid + MGridMut<YMajor>) {
        ///
        col_unchecked_mut
        ///
        col_mut
    }
);

macro_rules! impl_xygrid {
    ($($MGrid:ident<$Major:ident> $XYGrid:ident $fn:ident $via:ident $(($mut:ident))?)*) => { $(
        impl<T: $MGrid<$Major>> $XYGrid for T {
            unsafe fn $fn(&$($mut)? self, index: impl Index1D) -> &$($mut)? [Self::Item] {
                self.$via(index)
            }
        }
    )* };
}

impl_xygrid!(
    MGrid   <XMajor> XGrid    row_unchecked     slice_unchecked
    MGrid   <YMajor> YGrid    col_unchecked     slice_unchecked
    MGridMut<XMajor> XGridMut row_unchecked_mut slice_unchecked_mut (mut)
    MGridMut<YMajor> YGridMut col_unchecked_mut slice_unchecked_mut (mut)
);

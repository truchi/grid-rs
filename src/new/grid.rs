use crate::*;
use std::ops::{Index, IndexMut};

pub trait GridIter<I>: WithSize + Sized {
    /// The type of the column iterator.
    type Col: Iterator<Item = I>;

    /// The type of the row iterator.
    type Row: Iterator<Item = I>;

    /// The type of the columns iterator.
    type Cols: Iterator<Item = Self::Col>;

    /// The type of the rows iterator.
    type Rows: Iterator<Item = Self::Row>;

    /// The type of the items iterator.
    type Items: Iterator<Item = I>;

    /// Returns the item at `point` without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `point < size`
    unsafe fn item_unchecked(self, point: Point) -> I;

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
    fn item(self, point: Point) -> Option<I> {
        if point < self.size() {
            // SAFETY:
            // point < size
            Some(unsafe { self.item_unchecked(point) })
        } else {
            None
        }
    }

    /// Returns an iterator over items at column `index`,
    /// or `None` if out of bounds.
    fn col(self, index: impl Index1D) -> Option<Self::Col> {
        let (width, height) = self.size().into();
        let (col, range) = index.col(self.size())?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(col < width);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= height);
        Some(unsafe { self.col_unchecked((col, range)) })
    }

    /// Returns an iterator over items at row `index`,
    /// or `None` if out of bounds.
    fn row(self, index: impl Index1D) -> Option<Self::Row> {
        let (width, height) = self.size().into();
        let (row, range) = index.row(self.size())?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(row < height);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= width);
        Some(unsafe { self.row_unchecked((row, range)) })
    }

    /// Returns an iterator over columns at `index`,
    /// or `None` if out of bounds.
    fn cols(self, index: impl Index2D) -> Option<Self::Cols> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.cols_unchecked((x, y)) })
    }

    /// Returns an iterator over rows at `index`,
    /// or `None` if out of bounds.
    fn rows(self, index: impl Index2D) -> Option<Self::Rows> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.rows_unchecked((x, y)) })
    }

    /// Returns an iterator over items at `index`,
    /// or `None` if out of bounds.
    fn items(self, index: impl Index2D) -> Option<Self::Items> {
        let (width, height) = self.size().into();
        let Point { x, y } = index.checked(self.size())?;

        // SAFETY:
        // Index2D::checked guaranties that:
        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(x.end <= width);
        debug_assert!(y.end <= height);
        Some(unsafe { self.items_unchecked((x, y)) })
    }
}

macro_rules! grid {
    ($self:ident $(
        $(#[$meta:meta])*
        $Trait:ident $(: ($($Bounds:tt)*))? {
            ($Self:ty) -> $Item:ty;
            $(#[$item_unchecked_meta:meta])*
            $item_unchecked:ident
            $(#[$item_meta:meta])*
            $item:ident
        }
    )*) => { $(
        $(#[$meta])*
        pub trait $Trait<I> $(: $($Bounds)*)? {
            $(#[$item_unchecked_meta])*
            unsafe fn $item_unchecked($self: $Self, index: impl Index0D) -> $Item;

            $(#[$item_meta])*
            fn $item($self: $Self, index: impl Index0D) -> Option<$Item> {
                let index = index.checked($self.size())?;

                // SAFETY: index is checked
                Some(unsafe { $self.$item_unchecked(index) })
            }
        }
    )* };
}

grid!(self
    ///
    Grid: (WithSize + Index<Point, Output = I>) {
        (&Self) -> &I;
        ///
        get_unchecked
        ///
        get
    }
    ///
    GridMut: (Grid<I> + IndexMut<Point, Output = I>) {
        (&mut Self) -> &mut I;
        ///
        get_unchecked_mut
        ///
        get_mut
    }
);

macro_rules! mgrid {
    ($self:ident $(
        $(#[$meta:meta])*
        $Trait:ident$(<$M:ident>)?: ($($Bounds:tt)*) {
            ($Self:ty) -> $Items:ty;
            $(#[$slice_unchecked_meta:meta])*
            $slice_unchecked:ident
            $(#[$slice_meta:meta])*
            $slice:ident
        }
    )*) => { $(
        $(#[$meta])*
        pub trait $Trait<$($M: Major,)? I>: $($Bounds)* {
            $(#[$slice_unchecked_meta])*
            unsafe fn $slice_unchecked($self: $Self, index: impl Index1D) -> $Items;

            $(#[$slice_meta])*
            fn $slice($self: $Self, index: impl Index1D) -> Option<$Items> {
                let index = index.checked($self.msize())?;

                // SAFETY: index is checked
                Some(unsafe { $self.$slice_unchecked(index) })
            }
        }
    )* };
}

mgrid!(self
    ///
    MGrid<M>: (WithMSize<M> + Grid<I> + Index<usize, Output = [I]>) {
        (&Self) -> &[I];
        ///
        slice_unchecked
        ///
        slice
    }
    ///
    MGridMut<M>: (MGrid<M, I> + GridMut<I> + IndexMut<usize, Output = [I]>) {
        (&mut Self) -> &mut [I];
        ///
        slice_unchecked_mut
        ///
        slice_mut
    }
    ///
    XGrid: (MGrid<XMajor, I>) {
        (&Self) -> &[I];
        ///
        row_unchecked
        ///
        row
    }
    ///
    XGridMut: (XGrid<I> + MGridMut<XMajor, I>) {
        (&mut Self) -> &mut [I];
        ///
        row_unchecked_mut
        ///
        row_mut
    }
    ///
    YGrid: (MGrid<YMajor, I>) {
        (&Self) -> &[I];
        ///
        col_unchecked
        ///
        col
    }
    ///
    YGridMut: (YGrid<I> + MGridMut<YMajor, I>) {
        (&mut Self) -> &mut [I];
        ///
        col_unchecked_mut
        ///
        col_mut
    }
);

macro_rules! impl_xygrid {
    ($($MGrid:ident<$Major:ident> $XYGrid:ident $fn:ident $via:ident $(($mut:ident))?)*) => { $(
        impl<I, T: $MGrid<$Major, I>> $XYGrid<I> for T {
            unsafe fn $fn(&$($mut)? self, index: impl Index1D) -> &$($mut)? [I] {
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

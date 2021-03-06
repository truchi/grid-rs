use crate::*;

/// Base trait for dealing with grids.
///
/// Grids can yield their [`Item`](Grid::Item)s given a [`Index0D`](Index0D)
/// ([`Point`](Point)).
pub trait Grid: WithSize + Sized {
    /// The type of the grid's items.
    type Item;

    /// Returns the item at `index`, without bounds checking.
    ///
    /// ### Safety
    ///
    /// Calling this method with an out-of-bounds `index` is *undefined
    /// behavior*.
    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item;

    /// Returns the item at `index`, or [`None`](std::option::Option::None) if
    /// out of bounds.
    fn item(self, index: impl Index0D) -> Option<Self::Item> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }

    /// Creates a grid which copies all of its elements.
    ///
    /// This is useful when you have an iterator over `&T`, but you need an
    /// iterator over `T`.
    fn copied<'a, T>(self) -> Copied<Self>
    where
        Self: Grid<Item = &'a T>,
        T: 'a + Copy,
    {
        Copied(self)
    }

    /// Creates a grid which clones all of its elements.
    ///
    /// This is useful when you have an iterator over `&T`, but you need an
    /// iterator over `T`.
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Grid<Item = &'a T>,
        T: 'a + Clone,
    {
        Cloned(self)
    }

    /// Creates a new grid by cropping with `rect`, without bounds
    /// checking.
    unsafe fn cropped_unchecked(self, rect: impl Index2D) -> Cropped<Self> {
        Cropped::new_unchecked(rect, self)
    }

    /// Creates a new grid by cropping with `rect`, or
    /// [`None`](std::option::Option::None) if out of bounds.
    fn cropped(self, rect: impl Index2D) -> Option<Cropped<Self>> {
        Cropped::new(rect, self)
    }
}

macro_rules! grid1d {
    ($(
        $(#[$meta:meta])*
        $Trait:ident $Assoc:ident
        $(#[$unchecked_meta:meta])*
        $unchecked:ident
        $(#[$checked_meta:meta])*
        $checked:ident
    )*) => { $(
        $(#[$meta])*
        pub trait $Trait: Grid {
            type $Assoc: IntoIterator<Item = Self::Item>;

            $(#[$unchecked_meta])*
            unsafe fn $unchecked(self, index: impl Index1D) -> Self::$Assoc;

            $(#[$checked_meta])*
            fn $checked(self, index: impl Index1D) -> Option<Self::$Assoc> {
                let index = index.$checked(self.size())?;

                // SAFETY: index is checked
                Some(unsafe { self.$unchecked(index) })
            }
        }
    )* };
}

macro_rules! grid2d {
    ($(
        $(#[$meta:meta])*
        $Trait:ident $Assoc:ident ($Parent:ident $Item:ident)
        $(#[$unchecked_meta:meta])*
        $unchecked:ident
        $(#[$checked_meta:meta])*
        $checked:ident
    )*) => { $(
        $(#[$meta])*
        pub trait $Trait: $Parent {
            type $Assoc: IntoIterator<Item = Self::$Item>;

            $(#[$unchecked_meta])*
            unsafe fn $unchecked(self, index: impl Index2D) -> Self::$Assoc;

            $(#[$checked_meta])*
            fn $checked(self, index: impl Index2D) -> Option<Self::$Assoc> {
                let index = index.checked(self.size())?;

                // SAFETY: index is checked
                Some(unsafe { self.$unchecked(index) })
            }
        }
    )* };
}

grid1d!(
    /// Provides a [`Col`](GridCol::Col) 1D [`IntoIterator`](std::iter::IntoIterator).
    GridCol Col
        /// Returns the column at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index` is *undefined
        /// behavior*.
        col_unchecked
        /// Returns the column at `index`, or [`None`](std::option::Option::None) if
        /// out of bounds.
        col
    /// Provides a [`Row`](GridRow::Row) 1D [`IntoIterator`](std::iter::IntoIterator).
    GridRow Row
        /// Returns the row at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index` is *undefined
        /// behavior*.
        row_unchecked
        /// Returns the row at `index`, or [`None`](std::option::Option::None) if
        /// out of bounds.
        row
);

grid2d!(
    /// Provides a [`Cols`](GridCols::Cols) 2D [`IntoIterator`](std::iter::IntoIterator).
    GridCols Cols (GridCol Col)
        /// Returns the columns at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index` is *undefined
        /// behavior*.
        cols_unchecked
        /// Returns the columns at `index`, or [`None`](std::option::Option::None) if
        /// out of bounds.
        cols
    /// Provides a [`Rows`](GridRows::Rows) 2D [`IntoIterator`](std::iter::IntoIterator).
    GridRows Rows (GridRow Row)
        /// Returns the rows at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index` is *undefined
        /// behavior*.
        rows_unchecked
        /// Returns the rows at `index`, or [`None`](std::option::Option::None) if
        /// out of bounds.
        rows
    /// Provides an [`Items`](GridItems::Items) 2D [`IntoIterator`](std::iter::IntoIterator).
    GridItems Items (Grid Item)
        /// Returns the items at `index`, without bounds checking.
        ///
        /// ### Safety
        ///
        /// Calling this method with an out-of-bounds `index` is *undefined
        /// behavior*.
        items_unchecked
        /// Returns the items at `index`, or [`None`](std::option::Option::None) if
        /// out of bounds.
        items
);

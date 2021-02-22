use crate::*;

pub trait Grid: WithSize + Sized {
    type Item;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item;

    fn item(self, index: impl Index0D) -> Option<Self::Item> {
        let index = index.checked(self.size())?;

        // SAFETY: index is checked
        Some(unsafe { self.item_unchecked(index) })
    }

    fn crop(self, rect: impl Index2D) -> Option<Crop<Self>> {
        Crop::new(rect, self)
    }

    unsafe fn crop_unchecked(self, rect: impl Index2D) -> Crop<Self> {
        Crop::new_unchecked(rect, self)
    }

    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Grid<Item = &'a T>,
        T: 'a + Clone,
    {
        Cloned::new(self)
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

            $(#[$unchecked_meta:meta])*
            unsafe fn $unchecked(self, index: impl Index2D) -> Self::$Assoc;

            $(#[$checked_meta:meta])*
            fn $checked(self, index: impl Index2D) -> Option<Self::$Assoc> {
                let index = index.checked(self.size())?;

                // SAFETY: index is checked
                Some(unsafe { self.$unchecked(index) })
            }
        }
    )* };
}

grid1d!(
    GridCol Col col_unchecked col
    GridRow Row row_unchecked row
);

grid2d!(
    GridCols Cols (GridCol Col) cols_unchecked cols
    GridRows Rows (GridRow Row) rows_unchecked rows
    GridItems Items (Grid Item) items_unchecked items
);

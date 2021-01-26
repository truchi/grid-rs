use crate::*;
use std::ops::{Index, IndexMut};

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

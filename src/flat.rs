use crate::*;
use std::{
    iter::Flatten,
    marker::PhantomData,
    ops::{Index, IndexMut, RangeBounds},
    slice::{Iter, IterMut},
};

pub type ColFlat<I, T> = Flat<ColMajor, I, T>;
pub type RowFlat<I, T> = Flat<RowMajor, I, T>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Flat<M, I, T> {
    size:    M,
    items:   T,
    phantom: PhantomData<I>,
}

/// ### Constructors
impl<M: Major, I, T> Flat<M, I, T> {
    /// Creates a new [`Flat`](crate::Flat), without checking size.
    pub fn new_unchecked(size: Size, items: T) -> Self {
        Self {
            size: size.into(),
            items,
            phantom: PhantomData,
        }
    }

    /// Creates a new [`Flat`](crate::Flat) if `len != x * y`, `None` otherwise.
    pub fn new(size: Size, items: T) -> Option<Self>
    where
        T: AsRef<[I]>,
    {
        if items.as_ref().len() == size.x * size.y {
            Some(Self::new_unchecked(size, items))
        } else {
            None
        }
    }
}

impl<M, I, T: AsRef<[I]>> AsRef<[I]> for Flat<M, I, T> {
    fn as_ref(&self) -> &[I] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[I]>> AsMut<[I]> for Flat<M, I, T> {
    fn as_mut(&mut self) -> &mut [I] {
        self.items.as_mut()
    }
}

impl<M: Major, I, T> WithSize for Flat<M, I, T> {
    fn size(&self) -> Size {
        self.size.into()
    }
}

impl<M: Major, I, T> WithMSize<M> for Flat<M, I, T> {
    fn msize(&self) -> M {
        self.size
    }
}

/*
macro_rules! grid {
    ($($M:ident $Type:ident)*) => { $(
        grid!(impl $M
                  $Type AsRef as_ref get_unchecked
            (mut) $Type AsMut as_mut get_unchecked_mut
        );
    )* };
    (impl Col $($(($mut:ident))? $Type:ident $As:ident $as:ident $get:ident)*) => { $(
        grid!(impl $(($mut))? $Type $As $as
            Cols cols_unchecked
            Col  col_unchecked
            Row  row_unchecked (RowIter)
            $get
        );
    )* };
    (impl Row $($(($mut:ident))? $Type:ident $As:ident $as:ident $get:ident)*) => { $(
        grid!(impl $(($mut))? $Type $As $as
            Rows rows_unchecked
            Row  row_unchecked
            Col  col_unchecked (ColIter)
            $get
        );
    )* };
    (impl $(($mut:ident))? $Type:ident $As:ident $as:ident
        $Majors:ident $majors:ident
        $Major:ident  $major:ident
        $Minor:ident  $minor:ident ($MinorIter:ident)
        $get:ident
    ) => {
        impl<'a, I, T: $As<[I]>> Grid for &'a $($mut)? $Type<I, T> {
            type Item = &'a $($mut)? I;
            type $Major = &'a $($mut)? [I];
            type $Minor = $MinorIter<Self>;
            type Cols = ColsIter<Self>;
            type Rows = RowsIter<Self>;
            type Items = Flatten<Self::$Majors>;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                use crate::index::flat::Index0D;
                let msize = self.msize();
                let index = index.unchecked(msize.into()).index(msize);

                self.items.$as().$get(index)
            }

            unsafe fn $major(self, index: impl Index1D) -> Self::$Major {
                use crate::index::flat::Index1D;
                let msize = self.msize();
                let index = index.unchecked(msize).index(msize);

                self.items.$as().$get(index)
            }

            unsafe fn $minor(self, index: impl Index1D) -> Self::$Minor {
                Self::$Minor::new_unchecked(self, index, Self::item_unchecked)
            }

            unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
                Self::Cols::new_unchecked(self, index, Self::col_unchecked)
            }

            unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
                Self::Rows::new_unchecked(self, index, Self::row_unchecked)
            }

            unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
                self.$majors(index).flatten()
            }
        }
    };
}
*/

/*
grid!(
    Col ColFlat
    Row RowFlat
);
*/

// ========================= //
// ========================= //
// ========================= //
// ========================= //
// ========================= //
// ========================= //
// ========================= //
// ========================= //
// ========================= //
// ========================= //

macro_rules! grid_item {
    ($($As:ident $as:ident $get:ident $(($mut:ident))?)*) => { $(
        impl<'a, M: Major, I, T: $As<[I]>> GridItem for &'a $($mut)? Flat<M, I, T> {
            type Item = &'a $($mut)? I;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                use crate::index::flat::Index0D;
                let msize = self.msize();
                let index = index.unchecked(msize.into()).index(msize);

                self.items.$as().$get(index)
            }
        }
    )* };
}

grid_item!(
    AsRef as_ref get_unchecked
    AsMut as_mut get_unchecked_mut (mut)
);

impl<'a, M: Major, I, T: AsRef<[I]>> GridMajor<M> for &'a Flat<M, I, T> {
    type Major = &'a [I];

    unsafe fn major_unchecked(self, index: impl Index1D) -> Self::Major {
        use crate::index::flat::Index1D;
        let msize = self.msize();
        let index = index.unchecked(msize).index(msize);

        self.items.as_ref().get_unchecked(index)
    }
}

impl<'a, M: Major, I, T: AsMut<[I]>> GridMajor<M> for &'a mut Flat<M, I, T> {
    type Major = &'a mut [I];

    unsafe fn major_unchecked(self, index: impl Index1D) -> Self::Major {
        use crate::index::flat::Index1D;
        let msize = self.msize();
        let index = index.unchecked(msize).index(msize);

        self.items.as_mut().get_unchecked_mut(index)
    }
}

// ========================= //

impl<'a, I, T: AsRef<[I]>> GridRow for &'a RowFlat<I, T> {
    type Row = &'a [I];

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        self.major_unchecked(index)
    }
}

impl<'a, I, T: AsMut<[I]>> GridRow for &'a mut RowFlat<I, T> {
    type Row = &'a mut [I];

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        self.major_unchecked(index)
    }
}

// ========================= //

impl<'a, I, T: AsRef<[I]>> GridCol for &'a ColFlat<I, T> {
    type Col = &'a [I];

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        use crate::index::flat::Index1D;
        let msize = self.msize();
        let index = index.unchecked(msize).index(msize);

        self.items.as_ref().get_unchecked(index)
    }
}

impl<'a, I, T: AsMut<[I]>> GridCol for &'a mut ColFlat<I, T> {
    type Col = &'a mut [I];

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        use crate::index::flat::Index1D;
        let msize = self.msize();
        let index = index.unchecked(msize).index(msize);

        self.items.as_mut().get_unchecked_mut(index)
    }
}

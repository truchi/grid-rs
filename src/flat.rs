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

macro_rules! grid_item {
    () => {
        grid_item!(AsRef as_ref get_unchecked );
        grid_item!(AsMut as_mut get_unchecked_mut (mut));
    };
    ($As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, M: Major, I, T: $As<[I]>> GridItem for &'a $($mut)? Flat<M, I, T> {
            type Item = &'a $($mut)? I;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                use crate::index::flat::Index0D;
                let msize = self.msize();
                let index = index.unchecked(msize.into()).index(msize);

                self.items.$as().$get(index)
            }
        }
    };
}

macro_rules! grid_major {
    ($($Type:ident: $Trait:ident ($Assoc:ident $fn:ident))*) => { $(
        grid_major!($Type $Trait $Assoc $fn AsRef as_ref get_unchecked);
        grid_major!($Type $Trait $Assoc $fn AsMut as_mut get_unchecked_mut (mut));
    )* };
    ($Type:ident $Trait:ident $Assoc:ident $fn:ident $As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, I, T: $As<[I]>> $Trait for &'a $($mut)? $Type<I, T> {
            type $Assoc = &'a $($mut)? [I];

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                use crate::index::flat::Index1D;
                let msize = self.msize();
                let index = index.unchecked(msize).index(msize);

                self.items.$as().$get(index)
            }
        }
    };
}

macro_rules! grid_minor {
    ($($Type:ident($M:ident): $Trait:ident ($Assoc:ident $fn:ident))*) => { $(
        grid_minor!(AsRef $Type $M $Trait $Assoc $fn Index1D msize Minor);
        grid_minor!(AsMut $Type $M $Trait $Assoc $fn Index1D msize MinorMut (mut));
    )* };
    (
        $As:ident
        $Type:ident $M:ident
        $Trait:ident $Assoc:ident
        $fn:ident $Index:ident $size:ident
        $Iter:ident
        $(($mut:ident))?
    ) => {
        impl<'a, I, T: $As<[I]>> $Trait for &'a $($mut)? $Type<I, T> {
            type $Assoc = $Iter<'a, $M, I, T>;

            unsafe fn $fn(self, index: impl $Index) -> Self::$Assoc {
                Self::$Assoc::new(self, index.unchecked(self.$size()))
            }
        }
    };
}

macro_rules! grid_majors {
    ($($Type:ident($M:ident): $Trait:ident ($Assoc:ident $fn:ident))*) => { $(
        grid_minor!(AsRef $Type $M $Trait $Assoc $fn Index2D size Majors);
        grid_minor!(AsMut $Type $M $Trait $Assoc $fn Index2D size MajorsMut (mut));
    )* };
}

grid_item!();

grid_major!(
    RowFlat: GridRow (Row row_unchecked)
    ColFlat: GridCol (Col col_unchecked)
);

grid_minor!(
    RowFlat(RowMajor): GridCol(Col col_unchecked)
    ColFlat(ColMajor): GridRow(Row row_unchecked)
);

grid_majors!(
    RowFlat(RowMajor): GridRows(Rows rows_unchecked)
    ColFlat(ColMajor): GridCols(Cols cols_unchecked)
);

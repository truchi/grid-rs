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

macro_rules! grid {
    () => {
        // ITEM
        grid!(impl [ITEM] AsRef as_ref get_unchecked);
        grid!(impl [ITEM] AsMut as_mut get_unchecked_mut (mut));

        // MAJOR
        grid!(impl [SLICE] RowFlat GridRow Row row_unchecked AsRef as_ref get_unchecked);
        grid!(impl [SLICE] RowFlat GridRow Row row_unchecked AsMut as_mut get_unchecked_mut (mut));

        grid!(impl [SLICE] ColFlat GridCol Col col_unchecked AsRef as_ref get_unchecked);
        grid!(impl [SLICE] ColFlat GridCol Col col_unchecked AsMut as_mut get_unchecked_mut (mut));

        // MINOR
        grid!(impl [ITER] RowFlat RowMajor GridCol Col col_unchecked AsRef Index1D msize Minor);
        grid!(impl [ITER] RowFlat RowMajor GridCol Col col_unchecked AsMut Index1D msize MinorMut (mut));

        grid!(impl [ITER] ColFlat ColMajor GridRow Row row_unchecked AsRef Index1D msize Minor);
        grid!(impl [ITER] ColFlat ColMajor GridRow Row row_unchecked AsMut Index1D msize MinorMut (mut));

        // MAJORS
        grid!(impl [ITER] RowFlat RowMajor GridRows Rows rows_unchecked AsRef Index2D size Majors);
        grid!(impl [ITER] RowFlat RowMajor GridRows Rows rows_unchecked AsMut Index2D size MajorsMut (mut));

        grid!(impl [ITER] ColFlat ColMajor GridCols Cols cols_unchecked AsRef Index2D size Majors);
        grid!(impl [ITER] ColFlat ColMajor GridCols Cols cols_unchecked AsMut Index2D size MajorsMut (mut));
    };
    (impl [ITEM] $As:ident $as:ident $get:ident $(($mut:ident))?) => {
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
    (impl [SLICE] $Type:ident $Trait:ident $Assoc:ident $fn:ident $As:ident $as:ident $get:ident $(($mut:ident))?) => {
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
    (impl [ITER]
        $Type:ident $M:ident
        $Trait:ident $Assoc:ident $fn:ident
        $As:ident
        $Index:ident $size:ident
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

grid!();

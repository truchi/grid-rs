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
        grid_item!(impl [ITEM] AsRef as_ref get_unchecked);
        grid_item!(impl [ITEM] AsMut as_mut get_unchecked_mut (mut));
    };
    (impl [ITEM] $As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, M: Major, I, T: $As<[I]>> GridItem<&'a $($mut)? I> for &'a $($mut)? Flat<M, I, T> {
            unsafe fn item_unchecked(self, index: impl Index0D) -> &'a $($mut)? I {
                use crate::index::flat::Index0D;
                let msize = self.msize();
                let index = index.unchecked(msize.into()).index(msize);

                self.items.$as().$get(index)
            }
        }
    };
}

macro_rules! grid {
    ($(
        $Type:ident<$M:ident>
            $GridMajor:ident<$Major:ident> ($major:ident)
            $GridMinor:ident<$Minor:ident> ($minor:ident)
            $GridMajors:ident<$Majors:ident> ($majors:ident)
            $GridMinors:ident<$Minors:ident> ($minors:ident)
    )*) => { $(
        // Major
        grid!(impl [SLICE] $Type $GridMajor $Major $major AsRef as_ref get_unchecked);
        grid!(impl [SLICE] $Type $GridMajor $Major $major AsMut as_mut get_unchecked_mut (mut));
        grid!(impl [CLONED 1D] $Type $GridMajor $Major $major (iter));

        // Minor
        grid!(impl [ITER] $Type $M $GridMinor $Minor $minor AsRef Index1D msize Minor);
        grid!(impl [ITER] $Type $M $GridMinor $Minor $minor AsMut Index1D msize MinorMut (mut));
        grid!(impl [CLONED 1D] $Type $GridMinor $Minor $minor);

        // Majors
        grid!(impl [ITER] $Type $M $GridMajors $Majors $majors AsRef Index2D size Majors);
        grid!(impl [ITER] $Type $M $GridMajors $Majors $majors AsMut Index2D size MajorsMut (mut));
        grid!(impl [CLONED 2D] $Type $M $GridMajors $Majors $majors $Major (iter));

        // Minors
        grid!(impl [ITER] $Type $M $GridMinors $Minors $minors AsRef Index2D size Minors);
        grid!(impl [CLONED 2D] $Type $M $GridMinors $Minors $minors $Minor);
    )* };
    (impl [SLICE] $Type:ident $Trait:ident $Assoc:ident $fn:ident $As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, I, T: $As<[I]>> $Trait<&'a $($mut)? I> for &'a $($mut)? $Type<I, T> {
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
        impl<'a, I, T: $As<[I]>> $Trait<&'a $($mut)? I> for &'a $($mut)? $Type<I, T> {
            type $Assoc = $Iter<'a, $M, I, T>;

            unsafe fn $fn(self, index: impl $Index) -> Self::$Assoc {
                Self::$Assoc::new(self, index.unchecked(self.$size()))
            }
        }
    };
    (impl [CLONED 1D]
        $Type:ident
        $Trait:ident $Assoc:ident $fn:ident
        $(($iter:ident))?
    ) => {
        impl<'a, I: Clone, T: AsRef<[I]>> $Trait<I> for &'a $Type<I, T> {
            type $Assoc = std::iter::Cloned<
                <<Self as $Trait<&'a I>>::$Assoc as IntoIterator>::IntoIter
            >;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                <Self as $Trait<&'a I>>::$fn(self, index)
                    $(.$iter())?
                    .cloned()
            }
        }
    };
    (impl [CLONED 2D]
        $Type:ident $M:ident
        $Trait:ident $Assoc:ident $fn:ident
        $Assoc1D:ident
        $(($iter:ident))?
    ) => {
        impl<'a, I: Clone, T: AsRef<[I]>> $Trait<I> for &'a $Type<I, T> {
            type $Assoc = std::iter::Map<
                <Self as $Trait<&'a I>>::$Assoc,
                fn(<<Self as $Trait<&'a I>>::$Assoc as Iterator>::Item) -> Self::$Assoc1D,
            >;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                <Self as $Trait<&'a I>>::$fn(self, index).map(|xs| xs$(.$iter())?.cloned())
            }
        }
    };
}

grid_item!();

impl<'a, M: Major, I: Clone, T: AsRef<[I]>> GridItem<I> for &'a Flat<M, I, T> {
    unsafe fn item_unchecked(self, index: impl Index0D) -> I {
        <Self as GridItem<&'a I>>::item_unchecked(self, index).clone()
    }
}

grid!(
    RowFlat<RowMajor>
        GridRow<Row> (row_unchecked)
        GridCol<Col> (col_unchecked)
        GridRows<Rows> (rows_unchecked)
        GridCols<Cols> (cols_unchecked)
    ColFlat<ColMajor>
        GridCol<Col> (col_unchecked)
        GridRow<Row> (row_unchecked)
        GridCols<Cols> (cols_unchecked)
        GridRows<Rows> (rows_unchecked)
);

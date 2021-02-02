use crate::*;
use std::{
    iter::Flatten,
    marker::PhantomData,
    ops::{Index, IndexMut, RangeBounds},
    slice::{Iter, IterMut},
};

pub type XFlat<I, T> = Flat<XMajor, I, T>;
pub type YFlat<I, T> = Flat<YMajor, I, T>;

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

    /// Creates a new [`Flat`](crate::Flat) if `len >= x * y`, `None` otherwise.
    pub fn new(size: Size, items: T) -> Option<Self>
    where
        T: AsRef<[I]>,
    {
        if items.as_ref().len() >= size.x * size.y {
            Some(Self::new_unchecked(size, items))
        } else {
            None
        }
    }
}

impl<M: Major, I, T> Flat<M, I, T> {
    pub fn size(&self) -> M {
        self.size
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

macro_rules! grid_iter {
    ($($M:ident $Type:ident)*) => { $(
        grid_iter!(impl $M
                        $Type Iter   (get_unchecked     slice_unchecked     iter)
            (mut AsMut) $Type IterMut(get_unchecked_mut slice_unchecked_mut iter_mut)
        );
    )* };
    (impl X $($(($mut:ident $AsMut:ident))? $Type:ident $Iter:ident($get:ident $slice:ident $iter:ident))*) => { $(
        grid_iter!(impl $(($mut $AsMut))? $Type
            Rows rows_unchecked
            Row  row_unchecked ($Iter $iter)
            Col  col_unchecked (YIter1D)
            $get $slice
        );
    )* };
    (impl Y $($(($mut:ident $AsMut:ident))? $Type:ident $Iter:ident($get:ident $slice:ident $iter:ident))*) => { $(
        grid_iter!(impl $(($mut $AsMut))? $Type
            Cols cols_unchecked
            Col  col_unchecked ($Iter $iter)
            Row  row_unchecked (XIter1D)
            $get $slice
        );
    )* };
    (impl $(($mut:ident $AsMut:ident))? $Type:ident
        $Majors:ident $majors:ident
        $Major:ident  $major:ident ($Iter:ident $iter:ident)
        $Minor:ident  $minor:ident ($MinorIter:ident)
        $get:ident $slice:ident
    ) => {
        impl<'a, I, T: AsRef<[I]> $(+ $AsMut<[I]>)?> GridIter for &'a $($mut)? $Type<I, T> {
            type Item = &'a $($mut)? I;
            type $Major = $Iter<'a, I>;
            type $Minor = $MinorIter<Self>;
            type Cols = YIter2D<Self>;
            type Rows = XIter2D<Self>;
            type Items = Flatten<Self::$Majors>;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                self.$get(index)
            }

            unsafe fn $major(self, index: impl Index1D) -> Self::$Major {
                self.$slice(index).$iter()
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

grid_iter!(
    X XFlat
    Y YFlat
);

macro_rules! index {
    ($($Type:ident)*) => { $(
        index!(impl trait
            Index(index)<Point, Output = I>
            for $Type get
        );
        index!(impl trait
            Index(index)<usize, Output = [I]>
            for $Type slice
        );
        index!(impl trait <(R: RangeBounds<usize>)>
            Index(index)<(usize, R), Output = [I]>
            for $Type slice
        );
        index!(impl trait (mut AsMut)
            IndexMut(index_mut)<Point>
            for $Type get_mut
        );
        index!(impl trait (mut AsMut)
            IndexMut(index_mut)<usize>
            for $Type slice_mut
        );
        index!(impl trait (mut AsMut) <(R: RangeBounds<usize>)>
            IndexMut(index_mut)<(usize, R)>
            for $Type slice_mut
        );
    )* };
    (impl trait $(($mut:ident $AsMut:ident))? $(<($($Bounds:tt)*)>)?
        $Index:ident($index:ident)<$Idx:ty $(, Output = $Output:ty)?>
        for $Type:ident $fn:ident
    ) => {
        impl<I, T: AsRef<[I]> $(+ $AsMut<[I]>)? $(, $($Bounds)*)?> $Index<$Idx> for $Type<I, T> {
            $(type Output = $Output;)?

            fn $index(&$($mut)? self, index: $Idx) -> &$($mut)? Self::Output {
                self.$fn(index).expect("Out of bound index")
            }
        }
    };
}

macro_rules! grid {
    ($($Type:ident $Major:ident)*) => { $(
        grid!(impl trait
            [Grid] get_unchecked Index0D (Item -> Self::Item)
            for $Type as_ref get_unchecked
        );
        grid!(impl trait
            [MGrid<$Major>] slice_unchecked Index1D (-> [Self::Item])
            for $Type as_ref get_unchecked
        );
        grid!(impl trait (mut AsMut)
            [GridMut] get_unchecked_mut Index0D (-> Self::Item)
            for $Type as_mut get_unchecked_mut
        );
        grid!(impl trait (mut AsMut)
            [MGridMut<$Major>] slice_unchecked_mut Index1D (-> [Self::Item])
            for $Type as_mut get_unchecked_mut
        );
    )* };
    (impl trait $(($mut:ident $AsMut:ident))?
        [$Grid:path] $fn:ident $Index:ident ($($Assoc:ident)? -> $Output:ty)
        for $Type:ident $as:ident $via:ident
    ) => {
        impl<I, T: AsRef<[I]> + $($AsMut<[I]>)?> $Grid for $Type<I, T> {
            $(type $Assoc = I;)?

            unsafe fn $fn(&$($mut)? self, index: impl $Index) -> &$($mut)? $Output {
                use crate::new::index::flat::$Index;
                let size = self.msize();
                let index = index.unchecked(size).index(size);

                self.items.$as().$via(index)
            }
        }
    };
}

index!(XFlat YFlat);

grid!(
    XFlat XMajor
    YFlat YMajor
);

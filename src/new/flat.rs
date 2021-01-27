use crate::*;
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut, RangeBounds},
};

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

use std::{iter::Flatten, slice::Iter};

impl<'a, I, T: AsRef<[I]>> GridIter<&'a I> for &'a Flat<XMajor, I, T> {
    type Col = XHelper1D<Self, &'a I>;
    type Cols = YHelper2D<Self, Self::Col>;
    type Items = Flatten<Self::Rows>;
    type Row = Iter<'a, I>;
    type Rows = XHelper2D<Self, Self::Row>;

    unsafe fn item_unchecked(self, index: impl Index0D) -> &'a I {
        use crate::new::index::flat::Index0D;
        let size = self.msize();
        let index = index.unchecked(size).index(size);

        self.items.as_ref().get_unchecked(index)
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Self::Col::new_unchecked(self, index, Self::item_unchecked)
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        self.slice_unchecked(index).iter()
    }

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        Self::Cols::new_unchecked(self, index, Self::col_unchecked)
    }

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        Self::Rows::new_unchecked(self, index, Self::row_unchecked)
    }

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        todo!()
    }
}

// ================================================================ //

pub type XFlat<I, T> = Flat<XMajor, I, T>;
pub type YFlat<I, T> = Flat<YMajor, I, T>;

macro_rules! index {
    ($(
        $Type:ident <$($As:ident)*> $(<($($Bounds:tt)*)>)?
        $Index:ident($index:ident)<$Idx:ty $(, Output = $Output:ty)?>
        ($fn:ident $($mut:ident)?)
    )*) => { $(
        impl<I, T: $($As<[I]> +)* $(, $($Bounds)*)?> $Index<$Idx> for $Type<I, T> {
            $(type Output = $Output;)?

            fn $index(&$($mut)? self, index: $Idx) -> &$($mut)? Self::Output {
                self.$fn(index).expect("Out of bound index")
            }
        }
    )* };
}

index!(
    XFlat<AsRef> Index(index)<Point, Output = I> (get)
    YFlat<AsRef> Index(index)<Point, Output = I> (get)
    XFlat<AsRef> Index(index)<usize, Output = [I]> (slice)
    YFlat<AsRef> Index(index)<usize, Output = [I]> (slice)
    XFlat<AsRef><(R: RangeBounds<usize>)> Index(index)<(usize, R), Output = [I]> (slice)
    YFlat<AsRef><(R: RangeBounds<usize>)> Index(index)<(usize, R), Output = [I]> (slice)
    XFlat<AsRef AsMut> IndexMut(index_mut)<Point> (get_mut mut)
    YFlat<AsRef AsMut> IndexMut(index_mut)<Point> (get_mut mut)
    XFlat<AsRef AsMut> IndexMut(index_mut)<usize> (slice_mut mut)
    YFlat<AsRef AsMut> IndexMut(index_mut)<usize> (slice_mut mut)
    XFlat<AsRef AsMut><(R: RangeBounds<usize>)> IndexMut(index_mut)<(usize, R)> (slice_mut mut)
    YFlat<AsRef AsMut><(R: RangeBounds<usize>)> IndexMut(index_mut)<(usize, R)> (slice_mut mut)
);

macro_rules! grid {
    ($(
        $Type:ident<$($As:ident)*> $as:ident  $Grid:path:
        $Index:ident $Output:ty,
        ($fn:ident $via:ident $($mut:ident)?)
    )*) => { $(
        impl<I, T: $($As<[I]> +)*> $Grid for $Type<I, T> {
            unsafe fn $fn(&$($mut)? self, index: impl $Index) -> &$($mut)? $Output {
                use crate::new::index::flat::$Index;
                let size = self.msize();
                let index = index.unchecked(size).index(size);

                self.items.$as().$via(index)
            }
        }
    )* };
}

grid!(
    XFlat<AsRef> as_ref Grid<I>: Index0D I, (get_unchecked get_unchecked)
    YFlat<AsRef> as_ref Grid<I>: Index0D I, (get_unchecked get_unchecked)
    XFlat<AsRef> as_ref MGrid<XMajor, I>: Index1D [I], (slice_unchecked get_unchecked)
    YFlat<AsRef> as_ref MGrid<YMajor, I>: Index1D [I], (slice_unchecked get_unchecked)
    XFlat<AsRef AsMut> as_mut GridMut<I>: Index0D I, (get_unchecked_mut get_unchecked_mut mut)
    YFlat<AsRef AsMut> as_mut GridMut<I>: Index0D I, (get_unchecked_mut get_unchecked_mut mut)
    XFlat<AsRef AsMut> as_mut MGridMut<XMajor, I>: Index1D [I], (slice_unchecked_mut get_unchecked_mut mut)
    YFlat<AsRef AsMut> as_mut MGridMut<YMajor, I>: Index1D [I], (slice_unchecked_mut get_unchecked_mut mut)
);

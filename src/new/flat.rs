use crate::{
    Grid,
    GridMut,
    Index0D,
    Index1D,
    MGrid,
    MGridMut,
    Major,
    Point,
    Size,
    XGrid,
    XGridMut,
    XMajor,
    YMajor,
};
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
        $([$($size:tt)*])?
        ($fn:ident $via:ident $($mut:ident)?)
    )*) => { $(
        impl<I, T: $($As<[I]> +)*> $Grid for $Type<I, T> {
            $(grid!(impl $($size)*);)?

            unsafe fn $fn(&$($mut)? self, index: impl $Index) -> &$($mut)? $Output {
                use crate::new::index::flat::$Index;
                let size = self.size();
                let index = index.unchecked(size).index(size);

                self.items.$as().$via(index)
            }
        }
    )* };
    (impl size) => {
        fn size(&self) -> Size { self.size.into() }
    };
    (impl msize $M:ident) => {
        fn msize(&self) -> $M { self.size }
    };
}

grid!(
    XFlat<AsRef> as_ref Grid<I>: Index0D I, [size] (get_unchecked get_unchecked)
    YFlat<AsRef> as_ref Grid<I>: Index0D I, [size] (get_unchecked get_unchecked)
    XFlat<AsRef> as_ref MGrid<XMajor, I>: Index1D [I], [msize XMajor] (slice_unchecked get_unchecked)
    YFlat<AsRef> as_ref MGrid<YMajor, I>: Index1D [I], [msize YMajor] (slice_unchecked get_unchecked)
    XFlat<AsRef AsMut> as_mut GridMut<I>: Index0D I, (get_unchecked_mut get_unchecked_mut mut)
    YFlat<AsRef AsMut> as_mut GridMut<I>: Index0D I, (get_unchecked_mut get_unchecked_mut mut)
    XFlat<AsRef AsMut> as_mut MGridMut<XMajor, I>: Index1D [I], (slice_unchecked_mut get_unchecked_mut mut)
    YFlat<AsRef AsMut> as_mut MGridMut<YMajor, I>: Index1D [I], (slice_unchecked_mut get_unchecked_mut mut)
);

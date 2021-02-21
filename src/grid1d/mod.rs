mod index;
pub mod iter;

use crate::*;
use index::*;
use std::marker::PhantomData;

pub type ColGrid1D<I, T> = Grid1D<ColMajor, I, T>;
pub type RowGrid1D<I, T> = Grid1D<RowMajor, I, T>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Grid1D<M, I, T> {
    size:    M,
    items:   T,
    phantom: PhantomData<I>,
}

/// ### Constructors
impl<M: Major, I, T> Grid1D<M, I, T> {
    /// Creates a new [`Grid1D`](crate::Grid1D), without checking size.
    pub fn new_unchecked(size: Size, items: T) -> Self {
        Self {
            size: size.into(),
            items,
            phantom: PhantomData,
        }
    }

    /// Creates a new [`Grid1D`](crate::Grid1D) if `len != x * y`, `None`
    /// otherwise.
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

impl<M, I, T: AsRef<[I]>> AsRef<[I]> for Grid1D<M, I, T> {
    fn as_ref(&self) -> &[I] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[I]>> AsMut<[I]> for Grid1D<M, I, T> {
    fn as_mut(&mut self) -> &mut [I] {
        self.items.as_mut()
    }
}

impl<M: Major, I, T> WithSize for Grid1D<M, I, T> {
    fn size(&self) -> Size {
        self.size.into()
    }
}

impl<M: Major, I, T> WithMSize<M> for Grid1D<M, I, T> {
    fn msize(&self) -> M {
        self.size
    }
}

macro_rules! grid {
    ($(
        $Type:ident<$M:ident>
            $GridMajor:ident<$Major:ident> ($major:ident)
            $GridMinor:ident<$Minor:ident> ($minor:ident)
            $GridMajors:ident<$Majors:ident> ($majors:ident)
            $GridMinors:ident<$Minors:ident> ($minors:ident)
    )*) => {
        grid!(impl [ITEM] AsRef as_ref get_unchecked);
        grid!(impl [ITEM] AsMut as_mut get_unchecked_mut (mut));

        $(
            // Major
            grid!(impl [SLICE] $Type $GridMajor $Major $major AsRef as_ref get_unchecked);
            grid!(impl [SLICE] $Type $GridMajor $Major $major AsMut as_mut get_unchecked_mut (mut));

            // Minor
            grid!(impl [ITER] $Type $M $GridMinor $Minor $minor AsRef Index1D Minor);
            grid!(impl [ITER] $Type $M $GridMinor $Minor $minor AsMut Index1D MinorMut (mut));

            // Majors
            grid!(impl [ITER] $Type $M $GridMajors $Majors $majors AsRef Index2D Majors);
            grid!(impl [ITER] $Type $M $GridMajors $Majors $majors AsMut Index2D MajorsMut (mut));

            // Minors
            grid!(impl [ITER] $Type $M $GridMinors $Minors $minors AsRef Index2D Minors);

            // Items
            grid!(impl [ITEMS] $Type $GridMajors $Majors $majors AsRef);
            grid!(impl [ITEMS] $Type $GridMajors $Majors $majors AsMut (mut));
        )*
    };
    (impl [ITEM] $As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, M: Major, I, T: $As<[I]>> Grid for &'a $($mut)? Grid1D<M, I, T> {
            type Item = &'a $($mut)? I;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                let msize = self.msize();
                let index = index0d(index.unchecked(), msize);

                self.items.$as().$get(index)
            }
        }
    };
    (impl [SLICE] $Type:ident $Trait:ident $Assoc:ident $fn:ident $As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, I, T: $As<[I]>> $Trait for &'a $($mut)? $Type<I, T> {
            type $Assoc = &'a $($mut)? [I];

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                let index = index1d(index.$fn(self.size()), self.msize());

                self.items.$as().$get(index)
            }
        }
    };
    (impl [ITER]
        $Type:ident $M:ident
        $Trait:ident $Assoc:ident $fn:ident
        $As:ident
        $Index:ident
        $Iter:ident
        $(($mut:ident))?
    ) => {
        impl<'a, I, T: $As<[I]>> $Trait for &'a $($mut)? $Type<I, T> {
            type $Assoc = iter::$Iter<'a, $M, I, T>;

            unsafe fn $fn(self, index: impl $Index) -> Self::$Assoc {
                Self::$Assoc::new_unchecked(self, index)
            }
        }
    };
    (impl [ITEMS]
        $Type:ident
        $GridMajors:ident $Majors:ident $majors:ident
        $As:ident $(($mut:ident))?
    ) => {
        impl<'a, I, T: $As<[I]>> GridItems for &'a $($mut)? $Type<I, T> {
            type Items = std::iter::Flatten<<Self as $GridMajors>::$Majors>;

            unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
                self.$majors(index).flatten()
            }
        }
    };
}

grid!(
    RowGrid1D<RowMajor>
        GridRow<Row> (row_unchecked)
        GridCol<Col> (col_unchecked)
        GridRows<Rows> (rows_unchecked)
        GridCols<Cols> (cols_unchecked)
    ColGrid1D<ColMajor>
        GridCol<Col> (col_unchecked)
        GridRow<Row> (row_unchecked)
        GridCols<Cols> (cols_unchecked)
        GridRows<Rows> (rows_unchecked)
);

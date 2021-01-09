use crate::*;
use std::marker::PhantomData;

/// Error type for [`Grid2D`](crate::Grid2D) constructor.
/// TODO impl std::error::Error
#[derive(Copy, Clone, Debug)]
pub enum Grid2DError<T> {
    /// `width * height > usize::MAX`.
    Overflow(T),
    /// `major != U::len`.
    MajorMismatch(T, usize),
    /// `minor != T::len`.
    MinorMismatch(T),
}

pub struct Grid2D<M, I, T, U> {
    size:    M,
    items:   T,
    phantom: PhantomData<(I, U)>,
}

/// ### Constructors
impl<M: Major, I, T, U> Grid2D<M, I, T, U> {
    /// Creates a new [`Grid1D`](crate::Grid1D), without errors checking.
    pub fn new_unchecked(size: Size, items: T) -> Self {
        Self {
            size: size.into(),
            items,
            phantom: PhantomData,
        }
    }

    /// Creates a new [`Grid1D`](crate::Grid1D)
    /// or returns a [`Grid2DError`](Grid2DError).
    pub fn new(size: Size, items: T) -> Result<Self, Grid2DError<T>>
    where
        T: AsRef<[U]>,
        U: AsRef<[I]>,
    {
        let m = M::from(size);
        let major = m.major();
        let minor = m.minor();

        match size.width.checked_mul(size.height) {
            None => Err(Grid2DError::Overflow(items)),
            Some(_) =>
                if minor != items.as_ref().len() {
                    Err(Grid2DError::MinorMismatch(items))
                } else {
                    for (i, slice) in items.as_ref().iter().enumerate() {
                        if major != slice.as_ref().len() {
                            return Err(Grid2DError::MajorMismatch(items, i));
                        }
                    }

                    Ok(Self::new_unchecked(size, items))
                },
        }
    }
}

impl<M, I, T: AsRef<[U]>, U> AsRef<[U]> for Grid2D<M, I, T, U> {
    fn as_ref(&self) -> &[U] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[U]>, U> AsMut<[U]> for Grid2D<M, I, T, U> {
    fn as_mut(&mut self) -> &mut [U] {
        self.items.as_mut()
    }
}

impl<M: Major, I, T, U> WithSize for Grid2D<M, I, T, U> {
    fn size(&self) -> Size {
        self.size.into()
    }
}

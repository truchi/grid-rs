mod col_major;
mod row_major;

pub use col_major::*;
pub use row_major::*;

use crate::*;
use std::{convert::TryFrom, marker::PhantomData};

/// Error type for [`Grid1D`](crate::Grid1D) constructors.
#[derive(Copy, Clone, Debug)]
pub enum Grid1DError<T> {
    /// `width * height > usize::MAX`.
    Overflow(Size<usize>, T),
    /// `width * height != len`.
    Mismatch(Size<usize>, T),
}

/// 2D [`Grid1D`](crate::Grid1D).
///
/// @see [`ColMajor1D`](crate::ColMajor1D) and
/// [`RowMajor1D`](crate::RowMajor1D).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Grid1D<Major, Cell, Collection> {
    phantom: PhantomData<(Major, Cell)>,
    size:    Size<usize>,
    cells:   Collection,
}

/// ### Constructors
impl<Major, Cell, Collection: AsRef<[Cell]>> Grid1D<Major, Cell, Collection> {
    /// Creates a new [`Grid1D`](crate::Grid1D)
    /// or returns a [`Grid1DError`](Grid1DError).
    pub fn new<S: Into<Size<usize>>>(
        size: S,
        cells: Collection,
    ) -> Result<Self, Grid1DError<Collection>> {
        let size = size.into();

        match size.width.checked_mul(size.height) {
            None => Err(Grid1DError::Overflow(size, cells)),
            Some(area) =>
                if area != cells.as_ref().len() {
                    Err(Grid1DError::Mismatch(size, cells))
                } else {
                    Ok(Self {
                        size,
                        cells,
                        phantom: PhantomData,
                    })
                },
        }
    }
}

/// ### Methods
impl<Major, Cell, Collection: AsRef<[Cell]>> Grid1D<Major, Cell, Collection> {
    /// Returns the [`Size`](crate::Size).
    pub fn size(&self) -> Size<usize> {
        self.size
    }
}

impl<Major, Cell, Collection: AsRef<[Cell]>, S: Into<Size<usize>>> TryFrom<(S, Collection)>
    for Grid1D<Major, Cell, Collection>
{
    type Error = Grid1DError<Collection>;

    fn try_from((size, cells): (S, Collection)) -> Result<Self, Self::Error> {
        Self::new(size, cells)
    }
}

impl<Major, Cell, Collection: AsRef<[Cell]>> AsRef<[Cell]> for Grid1D<Major, Cell, Collection> {
    fn as_ref(&self) -> &[Cell] {
        self.cells.as_ref()
    }
}

impl<Major, Cell, Collection: AsMut<[Cell]>> AsMut<[Cell]> for Grid1D<Major, Cell, Collection> {
    fn as_mut(&mut self) -> &mut [Cell] {
        self.cells.as_mut()
    }
}

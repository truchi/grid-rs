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
/// RowMajor:      ColumnMajor:
/// - cell         - cell
/// - row          - col
/// - row_iter     - col_iter
/// - col_iter     - row_iter
/// - cells_iter   - cells_iter
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

/// ### Methods
impl<Cell, Collection: AsRef<[Cell]>> Grid1D<RowMajor, Cell, Collection> {
    pub unsafe fn get_row_unchecked(&self, index: impl Index1D) -> &[Cell] {
        self.cells
            .as_ref()
            .get_unchecked(RowMajor::row_unchecked(self.size, index))
    }

    pub fn get_row(&self, index: impl Index1D) -> Option<&[Cell]> {
        let range = RowMajor::row(self.size, index)?;
        let cells = self.cells.as_ref();

        // SAFETY:
        // RowMajor::row does the bounds checking
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= self.size.width * self.size.height);
        Some(unsafe { cells.get_unchecked(range) })
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

// ==================================================================================================== //
// ==================================================================================================== //
// ==================================================================================================== //
// ==================================================================================================== //
// ==================================================================================================== //

impl<'a, Cell: 'a, Collection: 'a + AsRef<[Cell]>> IGrid<'a>
    for Grid1D<RowMajor, Cell, Collection>
{
    type Cell = Cell;
    type Cells = std::iter::Flatten<Self::Rows>;
    type Col = ColHelper<'a, Self>;
    type Cols = ColsHelper<'a, Self>;
    type Row = std::slice::Iter<'a, Cell>;
    type Rows = RowsHelper<'a, Self>;

    fn size(&self) -> Size<usize> {
        self.size
    }

    unsafe fn cell_unchecked(&self, point: Point<usize>) -> &Cell {
        self.cells
            .as_ref()
            .get_unchecked(RowMajor::cell_unchecked(self.size, point))
    }

    unsafe fn row_unchecked(&'a self, index: impl Index1D) -> Self::Row {
        self.get_row_unchecked(index).iter()
    }

    unsafe fn col_unchecked(&'a self, index: impl Index1D) -> Self::Col {
        ColHelper::new_unchecked(self, index)
    }

    unsafe fn rows_unchecked(&'a self, index: impl Index2D) -> Self::Rows {
        RowsHelper::new_unchecked(self, index)
    }

    unsafe fn cols_unchecked(&'a self, index: impl Index2D) -> Self::Cols {
        ColsHelper::new_unchecked(self, index)
    }

    unsafe fn cells_unchecked(&'a self, index: impl Index2D) -> Self::Cells {
        self.rows_unchecked(index).flatten()
    }
}

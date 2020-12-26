use crate::*;

pub struct RowMutHelper<'a, T> {
    grid:  &'a mut T,
    row:   usize,
    range: std::ops::Range<usize>,
}

impl<'a, T: IGridMut<'a>> RowMutHelper<'a, T> {
    /// ### Safety
    /// callers **MUST** ensure:
    /// - cell_mut_unchecked returns valid, non-overlapping references
    pub unsafe fn new(grid: &'a mut T, index: impl Index1D) -> Option<Self> {
        let (width, height) = grid.size().into();
        let (row, range) = index.checked(height, width)?;

        // SAFETY:
        // Index1D::checked guaranties that:
        debug_assert!(row < height);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= width);
        #[allow(unused_unsafe)]
        Some(unsafe { Self::new_unchecked(grid, (row, range)) })
    }

    /// ### Safety
    /// callers **MUST** ensure:
    /// - cell_mut_unchecked returns valid, non-overlapping references
    /// - `row < height`
    /// - `start <= end`
    /// - `end <= width`
    pub unsafe fn new_unchecked(grid: &'a mut T, index: impl Index1D) -> Self {
        let (row, range) = index.unchecked(grid.size().width);

        Self { grid, row, range }
    }
}

impl<'a, T: IGridMut<'a>> Iterator for RowMutHelper<'a, T> {
    type Item = &'a mut T::Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let std::ops::Range { start, end } = self.range;

        if start < end {
            self.range.start += 1;
            let point = Point {
                x: start,
                y: self.row,
            };

            // SAFETY:
            // constructors guaranty that:
            debug_assert!(point < self.grid.size());
            let cell = unsafe { self.grid.cell_mut_unchecked(point) };

            // SAFETY:
            // constructors also guaranty that:
            // cell_mut_unchecked returns valid, non-overlapping references.
            // Then, it is safe to extend cell's lifetime
            let cell = unsafe { std::mem::transmute::<&mut T::Cell, &mut T::Cell>(cell) };

            Some(cell)
        } else {
            None
        }
    }
}

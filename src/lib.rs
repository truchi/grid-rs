// #![warn(missing_docs)]
// #![allow(unused)]

#[macro_use]
mod macros;

mod helpers;
mod traits;
mod types;

pub use helpers::*;
pub use traits::*;
pub use types::*;

#[derive(PartialEq, Debug)]
struct A(Vec<usize>);

impl WithSize for A {
    fn size(&self) -> Size<usize> {
        Size {
            width:  3,
            height: 3,
        }
    }
}

impl<'a> Grid<&'a usize> for &'a A {
    type Col = Col<&'a usize, &'a A>;
    type Cols = std::vec::IntoIter<Self::Col>;
    type Items = std::slice::Iter<'a, usize>;
    type Row = std::slice::Iter<'a, usize>;
    type Rows = std::vec::IntoIter<Self::Row>;

    unsafe fn cell_unchecked(self, point: Point<usize>) -> &'a usize {
        &self.0[point.y * self.size().width + point.x]
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Col::<&usize, &A>::new(self, index).unwrap()
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        todo!()
    }

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        todo!()
    }

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        todo!()
    }

    unsafe fn cells_unchecked(self, index: impl Index2D) -> Self::Items {
        todo!()
    }
}

impl<'a> Grid<&'a mut usize> for &'a mut A {
    type Col = Col<&'a mut usize, &'a mut A>;
    type Cols = std::vec::IntoIter<Self::Col>;
    type Items = std::slice::IterMut<'a, usize>;
    type Row = std::slice::IterMut<'a, usize>;
    type Rows = std::vec::IntoIter<Self::Row>;

    unsafe fn cell_unchecked(self, point: Point<usize>) -> &'a mut usize {
        // &mut self.0[0]
        let width = self.size().width;
        &mut self.0[point.y * width + point.x]
    }

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Col::<&mut usize, &mut A>::new(self, index).unwrap()
    }

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        todo!()
    }

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        todo!()
    }

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        todo!()
    }

    unsafe fn cells_unchecked(self, index: impl Index2D) -> Self::Items {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn a() {
    // let mut a = A(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    //
    // assert_eq!((&a).col(0).unwrap().copied().collect::<Vec<_>>(), &[
    // 0, 3, 6
    // ]);
    //
    // assert_eq!((&mut a).col(0).unwrap().map(|u| *u).collect::<Vec<_>>(), &[
    // 0, 3, 6
    // ]);
    // }

    #[test]
    fn b() {
        let mut a = A(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

        let mut idx = 0;
        for i in (&mut a).col(0).unwrap() {
            assert!(*i != 10, "{}", idx);
            *i = 10;
            idx += 1;
        }
    }
}

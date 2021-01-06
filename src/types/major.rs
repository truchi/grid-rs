use crate::*;
use std::ops::Range;

/// Trait for [`ColMajor`](crate::ColMajor) and [`RowMajor`](crate::RowMajor)
/// [`Size`](crate::Size)s of [`Grid1D`](crate::Grid1D)s.
pub trait Major: From<Size<usize>> + Into<Size<usize>> + Copy {
    /// Returns a new `Self` from the lengths of the major axis `major`
    /// and minor axis `minor`.
    fn new(major: usize, minor: usize) -> Self;

    /// Returns the length on the major axis.
    fn major(self) -> usize;

    /// Returns the length on the minor axis.
    fn minor(self) -> usize;

    /// Returns the index at `point` if `point < size`,
    /// `None` otherwise.
    fn index(self, point: Point<usize>) -> Option<usize> {
        if point < self.into() {
            Some(self.index_unchecked(point))
        } else {
            None
        }
    }

    /// Returns the index at `point`, without checking bounds.
    fn index_unchecked(self, point: Point<usize>) -> usize {
        let point = Self::from(point.as_size());

        point.minor() * self.major() + point.major()
    }

    /// Returns the range of the `index` on the major axis if `index < size`,
    /// `None` otherwise.
    fn range(self, index: impl Index1D) -> Option<Range<usize>> {
        let index = index.checked(self.minor(), self.major())?;

        Some(self.range_unchecked(index))
    }

    /// Returns the range of the `index` on the major axis if `index < size`,
    /// without checking bounds.
    fn range_unchecked(self, index: impl Index1D) -> Range<usize> {
        let (i, Range { start, end }) = index.unchecked(self.major());

        let point = Self::new(start, i).into().as_point();
        let start = self.index_unchecked(point);

        start..start + end
    }
}

macro_rules! majors {
    ($($(#[$meta:meta])* $Major:ident $major:ident $minor:ident)*) => { $(
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct $Major {
            pub width:  usize,
            pub height: usize,
        }

        impl From<Size<usize>> for $Major {
            fn from(Size { width, height }: Size<usize>) -> Self {
                Self { width, height }
            }
        }

        impl From<$Major> for Size<usize> {
            fn from($Major { width, height }: $Major) -> Self {
                Self { width, height }
            }
        }

        impl Major for $Major {
            fn new(major: usize, minor: usize) -> Self {
                Self { $major: major, $minor: minor }
            }

            fn major(self) -> usize { self.$major }
            fn minor(self) -> usize { self.$minor }
        }
    )* };
}

majors!(
    /// A [`Size`](crate::Size) for [`ColMajor`](crate::ColMajor) [`Grid1D`](crate::Grid1D)s.
    ColMajor height width
    /// A [`Size`](crate::Size) for [`RowMajor`](crate::RowMajor) [`Grid1D`](crate::Grid1D)s.
    RowMajor width height
);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const WIDTH: usize = 3;
    const HEIGHT: usize = 2;
    const COL_MAJOR: ColMajor = ColMajor {
        width:  WIDTH,
        height: HEIGHT,
    };
    const ROW_MAJOR: RowMajor = RowMajor {
        width:  WIDTH,
        height: HEIGHT,
    };

    #[test]
    fn major() {
        assert_eq!(ColMajor::new(2, 3), COL_MAJOR);
        assert_eq!(RowMajor::new(3, 2), ROW_MAJOR);

        assert_eq!(COL_MAJOR.major(), 2);
        assert_eq!(ROW_MAJOR.major(), 3);

        assert_eq!(COL_MAJOR.minor(), 3);
        assert_eq!(ROW_MAJOR.minor(), 2);
    }

    #[test]
    fn index() {
        let col_indexes = [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)];
        let row_indexes = [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)];
        let out_of_bounds = [
            (WIDTH - 1, HEIGHT),
            (WIDTH, HEIGHT - 1),
            (WIDTH, HEIGHT),
            (WIDTH + 1, HEIGHT + 1),
        ];

        for (i, (x, y)) in col_indexes.iter().copied().enumerate() {
            assert_eq!(COL_MAJOR.index(Point { x, y }), Some(i));
            assert_eq!(COL_MAJOR.index_unchecked(Point { x, y }), i);
        }

        for (i, (x, y)) in row_indexes.iter().copied().enumerate() {
            assert_eq!(ROW_MAJOR.index(Point { x, y }), Some(i));
            assert_eq!(ROW_MAJOR.index_unchecked(Point { x, y }), i);
        }

        for (x, y) in out_of_bounds.iter().copied() {
            assert_eq!(COL_MAJOR.index(Point { x, y }), None);
            assert_eq!(ROW_MAJOR.index(Point { x, y }), None);
        }
    }

    #[test]
    fn range() {
        let col_ranges = [0..2, 2..4, 4..6];
        let row_ranges = [0..3, 3..6];

        for (i, range) in col_ranges.iter().enumerate() {
            assert_eq!(COL_MAJOR.range(i), Some(range.clone()));
            assert_eq!(COL_MAJOR.range_unchecked(i), range.clone());
        }

        for (i, range) in row_ranges.iter().enumerate() {
            assert_eq!(ROW_MAJOR.range(i), Some(range.clone()));
            assert_eq!(ROW_MAJOR.range_unchecked(i), range.clone());
        }

        assert_eq!(COL_MAJOR.range(col_ranges.len()), None);
        assert_eq!(ROW_MAJOR.range(row_ranges.len()), None);
    }
}

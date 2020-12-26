use crate::*;
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub trait Index2D {
    fn unchecked(self, size: Size<usize>) -> Point<Range<usize>>;
    fn checked(self, size: Size<usize>) -> Option<Point<Range<usize>>>;
}

macro_rules! ranges {
    ($(($X:ty, $Y:ty))*) => { $(ranges!(impl ($X, $Y) ($Y, $X));)* };
    (impl $($Type:ty)*) => { $(
        impl Index2D for $Type {
            fn unchecked(self, size: Size<usize>) -> Point<Range<usize>> {
                Point {
                    x: ToRange::unchecked(self.0, size.width),
                    y: ToRange::unchecked(self.1, size.height),
                }
            }

            fn checked(self, size: Size<usize>) -> Option<Point<Range<usize>>> {
                Some(Point {
                    x: ToRange::checked(self.0, size.width)?,
                    y: ToRange::checked(self.1, size.height)?,
                })
            }
        }
    )* };
}

ranges!(impl
    (Range<usize>, Range<usize>)
    (RangeInclusive<usize>, RangeInclusive<usize>)
    (RangeFrom<usize>, RangeFrom<usize>)
    (RangeTo<usize>, RangeTo<usize>)
    (RangeToInclusive<usize>, RangeToInclusive<usize>)
    (RangeFull, RangeFull)
    ((Bound<usize>, Bound<usize>), (Bound<usize>, Bound<usize>))
);

ranges!(
    (Range<usize>, RangeInclusive<usize>)
    (Range<usize>, RangeFrom<usize>)
    (Range<usize>, RangeTo<usize>)
    (Range<usize>, RangeToInclusive<usize>)
    (Range<usize>, RangeFull)
    (Range<usize>, (Bound<usize>, Bound<usize>))

    (RangeInclusive<usize>, RangeFrom<usize>)
    (RangeInclusive<usize>, RangeTo<usize>)
    (RangeInclusive<usize>, RangeToInclusive<usize>)
    (RangeInclusive<usize>, RangeFull)
    (RangeInclusive<usize>, (Bound<usize>, Bound<usize>))

    (RangeFrom<usize>, RangeTo<usize>)
    (RangeFrom<usize>, RangeToInclusive<usize>)
    (RangeFrom<usize>, RangeFull)
    (RangeFrom<usize>, (Bound<usize>, Bound<usize>))

    (RangeTo<usize>, RangeToInclusive<usize>)
    (RangeTo<usize>, RangeFull)
    (RangeTo<usize>, (Bound<usize>, Bound<usize>))

    (RangeToInclusive<usize>, RangeFull)
    (RangeToInclusive<usize>, (Bound<usize>, Bound<usize>))

    (RangeFull, (Bound<usize>, Bound<usize>))
);
/*
RangeInclusive<usize>
RangeFrom<usize>
RangeTo<usize>
RangeToInclusive<usize>
RangeFull
*/

// fn unchecked(self, size: Size<usize>) -> Point<Range<usize>> { todo!() }
// fn checked(self, size: Size<usize>) -> Option<Point<Range<usize>>> { todo!()
// }

/*
impl Index2D for (usize, Range<usize>) {
    fn unchecked(self, size: Size<usize>) -> Point<Range<usize>> {
        todo!()
    }

    fn checked(self, size: Size<usize>) -> Option<Point<Range<usize>>> {
        todo!()
    }
}

impl Index1D for (usize, Range<usize>) {
    fn unchecked(self, _: usize) -> (usize, Range<usize>) {
        self
    }

    fn checked(self, len: usize) -> Option<(usize, Range<usize>)> {
        let (i, Range { start, end }) = self;

        if i < len && start <= end && end <= len {
            Some((i, start..end))
        } else {
            None
        }
    }
}

impl Index1D for (usize, RangeInclusive<usize>) {
    fn unchecked(self, _: usize) -> (usize, Range<usize>) {
        let (i, (start, end)) = (self.0, self.1.into_inner());

        (i, start..end + 1)
    }

    fn checked(self, len: usize) -> Option<(usize, Range<usize>)> {
        let (i, (start, end)) = (self.0, self.1.into_inner());

        (i, start..end.checked_add(1)?).checked(len)
    }
}

impl Index1D for (usize, RangeFrom<usize>) {
    fn unchecked(self, len: usize) -> (usize, Range<usize>) {
        let (i, RangeFrom { start }) = self;

        (i, start..len)
    }

    fn checked(self, len: usize) -> Option<(usize, Range<usize>)> {
        let (i, RangeFrom { start }) = self;

        if i < len && start <= len {
            Some((i, start..len))
        } else {
            None
        }
    }
}

impl Index1D for (usize, RangeTo<usize>) {
    fn unchecked(self, _: usize) -> (usize, Range<usize>) {
        let (i, RangeTo { end }) = self;

        (i, 0..end)
    }

    fn checked(self, len: usize) -> Option<(usize, Range<usize>)> {
        let (i, RangeTo { end }) = self;

        if i < len && end <= len {
            Some((i, 0..end))
        } else {
            None
        }
    }
}

impl Index1D for (usize, RangeToInclusive<usize>) {
    fn unchecked(self, _: usize) -> (usize, Range<usize>) {
        let (i, RangeToInclusive { end }) = self;

        (i, 0..end + 1)
    }

    fn checked(self, len: usize) -> Option<(usize, Range<usize>)> {
        let (i, RangeToInclusive { end }) = self;

        (i, 0..end.checked_add(1)?).checked(len)
    }
}

impl Index1D for (usize, RangeFull) {
    fn unchecked(self, len: usize) -> (usize, Range<usize>) {
        let (i, _) = self;

        (i, 0..len)
    }

    fn checked(self, len: usize) -> Option<(usize, Range<usize>)> {
        let (i, _) = self;

        if i < len {
            Some((i, 0..len))
        } else {
            None
        }
    }
}

impl Index1D for (usize, Bound<usize>, Bound<usize>) {
    fn unchecked(self, len: usize) -> (usize, Range<usize>) {
        (self.0, match (self.1, self.2) {
            (Included(start), Excluded(end)) => start..end,
            (Included(start), Included(end)) => start..end + 1,
            (Included(start), Unbounded) => start..len,
            (Unbounded, Excluded(end)) => 0..end,
            (Unbounded, Included(end)) => 0..end + 1,
            (Unbounded, Unbounded) => 0..len,
            (Excluded(start), Excluded(end)) => start + 1..end,
            (Excluded(start), Included(end)) => start + 1..end + 1,
            (Excluded(start), Unbounded) => start + 1..len,
        })
    }

    fn checked(self, len: usize) -> Option<(usize, Range<usize>)> {
        let (i, start, end) = self;

        match (start, end) {
            (Included(start), Excluded(end)) => (i, start..end).checked(len),
            (Included(start), Included(end)) => (i, start..=end).checked(len),
            (Included(start), Unbounded) => (i, start..).checked(len),
            (Unbounded, Excluded(end)) => (i, ..end).checked(len),
            (Unbounded, Included(end)) => (i, ..=end).checked(len),
            (Unbounded, Unbounded) => (i, ..).checked(len),
            (Excluded(start), Excluded(end)) => (i, start.checked_add(1)?..end).checked(len),
            (Excluded(start), Included(end)) => (i, start.checked_add(1)?..=end).checked(len),
            (Excluded(start), Unbounded) => (i, start.checked_add(1)?..).checked(len),
        }
    }
}
*/

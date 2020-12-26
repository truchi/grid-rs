use crate::*;
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub trait Index1D {
    fn unchecked(self, max: usize) -> (usize, Range<usize>);
    fn checked(self, max: (usize, usize)) -> Option<(usize, Range<usize>)>;
}

impl Index1D for usize {
    fn unchecked(self, max: usize) -> (usize, Range<usize>) {
        (self, 0..max)
    }

    fn checked(self, max: (usize, usize)) -> Option<(usize, Range<usize>)> {
        if self < max.0 {
            Some((self, 0..max.1))
        } else {
            None
        }
    }
}

impl Index1D for (usize, Bound<usize>, Bound<usize>) {
    fn unchecked(self, max: usize) -> (usize, Range<usize>) {
        (self.0, ToRange::unchecked((self.1, self.2), max))
    }

    fn checked(self, max: (usize, usize)) -> Option<(usize, Range<usize>)> {
        let (i, start, end) = self;

        if i < max.0 {
            Some((i, ToRange::checked((start, end), max.1)?))
        } else {
            None
        }
    }
}

macro_rules! ranges {
    ($($Type:ty)*) => { $(
        impl Index1D for (usize, $Type) {
            fn unchecked(self, max: usize) -> (usize, Range<usize>) {
                (self.0, ToRange::unchecked(self.1, max))
            }

            fn checked(self, max: (usize, usize)) -> Option<(usize, Range<usize>)> {
                let (i, range) = self;

                if i < max.0 { Some((i, ToRange::checked(range, max.1)?)) } else { None }
            }
        }
    )* };
}

ranges!(
    Range<usize>
    RangeInclusive<usize>
    RangeFrom<usize>
    RangeTo<usize>
    RangeToInclusive<usize>
    RangeFull
);

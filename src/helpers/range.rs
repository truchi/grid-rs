use std::ops::{
    Bound::{self, *},
    Range,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive,
};

pub trait ToRange {
    fn unchecked(self, len: usize) -> Range<usize>;
    fn checked(self, len: usize) -> Option<Range<usize>>;
}

impl ToRange for Range<usize> {
    fn unchecked(self, _: usize) -> Range<usize> {
        self
    }

    fn checked(self, len: usize) -> Option<Range<usize>> {
        let Range { start, end } = self;

        if start <= end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }
}

impl ToRange for RangeInclusive<usize> {
    fn unchecked(self, _: usize) -> Range<usize> {
        let (start, end) = self.into_inner();

        start..end + 1
    }

    fn checked(self, len: usize) -> Option<Range<usize>> {
        let (start, end) = self.into_inner();

        (start..end.checked_add(1)?).checked(len)
    }
}

impl ToRange for RangeFrom<usize> {
    fn unchecked(self, len: usize) -> Range<usize> {
        let RangeFrom { start } = self;

        start..len
    }

    fn checked(self, len: usize) -> Option<Range<usize>> {
        let RangeFrom { start } = self;

        if start <= len {
            Some(start..len)
        } else {
            None
        }
    }
}

impl ToRange for RangeTo<usize> {
    fn unchecked(self, _: usize) -> Range<usize> {
        let RangeTo { end } = self;

        0..end
    }

    fn checked(self, len: usize) -> Option<Range<usize>> {
        let RangeTo { end } = self;

        if end <= len {
            Some(0..end)
        } else {
            None
        }
    }
}

impl ToRange for RangeToInclusive<usize> {
    fn unchecked(self, _: usize) -> Range<usize> {
        let RangeToInclusive { end } = self;

        0..end + 1
    }

    fn checked(self, len: usize) -> Option<Range<usize>> {
        let RangeToInclusive { end } = self;

        (0..end.checked_add(1)?).checked(len)
    }
}

impl ToRange for RangeFull {
    fn unchecked(self, len: usize) -> Range<usize> {
        0..len
    }

    fn checked(self, len: usize) -> Option<Range<usize>> {
        Some(0..len)
    }
}

impl ToRange for (Bound<usize>, Bound<usize>) {
    fn unchecked(self, len: usize) -> Range<usize> {
        match self {
            (Included(start), Excluded(end)) => start..end,
            (Included(start), Included(end)) => start..end + 1,
            (Included(start), Unbounded) => start..len,
            (Unbounded, Excluded(end)) => 0..end,
            (Unbounded, Included(end)) => 0..end + 1,
            (Unbounded, Unbounded) => 0..len,
            (Excluded(start), Excluded(end)) => start + 1..end,
            (Excluded(start), Included(end)) => start + 1..end + 1,
            (Excluded(start), Unbounded) => start + 1..len,
        }
    }

    fn checked(self, len: usize) -> Option<Range<usize>> {
        match self {
            (Included(start), Excluded(end)) => (start..end).checked(len),
            (Included(start), Included(end)) => (start..=end).checked(len),
            (Included(start), Unbounded) => (start..).checked(len),
            (Unbounded, Excluded(end)) => (..end).checked(len),
            (Unbounded, Included(end)) => (..=end).checked(len),
            (Unbounded, Unbounded) => (..).checked(len),
            (Excluded(start), Excluded(end)) => (start.checked_add(1)?..end).checked(len),
            (Excluded(start), Included(end)) => (start.checked_add(1)?..=end).checked(len),
            (Excluded(start), Unbounded) => (start.checked_add(1)?..).checked(len),
        }
    }
}

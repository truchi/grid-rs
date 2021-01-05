use std::ops::{Bound::*, Range, RangeBounds};

/// Converts `T: RangeBounds<usize>` to `Range<usize>`.
pub trait ToRange {
    /// Converts to `Range` with bounds checking.
    fn checked(self, len: usize) -> Option<Range<usize>>;

    /// Converts to `Range` without bounds checking.
    fn unchecked(self, len: usize) -> Range<usize>;
}

impl<T: RangeBounds<usize>> ToRange for T {
    fn checked(self, len: usize) -> Option<Range<usize>> {
        match to_start_end(self)? {
            (Start::Included(start), End::Excluded(end)) =>
                if start <= end && end <= len {
                    Some(start..end)
                } else {
                    None
                },
            (Start::Included(start), End::Unbounded) =>
                if start <= len {
                    Some(start..len)
                } else {
                    None
                },
            (Start::Unbounded, End::Excluded(end)) =>
                if end <= len {
                    Some(0..end)
                } else {
                    None
                },
            (Start::Unbounded, End::Unbounded) => Some(0..len),
        }
    }

    fn unchecked(self, len: usize) -> Range<usize> {
        match (self.start_bound(), self.end_bound()) {
            (Included(start), Excluded(end)) => *start..*end,
            (Included(start), Included(end)) => *start..end + 1,
            (Included(start), Unbounded) => *start..len,
            (Unbounded, Excluded(end)) => 0..*end,
            (Unbounded, Included(end)) => 0..end + 1,
            (Unbounded, Unbounded) => 0..len,
            (Excluded(start), Excluded(end)) => start + 1..*end,
            (Excluded(start), Included(end)) => start + 1..end + 1,
            (Excluded(start), Unbounded) => start + 1..len,
        }
    }
}

enum Start {
    Included(usize),
    Unbounded,
}

enum End {
    Excluded(usize),
    Unbounded,
}

/// Converts `impl RangeBounds<usize>` into `(Start, End)`, or `None` if
/// overflow.
fn to_start_end(range: impl RangeBounds<usize>) -> Option<(Start, End)> {
    Some(match (range.start_bound(), range.end_bound()) {
        (Included(start), Excluded(end)) => (Start::Included(*start), End::Excluded(*end)),
        (Included(start), Included(end)) =>
            (Start::Included(*start), End::Excluded(end.checked_add(1)?)),
        (Included(start), Unbounded) => (Start::Included(*start), End::Unbounded),
        (Unbounded, Excluded(end)) => (Start::Unbounded, End::Excluded(*end)),
        (Unbounded, Included(end)) => (Start::Unbounded, End::Excluded(end.checked_add(1)?)),
        (Unbounded, Unbounded) => (Start::Unbounded, End::Unbounded),
        (Excluded(start), Excluded(end)) =>
            (Start::Included(start.checked_add(1)?), End::Excluded(*end)),
        (Excluded(start), Included(end)) => (
            Start::Included(start.checked_add(1)?),
            End::Excluded(end.checked_add(1)?),
        ),
        (Excluded(start), Unbounded) => (Start::Included(start.checked_add(1)?), End::Unbounded),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::ops::Bound;

    #[test]
    fn checked() {
        macro_rules! assert {
            (Some $len:literal $($start:expr, $end:expr, $expected:expr)*) => { $( {
                let start: Bound<usize> = $start;
                let end: Bound<usize> = $end;

                assert_eq!(
                    (start, end).checked($len),
                    Some($expected),
                    "({:?}, {:?}).checked({:?}) == Some({:?})",
                    start,
                    end,
                    $len,
                    $expected
                );
            } )* };
            (None $len:literal $($start:expr, $end:expr)*) => { $( {
                let start: Bound<usize> = $start;
                let end: Bound<usize> = $end;

                assert_eq!(
                    (start, end).checked($len),
                    None,
                    "({:?}, {:?}).checked({:?}) == None",
                    start,
                    end,
                    $len,
                );
            } )* };
        }

        // It converts to Range with bounds checking
        assert!(Some 10
            Unbounded, Unbounded, 0..10
            Unbounded, Included(8), 0..9
            Unbounded, Excluded(8), 0..8
            Included(5), Unbounded, 5..10
            Included(5), Included(8), 5..9
            Included(5), Excluded(8), 5..8
            Excluded(5), Unbounded, 6..10
            Excluded(5), Included(8), 6..9
            Excluded(5), Excluded(8), 6..8
        );

        // It returns None when start > end
        assert!(None 10
            Included(7), Included(3)
            Included(7), Excluded(3)
            Excluded(7), Included(3)
            Excluded(7), Excluded(3)
        );

        // It returns None when end > len
        assert!(None 10
            Unbounded, Included(20)
            Unbounded, Excluded(20)
            Included(5), Included(20)
            Included(5), Excluded(20)
            Excluded(5), Included(20)
            Excluded(5), Excluded(20)
        );

        // It returns None when usize::MAX
        assert!(None 10
            Unbounded, Included(usize::MAX)
            Included(5), Included(usize::MAX)
            Excluded(5), Included(usize::MAX)
            Excluded(usize::MAX), Unbounded
            Excluded(usize::MAX), Included(8)
            Excluded(usize::MAX), Excluded(8)
        );
    }

    #[test]
    fn unchecked() {
        fn range(a: Bound<usize>, b: Bound<usize>) -> (Bound<usize>, Bound<usize>) {
            (a, b)
        }

        // It converts to Range without bounds checking
        assert_eq!(range(Unbounded, Unbounded).unchecked(10), 0..10);
        assert_eq!(range(Unbounded, Included(20)).unchecked(10), 0..21);
        assert_eq!(range(Unbounded, Excluded(20)).unchecked(10), 0..20);
        assert_eq!(range(Included(30), Unbounded).unchecked(10), 30..10);
        assert_eq!(range(Included(30), Included(20)).unchecked(10), 30..21);
        assert_eq!(range(Included(30), Excluded(20)).unchecked(10), 30..20);
        assert_eq!(range(Excluded(30), Unbounded).unchecked(10), 31..10);
        assert_eq!(range(Excluded(30), Included(20)).unchecked(10), 31..21);
        assert_eq!(range(Excluded(30), Excluded(20)).unchecked(10), 31..20);
    }
}

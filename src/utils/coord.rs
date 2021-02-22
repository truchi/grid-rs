use std::{cmp::Ordering, ops::Range};

pub type Point<X = usize, Y = X> = Coord<X, Y>;
pub type Size<X = usize, Y = X> = Coord<X, Y>;
pub type Rect = Coord<Range<usize>>;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Coord<X = usize, Y = X> {
    pub x: X,
    pub y: Y,
}

impl<X: PartialOrd, Y: PartialOrd> PartialOrd for Coord<X, Y> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.x.partial_cmp(&other.x), self.y.partial_cmp(&other.y)) {
            (Some(self_ord), Some(other_ord)) if self_ord == other_ord => Some(self_ord),
            _ => None,
        }
    }
}

impl<X, Y> From<Coord<X, Y>> for (X, Y) {
    fn from(coord: Coord<X, Y>) -> Self {
        (coord.x, coord.y)
    }
}

impl<X, Y> From<(X, Y)> for Coord<X, Y> {
    fn from(coord: (X, Y)) -> Self {
        Self {
            x: coord.0,
            y: coord.1,
        }
    }
}

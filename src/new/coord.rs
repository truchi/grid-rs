use std::cmp::Ordering;

pub type Point = Coord;
pub type Size = Coord;

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

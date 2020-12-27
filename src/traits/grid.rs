use crate::*;

pub trait Grid {
    /// The type of the elements.
    type Cell;

    /// Returns the [`Size`](crate::Size) of this `grid`.
    fn size(&self) -> Size<usize>;
}

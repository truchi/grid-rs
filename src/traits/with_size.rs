use crate::*;

/// Trait fo rtypes with [`Size`](crate::Size).
pub trait WithSize {
    /// Returns the [`Size`](crate::Size).
    fn size(&self) -> Size<usize>;
}

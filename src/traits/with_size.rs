use crate::*;

/// Trait for types with [`Size`](crate::Size).
pub trait WithSize {
    /// Returns the [`Size`](crate::Size).
    fn size(&self) -> Size<usize>;
}

impl<T: std::ops::Deref<Target = U>, U: WithSize> WithSize for T {
    fn size(&self) -> Size<usize> {
        self.deref().size()
    }
}

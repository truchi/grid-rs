// #![warn(missing_docs)]
#![allow(unused)]

#[macro_use]
mod macros;

// mod cloned;
mod coord;
mod grid;
pub mod grid_1d;
mod index;
mod major;
mod range;
// mod repeat;

pub use self::grid::*;
// pub use cloned::*;
pub use coord::*;
pub use index::*;
pub use major::*;
pub use range::*;
// pub use repeat::*;

pub trait WithSize {
    fn size(&self) -> Size;
}

pub trait WithMSize<M: Major>: WithSize {
    fn msize(&self) -> M {
        self.size().into()
    }
}

impl<T: std::ops::Deref<Target = U>, U: WithSize> WithSize for T {
    fn size(&self) -> Size {
        self.deref().size()
    }
}

impl<M: Major, T: std::ops::Deref<Target = U>, U: WithMSize<M>> WithMSize<M> for T {
    fn msize(&self) -> M {
        self.deref().msize()
    }
}

// #![warn(missing_docs)]
// #![allow(unused)]

#[macro_use]
mod utils;

mod cloned;
mod grid;
pub mod grid1d;
mod index;
mod major;
// mod repeat;

pub use self::grid::*;
pub use cloned::*;
pub use index::*;
pub use major::*;
pub use utils::*;
// pub use repeat::*;

pub trait WithSize {
    fn size(&self) -> Size;
}

trait WithMSize<M: Major>: WithSize {
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

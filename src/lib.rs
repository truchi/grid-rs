// #![warn(missing_docs)]
// #![allow(unused)]

#[macro_use]
mod utils;

mod cloned;
mod crop;
mod grid;
pub mod grid1d;
mod index;
mod major;
pub mod repeat;
pub mod repeat_with;
mod with_msize;
mod with_size;

pub use self::grid::*;
pub use cloned::*;
pub use crop::*;
pub use grid1d::{ColGrid1D, Grid1D, RowGrid1D};
pub use index::*;
pub use major::*;
pub use repeat::Repeat;
pub use repeat_with::RepeatWith;
pub use utils::*;
pub(crate) use with_msize::*;
pub use with_size::*;

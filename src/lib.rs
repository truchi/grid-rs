// TODO: tests, docs, examples
// #![warn(missing_docs)]

pub mod grid1d;
pub mod repeat;
pub mod repeat_with;

pub use grid1d::{ColGrid1D, Grid1D, RowGrid1D};
pub use repeat::{repeat, Repeat};
pub use repeat_with::RepeatWith;

mod cloned;
mod cropped;
mod grid;
mod index;
mod major;
mod utils;
mod with_msize;
mod with_size;

pub use self::grid::*;
pub use cloned::*;
pub use cropped::*;
pub use index::*;
pub use major::*;
pub use utils::*;
pub use with_size::*;

pub(crate) use with_msize::*;

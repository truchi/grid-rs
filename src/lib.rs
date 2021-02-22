// #![warn(missing_docs)]
// #![allow(unused)]

mod cloned;
mod cropped;
mod grid;
pub mod grid1d;
mod index;
mod major;
pub mod repeat;
pub mod repeat_with;
mod utils;
mod with_msize;
mod with_size;

pub use self::grid::*;
pub use cloned::*;
pub use cropped::*;
pub use grid1d::{ColGrid1D, Grid1D, RowGrid1D};
pub use index::*;
pub use major::*;
pub use repeat::{repeat, Repeat};
pub use repeat_with::RepeatWith;
pub use utils::*;
pub(crate) use with_msize::*;
pub use with_size::*;

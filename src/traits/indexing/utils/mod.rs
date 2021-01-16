mod major_2d;
mod major_slice_index;
mod slice_index;
mod to_index;
mod to_index_1d;
mod to_index_2d;

pub use major_2d::*;
pub use major_slice_index::*;
pub use slice_index::*;
pub use to_index::*;
pub use to_index_1d::*;
pub use to_index_2d::*;

use crate::{Grid1D, Grid2D, Major, Point, ToRange};
use std::{
    marker::PhantomData,
    ops::{Range, RangeBounds},
    slice::SliceIndex,
};

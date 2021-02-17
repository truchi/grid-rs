use crate::*;
use std::ops::Range;

// TODO
#[allow(dead_code)]
pub struct Crop<T> {
    rect: Size<Range<usize>>,
    grid: T,
}

impl<T> Crop<T> {
    pub fn new(rect: Size<Range<usize>>, grid: T) -> Self {
        // TODO size <= grid.size
        Self { rect, grid }
    }
}

impl<I> WithSize for Crop<I> {
    fn size(&self) -> Size {
        let Size { x, y } = self.rect.clone();

        Size {
            x: x.end - x.start,
            y: y.end - y.start,
        }
    }
}

impl<T: GridItem> GridItem for Crop<T> {
    type Item = T::Item;

    unsafe fn item_unchecked(self, _index: impl Index0D) -> Self::Item {
        todo!()
    }
}

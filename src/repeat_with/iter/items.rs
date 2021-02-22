use crate::*;
use std::ops::Range;

pub struct Items<F> {
    fun:    F,
    xstart: usize,
    xend:   usize,
    yend:   usize,
    x:      usize,
    y:      usize,
}

impl<F> Items<F> {
    pub fn new(fun: F, index: Rect) -> Self {
        let Point {
            x: Range {
                start: x,
                end: xend,
            },
            y: Range {
                start: y,
                end: yend,
            },
        } = index;

        Self {
            fun,
            xstart: x,
            xend,
            yend,
            x,
            y,
        }
    }
}

impl<I, F: FnMut(Point) -> I> Iterator for Items<F> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.x, self.y);

        let (x, y) = if x < self.xend {
            self.x += 1;
            (x, y)
        } else if y < self.yend {
            self.x = self.xstart;
            self.y += 1;
            (x, y)
        } else {
            return None;
        };

        Some((self.fun)(Point { x, y }))
    }
}

use crate::*;
use std::ops::Range;

pub fn index0d<M: Major>(point: Point, size: M) -> usize {
    let point = M::from(point);

    point.minor() * size.major() + point.major()
}

pub fn index1d<M: Major>(
    (i, Range { start, end }): (usize, Range<usize>),
    size: M,
) -> Range<usize> {
    let start = index0d(M::new(start, i).into(), size);

    start..start + end
}

pub fn major_index2d<M: Major>(index: Point<Range<usize>>) -> (Range<usize>, Range<usize>) {
    let start = M::from(Point {
        x: index.x.start,
        y: index.y.start,
    });
    let end = M::from(Point {
        x: index.x.end,
        y: index.y.end,
    });

    (start.major()..end.major(), start.minor()..end.minor())
}

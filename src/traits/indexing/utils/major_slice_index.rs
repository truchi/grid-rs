use super::*;

pub type MajorIndex<I, U> = MajorSliceIndex<I, U, usize, usize>;
pub type MajorSlice<I, U> = MajorSliceIndex<I, U, Range<usize>, usize>;

pub struct MajorSliceIndex<I, U, Maj, Min>
where
    Maj: SliceIndex<[I]>,
    Min: SliceIndex<[U], Output = U>,
{
    major:   Maj,
    minor:   Min,
    phantom: PhantomData<(I, U)>,
}

impl<I, U, Maj: Clone, Min: Clone> Clone for MajorSliceIndex<I, U, Maj, Min>
where
    Maj: SliceIndex<[I]>,
    Min: SliceIndex<[U], Output = U>,
{
    fn clone(&self) -> Self {
        Self {
            major:   self.major.clone(),
            minor:   self.minor.clone(),
            phantom: PhantomData,
        }
    }
}

impl<I, U, Maj, Min> MajorSliceIndex<I, U, Maj, Min>
where
    Maj: SliceIndex<[I]>,
    Min: SliceIndex<[U], Output = U>,
{
    pub fn new(major: Maj, minor: Min) -> Self {
        Self {
            major,
            minor,
            phantom: PhantomData,
        }
    }

    pub fn cast<T: Major<Maj, Min>>(other: T) -> Self {
        Self::new(other.major(), other.minor())
    }
}

impl<I, U, Maj: Clone, Min: Clone> Major2D<I, U> for MajorSliceIndex<I, U, Maj, Min>
where
    Maj: SliceIndex<[I]>,
    Min: SliceIndex<[U], Output = U>,
{
    type Major = Maj;
    type Minor = Min;

    fn major(self) -> Maj {
        self.major
    }

    fn minor(self) -> Min {
        self.minor
    }
}

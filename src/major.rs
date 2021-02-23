use crate::Coord;

/// Encodes memory layout in [`Size`](crate::Size)s types
/// ([`RowMajor`](RowMajor) / [`ColMajor`](ColMajor)).
///
/// A `Major` is a [`Size`](crate::Size) which knows its *major* and *minor*
/// axis.
pub trait Major: Copy + From<Coord> + Into<Coord> {
    /// Returns a new `Self` from the lengths of the major axis `major`
    /// and minor axis `minor`.
    fn new(major: usize, minor: usize) -> Self;

    /// Returns the length on the major axis.
    fn major(self) -> usize;

    /// Returns the length on the minor axis.
    fn minor(self) -> usize;
}

macro_rules! majors {
    ($(
        $(#[$meta:meta])* $Major:ident ($major:ident $minor:ident)
        $(#[$x_meta:meta])* $_x:ident
        $(#[$y_meta:meta])* $_y:ident
    )*) => { $(
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct $Major {
            $(#[$x_meta])*
            pub x: usize,
            $(#[$y_meta])*
            pub y: usize,
        }

        impl From<Coord> for $Major {
            fn from(Coord { x, y }: Coord) -> Self {
                Self { x, y }
            }
        }

        impl From<$Major> for Coord {
            fn from($Major { x, y }: $Major) -> Self {
                Self { x, y }
            }
        }

        impl Major for $Major {
            fn new(major: usize, minor: usize) -> Self {
                Self { $major: major, $minor: minor }
            }

            fn major(self) -> usize { self.$major }
            fn minor(self) -> usize { self.$minor }
        }
    )* };
}

majors!(
    /// A [`Size`](crate::Size) for column-major grids.
    ColMajor (y x)
        /// The size on the x axis.
        x
        /// The size on the y axis.
        y
    /// A [`Size`](crate::Size) for row-major grids.
    RowMajor (x y)
        /// The size on the x axis.
        x
        /// The size on the y axis.
        y
);

use std::cmp::Ordering;

macro_rules! coords {
    ($(
        $(#[$TMeta:meta])*
        $type:ident: $Type:ident ($as:ident $As:ident $a:ident: $A:ident $b:ident: $B:ident)
        $(#[$XMeta:meta])* $x:ident: $X:ident
        $(#[$YMeta:meta])* $y:ident: $Y:ident
    )*) => { $(
        $(#[$TMeta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct $Type<$X, $Y = $X> {
            $(#[$XMeta])* pub $x: $X,
            $(#[$YMeta])* pub $y: $Y,
        }

        /// ### Convertions
        impl<$X, $Y> $Type<$X, $Y> {
            /// Converts to other units.
            pub fn to<T, U>(self) -> $Type<T, U>
            where
                $X: Into<T>,
                $Y: Into<U>,
            {
                $Type { $x: self.$x.into(), $y: self.$y.into() }
            }

            doc!("Converts to [`" stringify!($As) "`](crate::" stringify!($As) ").",
            pub fn $as<$A, $B>(self) -> $As<$A, $B>
            where
                $X: Into<$A>,
                $Y: Into<$B>,
            {
                $As { $a: self.$x.into(), $b: self.$y.into() }
            });
        }

        impl<T: Clone> From<T> for $Type<T> {
            fn from(value: T) -> Self { Self { $x: value.clone(), $y: value } }
        }

        impl<$X, $Y> From<($X, $Y)> for $Type<$X, $Y> {
            fn from(($x, $y): ($X, $Y)) -> Self { Self { $x, $y } }
        }

        impl<$X, $Y> From<$Type<$X, $Y>> for ($X, $Y) {
            fn from($type: $Type<$X, $Y>) -> Self { ($type.$x, $type.$y) }
        }

        impl<$A, $B, $X: PartialEq<$A>, $Y: PartialEq<$B>> PartialEq<$As<$A, $B>> for $Type<$X, $Y> {
            fn eq(&self, other: &$As<$A, $B>) -> bool {
                self.$x == other.$a && self.$y == other.$b
            }
        }

        impl<$X: PartialOrd, $Y: PartialOrd> PartialOrd for $Type<$X, $Y> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match (self.$x.partial_cmp(&other.$x), self.$y.partial_cmp(&other.$y)) {
                    (Some(self_ord), Some(other_ord)) if self_ord == other_ord => Some(self_ord),
                    _ => None,
                }
            }
        }

        impl<$A, $B, $X: PartialOrd<$A>, $Y: PartialOrd<$B>> PartialOrd<$As<$A, $B>> for $Type<$X, $Y> {
            fn partial_cmp(&self, other: &$As<$A, $B>) -> Option<Ordering> {
                match (self.$x.partial_cmp(&other.$a), self.$y.partial_cmp(&other.$b)) {
                    (Some(self_ord), Some(other_ord)) if self_ord == other_ord => Some(self_ord),
                    _ => None,
                }
            }
        }
    )* };
}

coords!(
    /// A `x`, `y` [`Point`](crate::Point).
    point: Point (as_size Size width: W height: H)
        /// X axis `x` component.
        x: X
        /// Y axis `y` component.
        y: Y
    /// A `width`, `height` [`Size`](crate::Size).
    size: Size (as_point Point x: X y: Y)
        /// X axis `width` component.
        width: W
        /// Y axis `height` component.
        height: H
);

#[allow(unused)]
macro_rules! doc {
    ($($doc:expr)*, $item:item) => { doc!{ impl concat![" ", $($doc,)*], $item } };
    (impl $doc:expr, $item:item) => { #[doc = $doc] $item };
}

#[allow(unused)]
macro_rules! s { ($($tt:tt)*) => { stringify!($($tt)*) }; }
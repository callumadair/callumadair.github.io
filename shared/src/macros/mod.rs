#[macro_export]
macro_rules! clone {
    ($($identifier:ident),*) => {
        $(let $identifier = $identifier.clone();)*
    };
}

#[macro_export]
macro_rules! clone_mut {
    ($($identifier:ident),*) => {
        $(let mut $identifier = $identifier.clone();)*
    };
}

#[macro_export]
macro_rules! impl_nested_error {
    ($parent:ty, $child:ident, $($originator:ident),*) => {
        $(
            impl From<$originator> for $parent
            {
                fn from(value: $originator) -> Self { Self::$child($child::from(value)) }
            }
        )*
    };
}

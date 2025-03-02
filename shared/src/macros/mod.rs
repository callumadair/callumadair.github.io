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

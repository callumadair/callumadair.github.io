#[macro_export]
macro_rules! clone {
    ($($identifier:ident),*) => {
        $(let $identifier = $identifier.clone();)*
    };
}

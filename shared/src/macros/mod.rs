// TODO (CA): document all these macros
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

#[macro_export]
macro_rules! impl_nested_status_code {
    ($error_type:ty, $($variant:ident),+) => {
        impl actix_web::ResponseError for $error_type {
            fn status_code(&self) -> actix_web::http::StatusCode {
                match self {
                    $(
                        Self::$variant(inner) => inner.status_code(),
                    )+
                }
            }
        }
    };
}

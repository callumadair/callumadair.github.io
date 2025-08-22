#[macro_export]
macro_rules! impl_nested_status_code {
    ($error_type:ty, $($variant:ident),+) => {
        impl actix_web::ResponseError for $error_type {
            fn status_code(&self) -> StatusCode {
                match self {
                    $(
                        Self::$variant(inner) => inner.status_code(),
                    )+
                }
            }
        }
    };
}

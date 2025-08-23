pub mod error;
pub mod image;
pub mod software;

pub type Result<T> = core::result::Result<T, error::EntityError>;

#[macro_export]
macro_rules! impl_into_active_value {
    ($new_type:ty) => {
        impl IntoActiveValue<$new_type> for $new_type
        {
            fn into_active_value(self) -> sea_orm::ActiveValue<$new_type> { ActiveValue::Set(self) }
        }
    };
}

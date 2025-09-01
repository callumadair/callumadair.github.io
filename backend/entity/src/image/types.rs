use sea_orm::{
    ActiveValue,
    IntoActiveValue,
    entity::prelude::*,
};

use crate::{
    error::ImageInstantiationError,
    impl_into_active_value,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveValueType, derive_more::Display)]
pub struct ImageID(i32);
impl ImageID
{
    pub fn new(value: i32) -> Self { Self(value) }

    pub fn as_int(&self) -> i32 { self.0 }
}
impl_into_active_value!(ImageID);

#[derive(Clone, Debug, PartialEq, Eq, DeriveValueType, derive_more::Display)]
pub struct ImageURL(String);
impl_into_active_value!(ImageURL);

impl ImageURL
{
    pub fn new(raw: &str) -> crate::Result<Self>
    {
        let trimmed = raw.trim();
        if trimmed.is_empty()
        {
            Err(ImageInstantiationError::ImageURLEmpty.into())
        }
        else
        {
            Ok(Self(trimmed.to_string()))
        }
    }
}

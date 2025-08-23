use sea_orm::{
    ActiveValue,
    IntoActiveValue,
    entity::prelude::*,
};

use crate::{
    error::SoftwareInstantiationError,
    impl_into_active_value,
};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    DeriveValueType,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct SoftwareName(String);
impl_into_active_value!(SoftwareName);

impl SoftwareName
{
    pub fn new(raw: &str) -> crate::Result<Self>
    {
        let trimmed = raw.trim();
        if trimmed.is_empty()
        {
            Err(SoftwareInstantiationError::SoftwareNameEmpty.into())
        }
        else
        {
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    DeriveValueType,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct SoftwareShortDescription(String);
impl_into_active_value!(SoftwareShortDescription);

impl SoftwareShortDescription
{
    pub fn new(raw: &str) -> crate::Result<Self>
    {
        let trimmed = raw.trim();
        if trimmed.is_empty()
        {
            Err(SoftwareInstantiationError::SoftwareShortDescriptionEmpty.into())
        }
        else
        {
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    DeriveValueType,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct SoftwareLongDescription(String);
impl_into_active_value!(SoftwareLongDescription);

impl SoftwareLongDescription
{
    pub fn new(raw: &str) -> crate::Result<Self>
    {
        let trimmed = raw.trim();
        if trimmed.is_empty()
        {
            Err(SoftwareInstantiationError::SoftwareLongDescriptionEmpty.into())
        }
        else
        {
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    DeriveValueType,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct SoftwareWebLink(String);
impl_into_active_value!(SoftwareWebLink);

impl SoftwareWebLink
{
    pub fn new(raw: &str) -> crate::Result<Self>
    {
        let trimmed = raw.trim();
        if trimmed.is_empty()
        {
            Err(SoftwareInstantiationError::SoftwareWebLinkEmpty.into())
        }
        else
        {
            Ok(Self(trimmed.to_string()))
        }
    }
}

use bon::bon;
use sea_orm::{
    ActiveValue,
    IntoActiveValue,
    entity::prelude::*,
};

use crate::{
    error::InstantiationError,
    impl_into_active_value,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveValueType, derive_more::Display)]
pub struct SoftwareToolId(i32);
impl SoftwareToolId
{
    pub fn new(value: i32) -> Self { Self(value) }
}
impl_into_active_value!(SoftwareToolId);

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
            Err(crate::error::EntityError::InstantiationError(
                InstantiationError::ImageURLEmpty,
            ))
        }
        else
        {
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "images")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub id:               i32,
    #[sea_orm(foreign_key)]
    pub software_tool_id: SoftwareToolId,
    pub url:              ImageURL,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::software::Entity",
        from = "Column::SoftwareToolId",
        to = "super::software::Column::Id"
    )]
    SoftwareTool,
}

impl Related<super::software::Entity> for Entity
{
    fn to() -> RelationDef { Relation::SoftwareTool.def() }
}

impl ActiveModelBehavior for ActiveModel {}

#[bon]
impl ActiveModel
{
    #[builder]
    pub fn new(
        id: Option<i32>,
        software_tool_id: i32,
        url: &str,
    ) -> crate::Result<Self>
    {
        let id = match id
        {
            Some(value) => ActiveValue::Set(value),
            None => ActiveValue::NotSet,
        };

        Ok(Self {
            id,
            software_tool_id: SoftwareToolId::new(software_tool_id).into_active_value(),
            url: ImageURL::new(url)?.into_active_value(),
        })
    }
}

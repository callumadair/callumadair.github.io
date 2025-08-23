use bon::bon;
use sea_orm::{
    ActiveValue,
    IntoActiveValue,
    entity::prelude::*,
};

use crate::image::types::{
    ImageURL,
    SoftwareToolId,
};

pub mod types;

pub type ImageActiveModel = ActiveModel;
pub type ImageEntity = Entity;
pub type ImageModel = Model;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "images")]
pub struct Model
{
    #[sea_orm(primary_key)]
    id:               i32,
    #[sea_orm(foreign_key)]
    software_tool_id: SoftwareToolId,
    image_url:        ImageURL,
}

impl Model
{
    pub fn id(&self) -> &i32 { &self.id }

    pub fn software_tool_id(&self) -> &SoftwareToolId { &self.software_tool_id }

    pub fn image_url(&self) -> &ImageURL { &self.image_url }
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
        software_tool_id: SoftwareToolId,
        image_url: ImageURL,
    ) -> Self
    {
        let id = match id
        {
            Some(value) => ActiveValue::Set(value),
            None => ActiveValue::NotSet,
        };

        Self {
            id,
            software_tool_id: software_tool_id.into_active_value(),
            image_url: image_url.into_active_value(),
        }
    }
}

use sea_orm::entity::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "images")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub id:               i32,
    #[sea_orm(foreign_key)]
    pub software_tool_id: i32,
    pub path:             String,
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

use sea_orm::entity::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "software_tools")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub id:          i32,
    pub name:        String,
    pub short_desc:  String,
    pub long_desc:   String,
    pub web_link:    String,
    pub image_links: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

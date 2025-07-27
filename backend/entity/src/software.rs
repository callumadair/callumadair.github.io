use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    body::BoxBody,
    http::header::ContentType,
};
use sea_orm::{
    IntoActiveValue,
    entity::prelude::*,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, utoipa::ToSchema)]
#[sea_orm(table_name = "software_tools")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub id:         i32,
    pub name:       String,
    pub short_desc: String,
    pub long_desc:  String,
    pub web_link:   String,
}

impl Responder for Model
{
    type Body = BoxBody;

    fn respond_to(
        self,
        _req: &HttpRequest,
    ) -> HttpResponse<Self::Body>
    {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<shared::software::SoftwareTool> for ActiveModel
{
    fn from(value: shared::software::SoftwareTool) -> Self
    {
        Self {
            name: value.name.into_active_value(),
            short_desc: value.short_desc.into_active_value(),
            long_desc: value.long_desc.into_active_value(),
            web_link: value.web_link.into_active_value(),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::image::Entity")]
    Image,
}

impl Related<super::image::Entity> for Entity
{
    fn to() -> RelationDef { Relation::Image.def() }
}

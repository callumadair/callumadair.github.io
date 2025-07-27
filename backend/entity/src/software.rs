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

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Deserialize,
    Serialize,
    utoipa::ToSchema,
    utoipa::ToResponse,
)]
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

impl Model
{
    pub async fn to_software_tool(
        self,
        db_conn: &DatabaseConnection,
    ) -> Result<shared::software::SoftwareTool, DbErr>
    {
        let images = self.find_related(crate::image::Entity).all(db_conn).await?;
        let image_links = images
            .into_iter()
            .map(|image| image.path)
            .collect::<Vec<String>>();
        let mut software_tool = shared::software::SoftwareTool::from(self);
        software_tool.image_links = image_links;
        Ok(software_tool)
    }
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

impl From<Model> for shared::software::SoftwareTool
{
    fn from(value: Model) -> Self
    {
        Self {
            name:        value.name,
            short_desc:  value.short_desc,
            long_desc:   value.long_desc,
            web_link:    value.web_link,
            image_links: vec![],
        }
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

use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    body::BoxBody,
    http::header::ContentType,
};
use bon::bon;
use sea_orm::{
    ActiveValue,
    IntoActiveValue,
    entity::prelude::*,
};

use crate::{
    error::EntityError,
    software::types::{
        SoftwareLongDescription,
        SoftwareName,
        SoftwareShortDescription,
        SoftwareWebLink,
    },
};

pub mod types;

pub type SoftwareActiveModel = ActiveModel;
pub type SoftwareModel = Model;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    utoipa::ToResponse,
)]
#[sea_orm(table_name = "software_tools")]
pub struct Model
{
    #[sea_orm(primary_key)]
    id:         i32,
    name:       SoftwareName,
    short_desc: SoftwareShortDescription,
    long_desc:  SoftwareLongDescription,
    web_link:   SoftwareWebLink,
}

impl Model
{
    pub fn id(&self) -> &i32 { &self.id }

    pub fn name(&self) -> &SoftwareName { &self.name }

    pub fn short_desc(&self) -> &SoftwareShortDescription { &self.short_desc }

    pub fn long_desc(&self) -> &SoftwareLongDescription { &self.long_desc }

    pub fn web_link(&self) -> &SoftwareWebLink { &self.web_link }

    pub async fn to_software_tool(
        self,
        db_conn: &DatabaseConnection,
    ) -> Result<shared::software::SoftwareTool, DbErr>
    {
        let images = self.find_related(crate::image::Entity).all(db_conn).await?;
        let image_links = images
            .into_iter()
            .map(|image| image.image_url().to_string())
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
//
impl From<Model> for shared::software::SoftwareTool
{
    fn from(value: Model) -> Self
    {
        Self {
            name:        value.name.to_string(),
            short_desc:  value.short_desc.to_string(),
            long_desc:   value.long_desc.to_string(),
            web_link:    value.web_link.to_string(),
            image_links: vec![],
        }
    }
}

#[bon]
impl ActiveModel
{
    #[builder]
    pub fn new(
        id: Option<i32>,
        name: &str,
        short_desc: &str,
        long_desc: &str,
        web_link: &str,
    ) -> crate::Result<Self>
    {
        let id = match id
        {
            Some(value) => ActiveValue::Set(value),
            None => ActiveValue::NotSet,
        };
        Ok(Self {
            id,
            name: SoftwareName::new(name)?.into_active_value(),
            short_desc: SoftwareShortDescription::new(short_desc)?.into_active_value(),
            long_desc: SoftwareLongDescription::new(long_desc)?.into_active_value(),
            web_link: SoftwareWebLink::new(web_link)?.into_active_value(),
        })
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<shared::software::SoftwareTool> for ActiveModel
{
    type Error = EntityError;

    fn try_from(value: shared::software::SoftwareTool) -> crate::Result<Self>
    {
        Ok(Self {
            name: SoftwareName::new(&value.name)?.into_active_value(),
            short_desc: SoftwareShortDescription::new(&value.short_desc)?.into_active_value(),
            long_desc: SoftwareLongDescription::new(&value.long_desc)?.into_active_value(),
            web_link: SoftwareWebLink::new(&value.web_link)?.into_active_value(),
            ..Default::default()
        })
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::image::Entity")]
    Images,
}

impl Related<super::image::Entity> for Entity
{
    fn to() -> RelationDef { Relation::Images.def() }
}

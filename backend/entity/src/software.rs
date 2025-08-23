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
    error::{
        EntityError,
        InstantiationError,
    },
    image::ImageURL,
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
            Err(EntityError::InstantiationError(
                InstantiationError::SoftwareNameEmpty,
            ))
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
            Err(EntityError::InstantiationError(
                InstantiationError::SoftwareShortDescriptionEmpty,
            ))
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
            Err(EntityError::InstantiationError(
                InstantiationError::SoftwareLongDescriptionEmpty,
            ))
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
            Err(EntityError::InstantiationError(
                InstantiationError::SoftwareWebLinkEmpty,
            ))
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
    pub id:         i32,
    pub name:       SoftwareName,
    pub short_desc: SoftwareShortDescription,
    pub long_desc:  SoftwareLongDescription,
    pub web_link:   SoftwareWebLink,
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
            .map(|image| image.url)
            .collect::<Vec<ImageURL>>();
        let mut software_tool = shared::software::SoftwareTool::from(self);
        // software_tool.image_links = image_links;
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
            name:        value.name.0,
            short_desc:  value.short_desc.0,
            long_desc:   value.long_desc.0,
            web_link:    value.web_link.0,
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

impl From<shared::software::SoftwareTool> for ActiveModel
{
    fn from(value: shared::software::SoftwareTool) -> Self
    {
        Self {
            name: SoftwareName(value.name).into_active_value(),
            short_desc: SoftwareShortDescription(value.short_desc).into_active_value(),
            long_desc: SoftwareLongDescription(value.long_desc).into_active_value(),
            web_link: SoftwareWebLink(value.web_link).into_active_value(),
            ..Default::default()
        }
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

use std::borrow::Cow;

use actix_web::{
    HttpResponse,
    ResponseError,
    body::BoxBody,
    http::{
        StatusCode,
        header::ContentType,
    },
};
use entity::error::{
    EntityError,
    InstantiationError,
};
use shared::impl_nested_error;
use utoipa::openapi::{
    RefOr,
    Schema,
};

use crate::error::models::{
    CreateModelError,
    DomainModelError,
};

pub type Result<T> = core::result::Result<T, BackendError>;
pub mod models;

#[derive(thiserror::Error, Debug)]
pub enum BackendError
{
    #[error("{0}")]
    ActixSettings(#[from] actix_settings::Error),
    #[error("{0}")]
    ColorEyreReport(#[from] color_eyre::Report),
    #[error("{0}")]
    Database(#[from] sea_orm::error::DbErr),
    #[error("Domain model error: {0}")]
    DomainModelError(#[from] DomainModelError),
    #[error("Entity error: {0}")]
    EntityError(#[from] EntityError),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    PrometheusError(#[from] prometheus::Error),
}
impl_nested_error!(BackendError, DomainModelError, CreateModelError);
impl_nested_error!(BackendError, EntityError, InstantiationError);

impl ResponseError for BackendError
{
    fn status_code(&self) -> StatusCode
    {
        match self
        {
            Self::ActixSettings(_)
            | Self::ColorEyreReport(_)
            | Self::Database(_)
            | Self::IoError(_)
            | Self::PrometheusError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DomainModelError(model_error) => model_error.status_code(),
            Self::EntityError(entity_error) => entity_error.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody>
    {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            // TODO (CA): replace with call to serde json.
            .body(self.to_string())
    }
}

impl utoipa::ToSchema for BackendError
{
    fn name() -> Cow<'static, str> { Cow::Borrowed("Backend Error") }
}

impl utoipa::PartialSchema for BackendError
{
    fn schema() -> RefOr<Schema>
    {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                "message",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String),
            )
            .required("message")
            .examples(Some(serde_json::json! {"message: server bad"}))
            .into()
    }
}

// impl IntoDynNode for Base
// {
//     fn into_dyn_node(self) -> DynamicNode
//     {
//         let res = rsx! {
//         div {
//             class: "alert alert-error",
//             role: "alert",
//             CircleX {
//                 size: 20
//             },
//             {self.to_string()}}
//         };
//         res.into_dyn_node()
//     }
// }

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
use utoipa::openapi::{
    RefOr,
    Schema,
};

pub(crate) type Result<T> = core::result::Result<T, BackendError>;
#[derive(thiserror::Error, Debug)]
pub enum BackendError
{
    #[error("{0}")]
    Database(#[from] sea_orm::error::DbErr),
}

impl ResponseError for BackendError
{
    fn status_code(&self) -> StatusCode
    {
        match self
        {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody>
    {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
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

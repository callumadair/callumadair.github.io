use actix_web::{
    HttpResponse,
    HttpResponseBuilder,
    Responder,
    get,
    http::{
        StatusCode,
        header::CONTENT_TYPE,
    },
    web,
};
use entity::software::Entity as SoftwareTool;
use sea_orm::{
    entity::prelude::*,
    sea_query::all,
};

use crate::AppState;

#[utoipa::path(
    responses(
        (status = OK, body = Vec<entity::software::Model>),
        (status = INTERNAL_SERVER_ERROR, body = crate::error::BackendError),
    )
)]
#[get("/software")]
pub async fn software(state: web::Data<AppState>) -> crate::error::Result<impl Responder>
{
    let software_tools: Vec<entity::software::Model> =
        SoftwareTool::find().all(&state.db_conn).await?;

    Ok(HttpResponse::Ok().json(software_tools))
}

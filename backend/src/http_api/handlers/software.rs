use actix_web::{
    HttpResponse,
    HttpResponseBuilder,
    Responder,
    get,
    http::StatusCode,
    post,
    web,
};
use entity::software::Entity as SoftwareTool;
use sea_orm::entity::prelude::*;
use utoipa_actix_web::scope::Scope;

use crate::AppState;


#[utoipa::path(
    responses(
        (status = OK, body = Vec<entity::software::Model>),
        (status = INTERNAL_SERVER_ERROR, body = crate::error::BackendError),
    )
)]
#[get("")]
async fn index(state: web::Data<AppState>) -> crate::error::Result<impl Responder>
{
    let software_tools: Vec<entity::software::Model> =
        SoftwareTool::find().all(&state.db_conn).await?;

    Ok(HttpResponse::Ok().json(software_tools))
}

#[utoipa::path(
    responses(
        (status = OK, body = entity::software::Model),
        (status = INTERNAL_SERVER_ERROR, body = crate::error::BackendError),
    )
)]
#[post("/new")]
async fn create(
    web::Json(new_entry): web::Json<shared::software::SoftwareTool>,
    state: web::Data<AppState>,
) -> crate::error::Result<impl Responder>
{
    let res = HttpResponseBuilder::new(StatusCode::OK).await?;
    Ok(res)
}

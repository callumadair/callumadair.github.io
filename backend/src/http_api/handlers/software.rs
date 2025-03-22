use actix_web::{
    HttpResponse,
    Responder,
    get,
    web,
};
use entity::software::Entity as SoftwareTool;
use sea_orm::{
    entity::prelude::*,
    sea_query::all,
};

use crate::AppState;

#[utoipa::path(
    responses((status = 200, body = str ))
)]
#[get("/software")]
pub async fn software(state: web::Data<AppState>) -> crate::error::Result<impl Responder>
{
    let software_tools: Vec<entity::software::Model> =
        SoftwareTool::find().all(&state.db_conn).await?;

    Ok(software_tools)
}

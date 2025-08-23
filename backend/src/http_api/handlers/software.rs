use actix_web::{
    HttpResponse,
    Responder,
    get,
    post,
    web,
};

use crate::{
    AppState,
    repository::{
        connections::SeaOrmDataBaseConnection,
        traits::SoftwareRepository,
    },
};

#[utoipa::path(
    responses(
        (status = OK, body = Vec<shared::software::SoftwareTool>),
        (status = INTERNAL_SERVER_ERROR, body = crate::error::BackendError),
    )
)]
#[get("/index")]
async fn index(
    state: web::Data<AppState<SeaOrmDataBaseConnection>>
) -> crate::error::Result<impl Responder>
{
    let software_tools = state.repository.get_all_software_tools().await?;

    Ok(HttpResponse::Ok().json(software_tools))
}

#[utoipa::path(
    responses(
        (status = OK, body = shared::software::SoftwareTool),
        (status = INTERNAL_SERVER_ERROR, body = crate::error::BackendError),
    )
)]
#[post("/create")]
async fn create(
    web::Json(new_entry): web::Json<shared::software::SoftwareTool>,
    state: web::Data<AppState<SeaOrmDataBaseConnection>>,
) -> crate::error::Result<impl Responder>
{
    let new_entry = state
        .repository
        .create_software(&new_entry.try_into()?)
        .await?;

    Ok(new_entry)
}

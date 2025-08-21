use actix_web::{
    HttpResponse,
    Responder,
    get,
    post,
    web,
};
use entity::{
    image::ActiveModel as ImageActiveModel,
    software::{
        ActiveModel as SoftwareActiveModel,
        Entity as SoftwareTool,
    },
};
use sea_orm::{
    IntoActiveValue,
    entity::prelude::*,
};

use crate::AppState;

#[utoipa::path(
    responses(
        (status = OK, body = Vec<shared::software::SoftwareTool>),
        (status = INTERNAL_SERVER_ERROR, body = crate::error::BackendError),
    )
)]
#[get("/index")]
async fn index(state: web::Data<AppState>) -> crate::error::Result<impl Responder>
{
    let software_entries: Vec<entity::software::Model> =
        SoftwareTool::find().all(&state.db_conn).await?;
    let mut software_tools = Vec::with_capacity(software_entries.len());
    for entry in software_entries
    {
        software_tools.push(entry.to_software_tool(&state.db_conn).await?);
    }

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
    state: web::Data<AppState>,
) -> crate::error::Result<impl Responder>
{
    let active_model: SoftwareActiveModel = new_entry.clone().into();
    let software_insert_response = active_model.insert(&state.db_conn).await?;
    // Now that we have successfully created the software tool,
    // insert all the images information.
    // Prep all the images to be stored.
    for image_link in &new_entry.image_links
    {
        let mut image_active_model: ImageActiveModel = image_link.clone().into();
        image_active_model.software_tool_id = software_insert_response.id.into_active_value();
        image_active_model.insert(&state.db_conn).await?;
    }

    let success_response = software_insert_response
        .to_software_tool(&state.db_conn)
        .await?;

    Ok(success_response)
}

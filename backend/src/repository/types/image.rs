use entity::image::{
    ImageActiveModel,
    types::{
        ImageURL,
        SoftwareToolId,
    },
};
use sea_orm::ActiveValue;

use crate::error::Result;

#[derive(bon::Builder)]
pub struct CreateImageRequest
{
    #[builder(with = |id: i32| {SoftwareToolId::new(id)})]
    software_tool_id: SoftwareToolId,
    #[builder(with = |url: &str| -> Result<_> {Ok(ImageURL::new(url)?)})]
    image_url:        ImageURL,
}

impl CreateImageRequest
{
    pub fn software_tool_id(&self) -> &SoftwareToolId { &self.software_tool_id }

    pub fn image_url(&self) -> &ImageURL { &self.image_url }
}

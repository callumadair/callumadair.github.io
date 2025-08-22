use entity::image::{
    ImageURL,
    SoftwareToolId,
};

use crate::error::Result;

#[derive(bon::Builder)]
pub struct CreateImageRequest
{
    #[builder(with = |id: i32| -> Result<_> {Ok(SoftwareToolId::new(id))})]
    software_tool_id: SoftwareToolId,
    #[builder(with = |url: &str| -> Result<_> {Ok(ImageURL::new(url)?)})]
    image_url:        ImageURL,
}

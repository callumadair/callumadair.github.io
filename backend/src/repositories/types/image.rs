use entity::{
    image::types::{
        ImageID,
        ImageURL,
    },
    software::types::SoftwareID,
};

use crate::error::Result;

#[derive(bon::Builder)]
pub struct CreateImageRequest
{
    #[builder(with = |id: i32| {SoftwareID::new(id)})]
    software_id: SoftwareID,
    #[builder(with = |url: &str| -> Result<_> {Ok(ImageURL::new(url)?)})]
    image_url:   ImageURL,
}

impl CreateImageRequest
{
    pub fn software_id(&self) -> &SoftwareID { &self.software_id }

    pub fn image_url(&self) -> &ImageURL { &self.image_url }
}

#[derive(bon::Builder)]
pub struct DeleteImageRequest
{
    #[builder(with = |id: i32| {ImageID::new(id)})]
    image_id: ImageID,
}

impl DeleteImageRequest
{
    pub fn image_id(&self) -> &ImageID { &self.image_id }
}

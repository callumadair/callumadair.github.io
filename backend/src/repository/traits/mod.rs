use entity::{image::ImageModel, software::SoftwareModel};

use crate::{
    error::Result,
    repository::types::{
        image::CreateImageRequest,
        software::CreateSoftwareRequest,
    },
};
pub trait ImageRepository
{
    fn create_image(
        &self,
        req: &CreateImageRequest,
    ) -> Result<ImageModel>;
}

pub trait SoftwareRepository
{
    fn create_software(
        &self,
        req: &CreateSoftwareRequest,
    ) -> Result<SoftwareModel>;
}

use entity::{
    image::Model as ImageModel,
    software::Model as SoftwareModel,
};

use crate::{
    error::Result,
    repository::types::{
        image::CreateImageRequest,
        software::CreateSoftwareRequest,
    },
};
pub trait SoftwareRepository
{
    fn create_software(
        &self,
        req: &CreateSoftwareRequest,
    ) -> Result<SoftwareModel>;
}

pub trait ImageRepository
{
    fn create_image(
        &self,
        req: &CreateImageRequest,
    ) -> Result<ImageModel>;
}

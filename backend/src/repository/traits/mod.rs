use entity::{
    image::ImageModel,
    software::SoftwareModel,
};
use shared::software::SoftwareTool;

use crate::{
    error::Result,
    repository::types::{
        image::CreateImageRequest,
        software::CreateSoftwareRequest,
    },
};
pub trait ImageRepository: Clone + Send + Sync + 'static
{
    async fn create_image(
        &self,
        req: &CreateImageRequest,
    ) -> Result<ImageModel>;
}

pub trait SoftwareRepository: Clone + Send + Sync + 'static
{
    async fn create_software(
        &self,
        req: &CreateSoftwareRequest,
    ) -> Result<SoftwareModel>;

    async fn get_all_software(&self) -> Result<Vec<SoftwareModel>>;

    async fn get_all_software_tools(&self) -> Result<Vec<SoftwareTool>>;
}

use entity::{
    image::ImageModel,
    software::SoftwareModel,
};
use shared::software::SoftwareTool;

use crate::{
    error::Result,
    repositories::types::{
        image::CreateImageRequest,
        software::CreateSoftwareRequest,
    },
};

pub trait ImageRepository: Clone + Send + Sync + 'static
{
    fn create_image(
        &self,
        req: &CreateImageRequest,
    ) -> impl std::future::Future<Output = Result<ImageModel>> + Send;
}

pub trait SoftwareRepository: Clone + Send + Sync + 'static
{
    fn create_software(
        &self,
        req: &CreateSoftwareRequest,
    ) -> impl std::future::Future<Output = Result<SoftwareModel>> + Send;

    fn get_all_software(
        &self
    ) -> impl std::future::Future<Output = Result<Vec<SoftwareModel>>> + Send;

    fn get_all_software_tools(
        &self
    ) -> impl std::future::Future<Output = Result<Vec<SoftwareTool>>> + Send;
}

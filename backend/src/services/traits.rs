use std::future::Future;

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

pub trait ImageService: Clone + Send + Sync + 'static
{
    fn create_image(
        &self,
        req: &CreateImageRequest,
    ) -> impl Future<Output = Result<ImageModel>> + Send;
}

pub trait SoftwareService: Clone + Send + Sync + 'static
{
    fn create_software(
        &self,
        req: &CreateSoftwareRequest,
    ) -> impl Future<Output = Result<SoftwareModel>> + Send;

    fn get_all_software(&self) -> impl Future<Output = Result<Vec<SoftwareModel>>> + Send;

    fn get_all_software_tools(&self) -> impl Future<Output = Result<Vec<SoftwareTool>>> + Send;
}

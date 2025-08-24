use entity::{
    image::ImageModel,
    software::SoftwareModel,
};

use crate::{
    error::Result,
    metrics::traits::{
        ImageMetrics,
        SoftwareMetrics,
    },
    repositories::{
        traits::{
            ImageRepository,
            SoftwareRepository,
        },
        types::{
            image::CreateImageRequest,
            software::CreateSoftwareRequest,
        },
    },
    services::traits::{
        ImageService,
        SoftwareService,
    },
};

#[derive(Debug, Clone)]
pub struct Service<R, M>
where
    R: ImageRepository + SoftwareRepository,
    M: SoftwareMetrics,
{
    repository:     R,
    metrics_client: M,
}

impl<R, M> Service<R, M>
where
    R: ImageRepository + SoftwareRepository,
    M: ImageMetrics + SoftwareMetrics,
{
    pub fn new(
        repository: R,
        metrics_client: M,
    ) -> Self
    {
        Self {
            repository,
            metrics_client,
        }
    }
}

impl<R, M> ImageService for Service<R, M>
where
    R: ImageRepository + SoftwareRepository,
    M: ImageMetrics + SoftwareMetrics,
{
    async fn create_image(
        &self,
        req: &CreateImageRequest,
    ) -> Result<ImageModel>
    {
        let result = self.repository.create_image(req).await;
        if result.is_err()
        {
            self.metrics_client.record_image_creation_failure().await;
        }
        else
        {
            self.metrics_client.record_image_creation_success().await;
        }
        result
    }
}

impl<R, M> SoftwareService for Service<R, M>
where
    R: ImageRepository + SoftwareRepository,
    M: SoftwareMetrics,
{
    async fn create_software(
        &self,
        req: &CreateSoftwareRequest,
    ) -> Result<SoftwareModel>
    {
        let result = self.repository.create_software(req).await;
        if result.is_err()
        {
            self.metrics_client.record_software_creation_failure().await;
        }
        else
        {
            self.metrics_client.record_software_creation_success().await;
        }
        result
    }

    async fn get_all_software(&self) -> Result<Vec<SoftwareModel>>
    {
        let result = self.repository.get_all_software().await;
        if result.is_err()
        {
            self.metrics_client.record_get_all_software_failure().await;
        }
        else
        {
            self.metrics_client.record_get_all_software_success().await;
        }
        result
    }

    async fn get_all_software_tools(&self) -> Result<Vec<shared::software::SoftwareTool>>
    {
        let result = self.repository.get_all_software_tools().await;
        if result.is_err()
        {
            self.metrics_client.record_get_all_software_failure().await;
        }
        else
        {
            self.metrics_client.record_get_all_software_success().await;
        }
        result
    }
}

use entity::{
    image::ImageModel,
    software::SoftwareModel,
};

use crate::{
    error::Result,
    metrics::traits::SoftwareMetrics,
    repositories::{
        traits::SoftwareRepository,
        types::{
            image::CreateImageRequest,
            software::CreateSoftwareRequest,
        },
    },
    services::traits::SoftwareService,
};

#[derive(Debug, Clone)]
pub struct Service<R, M>
where
    R: SoftwareRepository,
    M: SoftwareMetrics,
{
    repository:     R,
    metrics_client: M,
}

impl<R, M> Service<R, M>
where
    R: SoftwareRepository,
    M: SoftwareMetrics,
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

impl<R, M> SoftwareService for Service<R, M>
where
    R: SoftwareRepository,
    M: SoftwareMetrics,
{
    async fn create_image(
        &self,
        req: &CreateImageRequest,
    ) -> Result<ImageModel>
    {
        let result = self.repository.create_image(req).await;
        if result.is_err()
        {
            self.metrics_client.record_image_creation_failure().await?;
        }
        else
        {
            self.metrics_client.record_image_creation_success().await?;
        }
        result
    }

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

    async fn delete_image(
        &self,
        req: &crate::repositories::types::image::DeleteImageRequest,
    ) -> Result<()>
    {
        let result = self.repository.delete_image(req).await;
        if result.is_err()
        {
            // TODO implement trait methods for metrics
            // here. self.metrics_client.
        }
        else
        {
            // TODO implement trait methods for metrics
            // here. self.
        }
        result
    }

    async fn delete_software(
        &self,
        req: &crate::repositories::types::software::DeleteSoftwareRequest,
    ) -> Result<()>
    {
        let result = self.repository.delete_software(req).await;
        if result.is_err()
        {
            // TODO implement trait methods for metrics
            // here. self.metrics_client.
        }
        else
        {
            // TODO implement trait methods for metrics
            // here. self.
        }
        result
    }
}

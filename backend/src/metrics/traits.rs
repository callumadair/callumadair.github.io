use crate::error::Result;

pub trait SoftwareMetrics: Clone + Send + Sync + 'static
{
    fn record_software_creation_failure(&self) -> impl Future<Output = ()> + Send;

    fn record_software_creation_success(&self) -> impl Future<Output = ()> + Send;

    fn record_get_all_software_success(&self) -> impl Future<Output = ()> + Send;

    fn record_get_all_software_failure(&self) -> impl Future<Output = ()> + Send;
}

pub trait ImageMetrics: Clone + Send + Sync + 'static
{
    fn record_image_creation_failure(&self) -> impl Future<Output = Result<()>> + Send;

    fn record_image_creation_success(&self) -> impl Future<Output = Result<()>> + Send;
}

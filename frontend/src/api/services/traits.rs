use shared::software::SoftwareTool;

pub trait ApiService: Clone + Send + Sync + 'static
{
    fn get_software_index(
        &self
    ) -> impl Future<Output = crate::error::types::Result<Vec<SoftwareTool>>> + Send;
}

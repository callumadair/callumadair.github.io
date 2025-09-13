use shared::software::SoftwareTool;

use crate::api::clients::traits::ApiUrl;

pub trait ApiService
{
    fn get_software_index(
        &self
    ) -> impl Future<Output = crate::error::types::Result<Vec<SoftwareTool>>>;
}

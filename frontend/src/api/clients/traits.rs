use std::collections::HashMap;

use shared::software::SoftwareTool;

use crate::error::types::{
    FrontendError,
    Result,
};

#[derive(Clone, Debug)]
pub struct ApiUrlMap
{
    inner: HashMap<ApiUrl, String>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, strum::Display)]
pub enum ApiUrl
{
    SoftwareIndex,
}

impl ApiUrlMap
{
    pub fn new(inner: HashMap<ApiUrl, String>) -> Self { Self { inner } }

    pub fn get_url(
        &self,
        url: &ApiUrl,
    ) -> crate::error::types::Result<&String>
    {
        self.inner
            .get(url)
            .ok_or(FrontendError::HashMapValueMissing(format!(
                "No value found for url: {url}."
            )))
    }
}

pub trait ApiClient: Clone + Send + Sync + 'static
{
    fn get_software_index(&self) -> impl Future<Output = Result<Vec<SoftwareTool>>> + Send;
}

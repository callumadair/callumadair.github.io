use shared::software::SoftwareTool;

use crate::{
    api::clients::traits::{
        ApiClient,
        ApiUrl,
        ApiUrlMap,
    },
    error::types::Result,
};

#[derive(Clone, Debug)]
pub struct ReqwestClient
{
    client:    reqwest::Client,
    url_paths: ApiUrlMap,
}

impl ReqwestClient
{
    pub fn new(
        client: reqwest::Client,
        url_paths: ApiUrlMap,
    ) -> Self
    {
        Self { client, url_paths }
    }
}

impl ApiClient for ReqwestClient
{
    async fn get_software_index(&self) -> Result<Vec<shared::software::SoftwareTool>>
    {
        let res = self
            .client
            .get(self.url_paths.get_url(&ApiUrl::SoftwareIndex)?)
            .send()
            .await?;
        Ok(res.json::<Vec<SoftwareTool>>().await?)
    }
}

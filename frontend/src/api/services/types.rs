use crate::api::{
    clients::traits::ApiClient,
    services::traits::ApiService,
};

#[derive(Clone)]
pub struct Service<C>
where
    C: ApiClient,
{
    api_client: C,
}

impl<C> Service<C>
where
    C: ApiClient,
{
    pub fn new(api_client: C) -> Self { Self { api_client } }
}

impl<C> ApiService for Service<C>
where
    C: ApiClient,
{
    async fn get_software_index(
        &self
    ) -> crate::error::types::Result<Vec<shared::software::SoftwareTool>>
    {
        self.api_client.get_software_index().await
    }
}

use crate::api::clients::traits::ApiClient;

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
    fn new(api_client: C) -> Self { Self { api_client } }
}

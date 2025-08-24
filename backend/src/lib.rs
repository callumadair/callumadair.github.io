pub mod metrics;
pub mod services;
use std::sync::Arc;

use crate::{
    metrics::types::Prometheus,
    repositories::connections::SeaOrmDataBaseConnection,
    services::{
        traits::{
            ImageService,
            SoftwareService,
        },
        types::Service,
    },
};

pub mod database;
pub mod error;
pub mod http_api;
pub mod repositories;

/// The AppState type that I currently use, partly because
/// the actix-web route macros do not like the trait
/// generics.
pub type DefaultAppState = AppState<Service<SeaOrmDataBaseConnection, Prometheus>>;

/// Exists for GET requests to query current app state.
#[derive(Debug, Clone)]
pub struct AppState<S: ImageService + SoftwareService>
{
    service: Arc<S>,
}

impl<S> AppState<S>
where
    S: ImageService + SoftwareService,
{
    pub fn new(service: S) -> Self
    {
        let service = Arc::new(service);
        Self { service }
    }

    pub fn service(&self) -> &Arc<S> { &self.service }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::{
        error::Result,
        metrics::types::CounterName,
    };

    // TODO (CA): replace with real test at some point.
    // A dummy test to enable CI jobs to run happily.
    #[tokio::test]
    async fn test_new() -> Result<()>
    {
        let repository =
            SeaOrmDataBaseConnection::new("postgres://postgres:password@localhost:5432").await?;
        let software_opts = prometheus::Opts::new(
            "software_creation_failure",
            "Number of attempts to create a software entry that have failed.",
        );
        let prometheus_client = Prometheus::builder()
            .counter_opt(CounterName::SoftwareCreationFailure, software_opts)
            .build()?;
        let service = Service::new(repository, prometheus_client);
        let _app_state = AppState::new(service);

        Ok(())
    }
}

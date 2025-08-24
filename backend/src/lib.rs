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

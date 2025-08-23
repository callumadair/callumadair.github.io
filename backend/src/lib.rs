use std::sync::Arc;

use crate::repository::traits::{
    ImageRepository,
    SoftwareRepository,
};

pub mod database;
pub mod error;
pub mod http_api;
pub mod repository;

/// Exists for GET requests to query current app state.
#[derive(Debug, Clone)]
pub struct AppState<R: ImageRepository + SoftwareRepository>
{
    pub repository: Arc<R>,
}

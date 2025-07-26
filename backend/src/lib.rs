use sea_orm::DatabaseConnection;

pub mod database;
pub mod error;
pub mod http_api;

/// Exists for GET requests to query current app state.
#[derive(Debug, Clone)]
pub struct AppState
{
    pub db_conn: DatabaseConnection,
}

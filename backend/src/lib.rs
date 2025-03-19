use actix_settings::{
    BasicSettings,
    NoSettings,
};
use sea_orm::DatabaseConnection;

pub mod database;
pub mod error;
pub mod http_api;

#[derive(Debug, Clone)]
pub struct AppState
{
    pub db_conn: DatabaseConnection,
}

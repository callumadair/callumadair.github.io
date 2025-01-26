use sea_orm::DatabaseConnection;

pub mod index;

#[derive(Debug, Clone)]
struct AppState
{
    conn: DatabaseConnection,
}

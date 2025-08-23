use entity::error::EntityError;
use sea_orm_migration::DbErr;

#[derive(Debug, thiserror::Error)]
pub enum MigrationError
{
    #[error("Error caused by database: {0}")]
    DbErr(#[from] DbErr),
    #[error("Error caused by entity: {0}")]
    EntityError(#[from] EntityError),
}

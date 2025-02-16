use sea_orm_migration::{
    prelude::*,
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr>
    {
        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .if_not_exists()
                    .col(pk_auto(Image::Id))
                    .col(string(Image::SoftwareToolId))
                    .col(string(Image::Path))
                    .to_owned(),
            )
            .await
    }

    async fn down(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(Image::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Image
{
    Table,
    Id,
    SoftwareToolId,
    Path,
}

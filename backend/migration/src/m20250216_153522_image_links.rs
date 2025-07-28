use sea_orm_migration::{
    prelude::*,
    schema::*,
};

use crate::{
    m20250216_151738_software_tools::SoftwareTools,
    m20250216_153522_image_links::Images::SoftwareToolId,
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
                    .table(Images::Table)
                    .if_not_exists()
                    .col(pk_auto(Images::Id))
                    .col(integer(SoftwareToolId))
                    .col(string(Images::Path))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("software-tool-id")
                            .from(Images::Table, Images::SoftwareToolId)
                            .to(SoftwareTools::Table, SoftwareTools::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
            .drop_table(Table::drop().table(Images::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Images
{
    Table,
    Id,
    SoftwareToolId,
    Path,
}

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
                    .table(SoftwareTools::Table)
                    .if_not_exists()
                    .col(pk_auto(SoftwareTools::Id))
                    .col(string(SoftwareTools::Name))
                    .col(string(SoftwareTools::ShortDesc))
                    .col(string(SoftwareTools::LongDesc))
                    .col(string(SoftwareTools::WebLink))
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
            .drop_table(Table::drop().table(SoftwareTools::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum SoftwareTools
{
    Table,
    Id,
    Name,
    ShortDesc,
    LongDesc,
    WebLink,
}

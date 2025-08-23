use entity::software::types::{
    SoftwareLongDescription,
    SoftwareName,
    SoftwareShortDescription,
    SoftwareWebLink,
};
use sea_orm_migration::{
    prelude::*,
    schema::*,
};

use crate::{
    error::MigrationError,
    sea_orm::ActiveModelTrait,
};

#[derive(DeriveMigrationName)]
pub struct Migration;
impl Migration
{
    async fn seed_data(db_conn: &SchemaManagerConnection<'_>) -> Result<(), MigrationError>
    {
        entity::software::ActiveModel::builder()
            .id(1)
            .name(SoftwareName::new("Starship")?)
            .short_desc(SoftwareShortDescription::new(
                "A nice modern terminal prompt",
            )?)
            .long_desc(SoftwareLongDescription::new("Starship is neat")?)
            .web_link(SoftwareWebLink::new("https://starship.rs")?)
            .build()
            .insert(db_conn)
            .await?;

        entity::software::ActiveModel::builder()
            .id(2)
            .name(SoftwareName::new("Hyperfine")?)
            .short_desc(SoftwareShortDescription::new(
                "A benchmarking tool written in rust",
            )?)
            .long_desc(SoftwareLongDescription::new("Hyperfine is neat")?)
            .web_link(SoftwareWebLink::new(
                "https://github.com/sharkdp/hyperfine",
            )?)
            .build()
            .insert(db_conn)
            .await?;

        entity::software::ActiveModel::builder()
            .id(3)
            .name(SoftwareName::new("Nushell")?)
            .short_desc(SoftwareShortDescription::new("A new way of doing shells")?)
            .long_desc(SoftwareLongDescription::new("Nushell is neat")?)
            .web_link(SoftwareWebLink::new("https://www.nushell.sh")?)
            .build()
            .insert(db_conn)
            .await?;
        Ok(())
    }
}

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
            .await?;
        let db = manager.get_connection();
        Self::seed_data(db)
            .await
            .map_err(|migration_error| DbErr::Custom(migration_error.to_string()))?;

        Ok(())
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

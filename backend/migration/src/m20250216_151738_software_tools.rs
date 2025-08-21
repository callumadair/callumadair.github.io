use sea_orm_migration::{
    prelude::*,
    schema::*,
};

use crate::sea_orm::ActiveModelTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;
impl Migration
{
    async fn seed_data(db_conn: &SchemaManagerConnection<'_>) -> Result<(), DbErr>
    {
        entity::software::ActiveModel::builder()
            .id(1)
            .name("Starship")
            .short_desc("A nice modern terminal prompt")
            .long_desc("Starship is neat")
            .web_link("https://starship.rs")
            .build()
            .map_err(|entity_err| DbErr::Custom(entity_err.to_string()))?
            .insert(db_conn)
            .await?;

        entity::software::ActiveModel::builder()
            .id(2)
            .name("Hyperfine")
            .short_desc("A benchmarking tool written in rust")
            .long_desc("Hyperfine is neat")
            .web_link("https://github.com/sharkdp/hyperfine")
            .build()
            .map_err(|entity_err| DbErr::Custom(entity_err.to_string()))?
            .insert(db_conn)
            .await?;

        entity::software::ActiveModel::builder()
            .id(3)
            .name("Nushell")
            .short_desc("A new way of doing shells")
            .long_desc("Nushell is neat")
            .web_link("https://www.nushell.sh")
            .build()
            .map_err(|entity_err| DbErr::Custom(entity_err.to_string()))?
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
        Self::seed_data(db).await?;
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

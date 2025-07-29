use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::IntoActiveValue,
};

use crate::sea_orm::ActiveModelTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;
impl Migration
{
    async fn seed_data(db_conn: &SchemaManagerConnection<'_>) -> Result<(), DbErr>
    {
        entity::software::ActiveModel {
            id:         1.into_active_value(),
            name:       String::from("Starship").into_active_value(),
            short_desc: String::from("A nice modern terminal prompt").into_active_value(),
            long_desc:  String::from("Starship is neat").into_active_value(),
            web_link:   String::from("https://starship.rs").into_active_value(),
        }
        .insert(db_conn)
        .await?;

        entity::software::ActiveModel {
            id:         2.into_active_value(),
            name:       String::from("Hyperfine").into_active_value(),
            short_desc: String::from("A benchmarking tool written in rust").into_active_value(),
            long_desc:  String::from("Hyperfine is neat").into_active_value(),
            web_link:   String::from("https://github.com/sharkdp/hyperfine").into_active_value(),
        }
        .insert(db_conn)
        .await?;

        entity::software::ActiveModel {
            id:         3.into_active_value(),
            name:       String::from("Nushell").into_active_value(),
            short_desc: String::from("A new way of doing shells").into_active_value(),
            long_desc:  String::from("Nushell is neat").into_active_value(),
            web_link:   String::from("https://www.nushell.sh").into_active_value(),
        }
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
